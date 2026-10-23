using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using AllWin.Adapter.Core.Protocol;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

// ADR-0005 N11 (U15): the local "Allow live accounts" opt-in and the live-account decision table (U19, U19.1).
namespace AllWin.Adapter.Core.Engine
{
    /// <summary>Rows of the ADR-0005 §N11 decision table. Every platform write on a live account is classified by row.</summary>
    public enum LiveRow
    {
        /// <summary>Submit an entry. Also used for changing a working entry or exit order (it could add exposure).</summary>
        Entry = 1,

        /// <summary>Cancel a working All Win entry. Also used for a working All Win exit (undoing All Win's own order).</summary>
        CancelEntry = 2,
        CancelOrphanLeg = 3,
        ShrinkLeg = 4,
        CancelWrongSideLeg = 5,
        InitialBracket = 6,
        UpsizeLeg = 7,
        RepriceLeg = 8,
        Flatten = 9,
        Reduce = 10,
        CancelProtectingLeg = 11,
    }

    /// <summary>States of a live (or kind-unknown) account in one instrument (ADR-0005 §N11).</summary>
    public enum LiveState
    {
        /// <summary>Confirmed, opt-in on, welcome.simulation_only false: normal rules.</summary>
        B,

        /// <summary>Not confirmed, confirmation invalidated by a kind change, or kind unknown.</summary>
        A,

        /// <summary>Confirmed, after "off" or simulation_only true; position flat.</summary>
        C0,

        /// <summary>Like C0, non-flat with an All Win identity.</summary>
        C1,

        /// <summary>Like C0, non-flat with an unknown or external identity.</summary>
        C2,
    }

    /// <summary>U19: what a confirmed account may still do after "off". Build/server configuration, never a UI toggle.</summary>
    public enum LiveOffPolicy
    {
        /// <summary>U19 (a), recommended: entries are cancelled at once; exits, flatten and protection follow the C columns.</summary>
        CancelEntriesKeepExits,

        /// <summary>U19 (b): a confirmed account after "off" behaves as state A.</summary>
        RefuseAll,
    }

    /// <summary>U19.1: row 11 after "off". Build/server configuration, never a UI toggle.</summary>
    public enum ProtectiveCancelPolicy
    {
        /// <summary>U19.1 (b), recommended: a Core cancel of a stop/target that still protects is refused (I2).</summary>
        Refuse,

        /// <summary>U19.1 (a): accepted in C1 and C2; still refused in A.</summary>
        AllowAfterOff,
    }

    /// <summary>The ADR-0005 §N11 table as a pure function. "—" cells (not applicable) are refusals: fail closed.</summary>
    public static class LiveAccountTable
    {
        public static bool Allows(LiveRow row, LiveState state, LiveOffPolicy offPolicy, ProtectiveCancelPolicy cancelPolicy)
        {
            if (state == LiveState.B) return true;
            var c = state == LiveState.C0 || state == LiveState.C1 || state == LiveState.C2;
            if (c && offPolicy == LiveOffPolicy.RefuseAll) state = LiveState.A;
            switch (row)
            {
                case LiveRow.Entry: return false;
                case LiveRow.CancelEntry: return true;
                case LiveRow.CancelOrphanLeg: return state == LiveState.A || state == LiveState.C0;
                case LiveRow.ShrinkLeg:
                case LiveRow.CancelWrongSideLeg:
                case LiveRow.InitialBracket: return state == LiveState.A || state == LiveState.C1 || state == LiveState.C2;
                case LiveRow.UpsizeLeg:
                case LiveRow.RepriceLeg: return state == LiveState.C1;
                case LiveRow.Flatten:
                case LiveRow.Reduce: return state == LiveState.C0 || state == LiveState.C1;
                case LiveRow.CancelProtectingLeg:
                    return cancelPolicy == ProtectiveCancelPolicy.AllowAfterOff && (state == LiveState.C1 || state == LiveState.C2);
                default: return false;
            }
        }

        /// <summary>"a" or "b"; anything else, missing or invalid, is the recommended option (a).</summary>
        public static LiveOffPolicy ParseOffPolicy(string? value) =>
            string.Equals(value?.Trim(), "b", StringComparison.OrdinalIgnoreCase) ? LiveOffPolicy.RefuseAll : LiveOffPolicy.CancelEntriesKeepExits;

        /// <summary>"a" or "b"; anything else, missing or invalid, is the recommended option (b).</summary>
        public static ProtectiveCancelPolicy ParseProtectiveCancelPolicy(string? value) =>
            string.Equals(value?.Trim(), "a", StringComparison.OrdinalIgnoreCase) ? ProtectiveCancelPolicy.AllowAfterOff : ProtectiveCancelPolicy.Refuse;
    }

    /// <summary>A live account confirmed in the dialog: (connection, provider, platform_account) (Vigia L1).</summary>
    public sealed class LiveAccountKey : IEquatable<LiveAccountKey>
    {
        public LiveAccountKey(string connection, string provider, string platformAccount)
        {
            Connection = connection;
            Provider = provider;
            PlatformAccount = platformAccount;
        }

        public string Connection { get; }
        public string Provider { get; }
        public string PlatformAccount { get; }

        public bool Equals(LiveAccountKey? other) => other != null
            && string.Equals(Connection, other.Connection, StringComparison.Ordinal)
            && string.Equals(Provider, other.Provider, StringComparison.Ordinal)
            && string.Equals(PlatformAccount, other.PlatformAccount, StringComparison.Ordinal);

        public override bool Equals(object? obj) => Equals(obj as LiveAccountKey);

