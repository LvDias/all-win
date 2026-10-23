using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.Linq;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;
using Newtonsoft.Json.Linq;

namespace AllWin.FakeCore
{
    /// <summary>Result of one scenario run; results are asserted from the report/snapshot stream (no screen needed).</summary>
    public sealed class ScenarioResult
    {
        public string Name { get; set; } = "";
        public bool Passed { get; set; }
        public List<string> Steps { get; } = new List<string>();
        public string? Failure { get; set; }
    }

    /// <summary>
    /// Executes a scenario file against a <see cref="FakeCoreServer"/>. Steps marked as simulator steps need
    /// <see cref="SimAdapterHost"/> (Linux); the others also run against NinjaTrader on the Windows machine.
    /// The format is documented in plugins/ninjatrader8/tools/README.md.
    /// </summary>
    public sealed class ScenarioRunner
    {
        private readonly JObject _scenario;
        private readonly FakeCoreServer _core;
        private readonly SimAdapterHost? _sim;
        private readonly Action<string> _trace;
        private readonly Dictionary<string, string> _commandIds = new Dictionary<string, string>(StringComparer.Ordinal);
        private readonly Dictionary<string, CommandPayload> _commands = new Dictionary<string, CommandPayload>(StringComparer.Ordinal);
        private readonly Dictionary<string, string> _msgIds = new Dictionary<string, string>(StringComparer.Ordinal);
        private readonly string _instructionId = Ids.New();

        public ScenarioRunner(JObject scenario, FakeCoreServer core, SimAdapterHost? sim, Action<string>? trace = null)
        {
            _scenario = scenario;
            _core = core;
            _sim = sim;
            _trace = trace ?? (_ => { });
        }

        public static IReadOnlyList<FakeAccount> AccountsOf(JObject scenario) =>
            ((JArray?)scenario["accounts"] ?? new JArray()).Select(a => new FakeAccount
            {
                Alias = (string)a["alias"]!,
                PlatformAccount = (string)a["platform_account"]!,
                AccountId = StableId((string)a["platform_account"]!),
                Roles = a["roles"] is JArray roles ? roles.Select(r => (string)r! == "master" ? AccountRole.Master : AccountRole.Destination).ToList() : new List<AccountRole> { AccountRole.Destination },
            }).ToList();

        /// <summary>Same account_id for the same platform account in every run, so a real plugin keeps its mapping.</summary>
        private static string StableId(string platformAccount)
        {
            var hash = System.Security.Cryptography.SHA256.HashData(System.Text.Encoding.UTF8.GetBytes("allwin-fakecore:" + platformAccount));
            hash[6] = (byte)((hash[6] & 0x0F) | 0x80);
            hash[8] = (byte)((hash[8] & 0x3F) | 0x80);
            var hex = Convert.ToHexString(hash, 0, 16).ToLowerInvariant();
            return hex.Substring(0, 8) + "-" + hex.Substring(8, 4) + "-" + hex.Substring(12, 4) + "-" + hex.Substring(16, 4) + "-" + hex.Substring(20, 12);
        }

        public async Task<ScenarioResult> RunAsync()
        {
            var result = new ScenarioResult { Name = (string?)_scenario["name"] ?? "scenario" };
            var steps = (JArray)_scenario["steps"]!;
            for (var i = 0; i < steps.Count; i++)
            {
                var step = (JObject)steps[i];
                var label = (i + 1).ToString(CultureInfo.InvariantCulture) + ". " + (string)step["do"]! + Describe(step);
                try
                {
                    await RunStepAsync(step).ConfigureAwait(false);
                    result.Steps.Add("ok   " + label);
                    _trace("step ok: " + label);
                }
                catch (ScenarioFailure e)
                {
                    result.Steps.Add("FAIL " + label + ": " + e.Message);
                    result.Failure = label + ": " + e.Message;
                    return result;
                }
            }
            var violations = _core.Violations;
            if (violations.Count > 0)
            {
                result.Failure = "protocol violations: " + string.Join("; ", violations);
                return result;
            }
            result.Passed = true;
            return result;
        }