        public override int GetHashCode()
        {
            unchecked
            {
                var h = StringComparer.Ordinal.GetHashCode(Connection);
                h = h * 31 + StringComparer.Ordinal.GetHashCode(Provider);
                return h * 31 + StringComparer.Ordinal.GetHashCode(PlatformAccount);
            }
        }

        public override string ToString() => Connection + "/" + Provider + "/" + PlatformAccount;
    }

    /// <summary>
    /// The local opt-in as the engine reads it. The host keeps it in a per-Windows-user file protected with
    /// DPAPI CurrentUser (ADR-0005 N11); only the plugin window's confirmation dialog writes it.
    /// </summary>
    public interface ILiveAccountConsent
    {
        /// <summary>"Allow live accounts" is on.</summary>
        bool Enabled { get; }

        /// <summary>When it was last turned on; commands received earlier are not covered.</summary>
        DateTime? EnabledAt { get; }

        bool IsConfirmed(LiveAccountKey key);

        /// <summary>The account's kind changed: its confirmation no longer counts (Vigia I1) until confirmed again.</summary>
        void Invalidate(LiveAccountKey key);
    }

    /// <summary>
    /// The consent document. <see cref="Parse"/> is strict: anything that is not exactly what <see cref="Serialize"/>
    /// writes means off with no confirmed account (fail closed). The host persists the serialized text through
    /// DPAPI and calls <see cref="AdapterEngine.OnLiveConsentChanged"/> after a change. The engine reads it on its own
    /// thread: <see cref="Allow"/> and <see cref="Block"/> must run there too (through <see cref="EngineLoop"/>).
    /// </summary>
    public sealed class LiveConsentState : ILiveAccountConsent
    {
        private const int FormatVersion = 1;
        private readonly HashSet<LiveAccountKey> _confirmed = new HashSet<LiveAccountKey>();

        public bool Enabled { get; private set; }
        public DateTime? EnabledAt { get; private set; }
        public IReadOnlyCollection<LiveAccountKey> Confirmed => _confirmed;

        /// <summary>Raised after <see cref="Invalidate"/> so the host can persist the document.</summary>
        public event Action? Changed;

        public static LiveConsentState Off() => new LiveConsentState();

        /// <summary>The dialog's "Allow": confirms exactly the live accounts it named.</summary>
        public void Allow(IEnumerable<LiveAccountKey> accounts, DateTime nowUtc)
        {
            _confirmed.Clear();
            foreach (var key in accounts) _confirmed.Add(key);
            Enabled = true;
            EnabledAt = nowUtc;
        }

        /// <summary>"Block live accounts": takes effect at once. Confirmations are kept so the C states apply.</summary>
        public void Block()
        {
            Enabled = false;
        }

        public bool IsConfirmed(LiveAccountKey key) => _confirmed.Contains(key);

        public void Invalidate(LiveAccountKey key)
        {
            if (_confirmed.Remove(key)) Changed?.Invoke();
        }

        public string Serialize()
        {
            var accounts = new JArray();
            foreach (var key in _confirmed.OrderBy(k => k.ToString(), StringComparer.Ordinal))
                accounts.Add(new JObject { ["connection"] = key.Connection, ["provider"] = key.Provider, ["platform_account"] = key.PlatformAccount });
            var doc = new JObject
            {
                ["v"] = FormatVersion,
                ["enabled"] = Enabled,
                ["enabled_at"] = EnabledAt.HasValue ? UtcTimestamp.Format(EnabledAt.Value) : null,
                ["accounts"] = accounts,
            };
            return doc.ToString(Formatting.None);
        }

        /// <summary>Missing, unreadable or invalid text → off, nothing confirmed (ADR-0005 N11 "default off").</summary>
        public static LiveConsentState Parse(string? text)
        {
            if (string.IsNullOrEmpty(text)) return Off();
            try
            {
                var doc = ProtocolJson.ParseObject(text!);
                if (doc.Properties().Select(p => p.Name).Except(new[] { "v", "enabled", "enabled_at", "accounts" }).Any()) return Off();
                if (doc["v"]?.Type != JTokenType.Integer || (int)doc["v"]! != FormatVersion) return Off();
                if (doc["enabled"]?.Type != JTokenType.Boolean || !(doc["accounts"] is JArray accounts)) return Off();
                var state = new LiveConsentState { Enabled = (bool)doc["enabled"]! };
                var at = doc["enabled_at"];
                if (at != null && at.Type != JTokenType.Null)
                {
                    if (at.Type != JTokenType.String) return Off();
                    state.EnabledAt = UtcTimestamp.Parse((string)at!);
                }
                if (state.Enabled && !state.EnabledAt.HasValue) return Off();
                foreach (var item in accounts)
                {
                    if (!(item is JObject o) || o.Count != 3) return Off();
                    var connection = o["connection"]?.Type == JTokenType.String ? (string)o["connection"]! : null;
                    var provider = o["provider"]?.Type == JTokenType.String ? (string)o["provider"]! : null;
                    var name = o["platform_account"]?.Type == JTokenType.String ? (string)o["platform_account"]! : null;
                    if (string.IsNullOrEmpty(connection) || string.IsNullOrEmpty(provider) || string.IsNullOrEmpty(name)) return Off();
                    state._confirmed.Add(new LiveAccountKey(connection!, provider!, name!));
                }
                return state;
            }
            catch (Exception e) when (e is JsonException || e is FormatException || e is InvalidCastException || e is ArgumentException)
            {
                return Off();
            }
        }

        public override string ToString() =>
            "enabled=" + Enabled.ToString(CultureInfo.InvariantCulture) + " accounts=" + _confirmed.Count.ToString(CultureInfo.InvariantCulture);
    }
}