        private async Task RunStepAsync(JObject step)
        {
            switch ((string)step["do"]!)
            {
                case "wait_ready":
                    await WaitAsync(() => _core.SessionReady, Timeout(step, 20000), "adapter session ready (reconnect snapshot)").ConfigureAwait(false);
                    break;
                case "expect_session":
                    var count = (int)step["count"]!;
                    await WaitAsync(() => _core.SessionCount >= count && _core.SessionReady, Timeout(step, 30000), "session #" + count + " ready").ConfigureAwait(false);
                    break;
                case "command":
                    await SendCommandAsync(step).ConfigureAwait(false);
                    break;
                case "repeat_command":
                    // Same command_id with a new msg_id: a business duplicate.
                    await _core.SendReliableAsync(_commands[(string)step["id"]!]).ConfigureAwait(false);
                    break;
                case "resend":
                    // Same msg_id and bytes: a transport duplicate.
                    await _core.ResendAsync(_msgIds[(string)step["msg"]!]).ConfigureAwait(false);
                    break;
                case "expect_report":
                    await ExpectReportAsync(step).ConfigureAwait(false);
                    break;
                case "expect_no_report":
                    await Task.Delay(Timeout(step, 1500, "within_ms")).ConfigureAwait(false);
                    var unexpected = MatchingReports(step).Count;
                    if (unexpected > 0) throw new ScenarioFailure(unexpected + " unexpected matching report(s)");
                    break;
                case "expect_snapshot":
                    await WaitAsync(() => MatchingSnapshot(step), Timeout(step, 15000), "snapshot " + step.ToString(Newtonsoft.Json.Formatting.None)).ConfigureAwait(false);
                    break;
                case "expect_ack":
                    var acked = _msgIds[(string)step["msg"]!];
                    await WaitAsync(() => _core.Received.Any(r => r.Envelope.Type == MessageTypes.Ack && r.Envelope.PayloadAs<AckPayload>().MsgIds.Contains(acked)),
                        Timeout(step, 10000), "ack of " + (string)step["msg"]!).ConfigureAwait(false);
                    break;
                case "limits":
                    await SendLimitsAsync(step).ConfigureAwait(false);
                    break;
                case "close":
                    await _core.CloseAsync((int?)step["code"] ?? 4000).ConfigureAwait(false);
                    break;
                case "drop_acks":
                    _core.DropAcks = (bool)step["value"]!;
                    break;
                case "sleep":
                    await Task.Delay((int)step["ms"]!).ConfigureAwait(false);
                    break;
                case "sim":
                    if (_sim == null) throw new ScenarioFailure("simulator step: run with -with-sim (Linux) or use an NT8-compatible scenario");
                    await _sim.ApplyAsync(step, AliasToPlatformAccount).ConfigureAwait(false);
                    break;
                default:
                    throw new ScenarioFailure("unknown step " + step["do"]);
            }
        }

        private string AliasToPlatformAccount(string alias) => _core.Accounts.Single(a => a.Alias == alias).PlatformAccount;

        private async Task SendCommandAsync(JObject step)
        {
            await WaitAsync(() => _core.SessionReady, 20000, "session ready before commands (ADR-0004 §3 step 3)").ConfigureAwait(false);
            var alias = (string)step["id"]!;
            var account = _core.Accounts.Single(a => a.Alias == ((string?)step["account"] ?? _core.Accounts[0].Alias));
            var commandId = Ids.New();
            _commandIds[alias] = commandId;

            var json = new JObject
            {
                ["command_id"] = commandId,
                ["instruction_id"] = _instructionId,
                ["account_id"] = account.AccountId,
                ["platform_account"] = account.PlatformAccount,
                ["instrument"] = step["instrument"]?.DeepClone() ?? _scenario["instrument"]?.DeepClone() ?? new JObject { ["symbol"] = "MNQ", ["contract_month"] = "2026-12" },
                ["kind"] = step["kind"]!.DeepClone(),
            };
            foreach (var key in new[] { "place", "modify", "cancel", "flatten" })
                if (step[key] != null) json[key] = ResolveAliases(step[key]!.DeepClone());
            if (step["depends_on"] != null) json["depends_on"] = _commandIds[(string)step["depends_on"]!];
            // Protocol 1.2 position-identity guard: the alias of the command that opened the position.
            if (step["position_ref"] != null) json["refs"] = new JObject { ["position_ref"] = _commandIds[(string)step["position_ref"]!] };
            var isEntry = (string)json["kind"]! == "place" && (string?)json["place"]?["position_effect"] == "open";
            if (isEntry || step["ttl_ms"] != null)
                json["expires_at"] = UtcTimestamp.Format(DateTime.UtcNow.AddMilliseconds((double?)step["ttl_ms"] ?? 5000));

            var command = ProtocolCodec.ToModel<CommandPayload>(json);
            _commands[alias] = command;
            var sentAt = DateTime.UtcNow.AddMilliseconds((double?)step["sent_at_offset_ms"] ?? 0);
            var msgId = await _core.SendReliableAsync(command, sentAt: sentAt).ConfigureAwait(false);
            if (step["msg"] != null) _msgIds[(string)step["msg"]!] = msgId;
        }

        private JToken ResolveAliases(JToken spec)
        {
            if (spec["targets"] is JArray targets)
                foreach (var target in targets.OfType<JObject>())
                    if (target["command"] != null)
                    {
                        target["command_id"] = _commandIds[(string)target["command"]!];
                        target.Remove("command");
                    }
            return spec;
        }

        private async Task SendLimitsAsync(JObject step)
        {
            var account = _core.Accounts.Single(a => a.Alias == ((string?)step["account"] ?? _core.Accounts[0].Alias));
            var limits = new AccountLimits
            {
                AccountId = account.AccountId, PlatformAccount = account.PlatformAccount, Version = (long?)step["version"] ?? 2,
                EnforcementEnabled = (bool?)step["enforcement_enabled"] ?? true, MaxPosition = (int?)step["max_position"] ?? 10,
                TradingDay = new TradingDaySpec { ResetLocalTime = "17:00", TimeZone = "America/Chicago" }, WarningThresholdPct = 80,
            };
            if (step["blocked_for_ms"] != null) limits.BlockedUntil = DateTime.UtcNow.AddMilliseconds((double)step["blocked_for_ms"]!);
            var msgId = await _core.SendReliableAsync(new LimitsPayload { Accounts = { limits } }).ConfigureAwait(false);
            if (step["msg"] != null) _msgIds[(string)step["msg"]!] = msgId;
        }

        private async Task ExpectReportAsync(JObject step)
        {
            var count = (int?)step["count"] ?? 1;
            await WaitAsync(() => MatchingReports(step).Count >= count, Timeout(step, 15000), "report " + Describe(step)).ConfigureAwait(false);
            if ((bool?)step["exact"] == true)
            {
                await Task.Delay(500).ConfigureAwait(false);
                var actual = MatchingReports(step).Count;
                if (actual != count) throw new ScenarioFailure("expected exactly " + count + " matching report(s), got " + actual);
            }
        }

        private List<JObject> MatchingReports(JObject step)
        {
            string? commandId = null;
            if (step["command"] != null && !_commandIds.TryGetValue((string)step["command"]!, out commandId)) return new List<JObject>();
            var match = step["match"] as JObject ?? new JObject();
            var external = (bool?)step["external"] ?? false;
            return _core.Received.Where(r => !r.Duplicate && r.Envelope.Type == MessageTypes.ExecutionReport)
                .Select(r => r.Envelope.Payload)
                .Where(p => commandId == null ? (!external || p["command_id"] == null) : (string?)p["command_id"] == commandId)
                .Where(p => Matches(p, match)).ToList();
        }

        private bool MatchingSnapshot(JObject step)
        {
            var trigger = (string?)step["trigger"];
            var currentSession = _core.SessionCount;
            foreach (var received in _core.Received.Where(r => !r.Duplicate && r.Envelope.Type == MessageTypes.PositionSnapshot).Reverse())
            {
                if (((string?)step["session"] ?? "current") == "current" && received.Session != currentSession) continue;
                var payload = received.Envelope.Payload;
                if (trigger != null && (string?)payload["trigger"] != trigger) continue;
                if (step["positions"] is JObject positions && !PositionsMatch(payload, positions)) continue;
                if (step["connected"] is JObject connected && !connected.Properties().All(c =>
                        payload["accounts"]!.Any(a => (string?)a["platform_account"] == AliasToPlatformAccount(c.Name) && (bool)a["connected"]! == (bool)c.Value))) continue;
                if (step["working_orders"] is JObject working && !working.Properties().All(w =>
                        payload["accounts"]!.Any(a => (string?)a["platform_account"] == AliasToPlatformAccount(w.Name) && ((JArray)a["working_orders"]!).Count == (int)w.Value))) continue;
                return true;
            }
            return false;
        }

        private bool PositionsMatch(JObject snapshot, JObject expected)
        {
            foreach (var account in expected.Properties())
            {
                var actual = snapshot["accounts"]!.FirstOrDefault(a => (string?)a["platform_account"] == AliasToPlatformAccount(account.Name));
                if (actual == null) return false;
                foreach (var symbol in ((JObject)account.Value).Properties())
                {
                    var quantity = actual["positions"]!.Where(p => (string?)p["instrument"]!["symbol"] == symbol.Name).Sum(p => (int)p["position"]!["quantity"]!);
                    if (quantity != (int)symbol.Value) return false;
                }
            }
            return true;
        }

        /// <summary>Subset match: every expected property must be present and equal (strings compared exactly).</summary>
        internal static bool Matches(JToken actual, JToken expected)
        {
            if (expected is JObject eo)
                return actual is JObject ao && eo.Properties().All(p => ao[p.Name] != null && Matches(ao[p.Name]!, p.Value));
            if (expected.Type == JTokenType.Null) return actual.Type == JTokenType.Null;
            if (expected is JArray ea)
                return actual is JArray aa && aa.Count == ea.Count && ea.Select((e, i) => Matches(aa[i], e)).All(x => x);
            return actual is JValue av && expected is JValue ev && string.Equals(Convert.ToString(av.Value, CultureInfo.InvariantCulture), Convert.ToString(ev.Value, CultureInfo.InvariantCulture), StringComparison.Ordinal);
        }

        private static async Task WaitAsync(Func<bool> condition, int timeoutMs, string what)
        {
            var watch = Stopwatch.StartNew();
            while (!condition())
            {
                if (watch.ElapsedMilliseconds > timeoutMs) throw new ScenarioFailure("timed out after " + timeoutMs + " ms waiting for " + what);
                await Task.Delay(25).ConfigureAwait(false);
            }
        }

        private static int Timeout(JObject step, int fallback, string key = "timeout_ms") => (int?)step[key] ?? fallback;

        private static string Describe(JObject step)
        {
            var parts = new List<string>();
            foreach (var key in new[] { "id", "command", "kind", "trigger", "msg" })
                if (step[key] != null) parts.Add(key + "=" + step[key]);
            if (step["match"] != null) parts.Add(step["match"]!.ToString(Newtonsoft.Json.Formatting.None));
            return parts.Count == 0 ? "" : " (" + string.Join(" ", parts) + ")";
        }
    }

    public sealed class ScenarioFailure : Exception
    {
        public ScenarioFailure(string message) : base(message) { }
    }
}
