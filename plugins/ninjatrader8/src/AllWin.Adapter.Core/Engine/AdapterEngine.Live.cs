using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Engine
{
    // Position identity (ADR-0004 §5 position-identity guard, U13) and the live-account decision table
    // (ADR-0005 N11, U15/U19/U19.1).
    public sealed partial class AdapterEngine
    {
        public const string NoticeUnconsentedActions = "live.unconsented_actions";
        public const string NoticeUnprotectedPosition = "live.unprotected_position";
        public const string NoticeKindUnknown = "live.kind_unknown_alert";

        private static readonly TimeSpan UnknownKindAlertAfter = TimeSpan.FromSeconds(10);
        private readonly Dictionary<string, long> _positionSeqs = new Dictionary<string, long>(StringComparer.Ordinal);
        private readonly Dictionary<string, DateTime> _unknownSince = new Dictionary<string, DateTime>(StringComparer.Ordinal);

        private readonly Dictionary<string, PositionIdentityRecord> _identities = new Dictionary<string, PositionIdentityRecord>(StringComparer.Ordinal);
        private readonly Dictionary<string, HashSet<string>> _liveNotices = new Dictionary<string, HashSet<string>>(StringComparer.Ordinal);

        /// <summary>"Allow live accounts" is on (shown in the window header, release builds only).</summary>
        public bool LiveAccountsAllowed => _consent.Enabled;

        // ------------------------------------------------------------------ position identity (U13)

        /// <summary>
        /// Applies one fill to the identity record of its (account, instrument), in the fill's journal batch.
        /// The identity is set only from flat or on a flip; adds and partial closes keep it; flat ends it. Anything
        /// the record cannot explain (a missed fill, a correction of the opening fill) makes it unknown.
        /// </summary>
        private void UpdateIdentity(string account, PlatformExecution execution, FillOperation operation, int deltaQuantity, string? commandId, char letter, int after)
        {
            var key = OwnerKey(account, execution.Instrument);
            _identities.TryGetValue(key, out var held);
            if (after == 0)
            {
                RemoveIdentity(key);
                return;
            }
            var before = held?.Quantity ?? 0;
            var signed = execution.Side == Side.Buy ? deltaQuantity : -deltaQuantity;
            PositionIdentityRecord next;
            if (operation != FillOperation.Add && held != null && held.OpeningFillId == execution.ExecutionId)
                next = UnknownIdentity(account, execution.Instrument, after, execution.ExecutionId, "opening fill corrected");
            else if (before + signed != after)
                next = UnknownIdentity(account, execution.Instrument, after, execution.ExecutionId, "position does not follow the recorded fills");
            else if (before == 0 || (before > 0) != (after > 0))
            {
                // From flat, or a flip: this fill's own command, if it is an All Win entry; a non-All Win fill is external.
                var identity = commandId == null ? PositionIdentityRecord.External : letter == 'e' ? commandId : PositionIdentityRecord.Unknown;
                next = NewIdentity(account, execution.Instrument, identity, after, execution.ExecutionId, execution.ExecutionId);
            }
            else next = NewIdentity(account, execution.Instrument, held!.Identity, after, execution.ExecutionId, held.OpeningFillId);
            SetIdentity(key, next);
        }

        /// <summary>The identity of the current position; null when flat. A record that disagrees with the platform is unknown.</summary>
        private string? IdentityOf(string account, Instrument instrument, int position, bool readOnly = false)
        {
            if (position == 0) return null;
            var key = OwnerKey(account, instrument);
            if (!_identities.TryGetValue(key, out var held)) return PositionIdentityRecord.Unknown;
            if (held.Quantity != position)
            {
                if (readOnly) return PositionIdentityRecord.Unknown;
                // E.g. an execution still queued (ADR-0005 N6): fail closed.
                SetIdentity(key, UnknownIdentity(account, instrument, position, held.LastFillId, "live position differs from the record"));
                return PositionIdentityRecord.Unknown;
            }
            return held.Identity;
        }

        /// <summary>
        /// At startup (after catch-up) and when an account connection comes back: a record whose quantity or last fill
        /// differs from the platform becomes unknown (e.g. closed and reopened the same size while we were down).
        /// </summary>
        private void VerifyIdentities(string? onlyAccount = null)
        {
            foreach (var pair in _identities.ToList())
            {
                var held = pair.Value;
                if (onlyAccount != null && held.Account != onlyAccount) continue;
                if (held.Identity == PositionIdentityRecord.Unknown || !IsObservable(held.Account)) continue;
                var position = PositionQuantity(held.Account, held.Instrument);
                if (position == 0)
                {
                    RemoveIdentity(pair.Key);
                    continue;
                }
                var latest = SafeExecutions(held.Account).Where(e => e.Instrument.Equals(held.Instrument)).OrderBy(e => e.Time).LastOrDefault();
                if (position != held.Quantity || latest == null || latest.ExecutionId != held.LastFillId)
                    SetIdentity(pair.Key, UnknownIdentity(held.Account, held.Instrument, position, latest?.ExecutionId, "platform differs from the record after a restart or reconnect"));
            }
        }

        /// <summary>SimulationAccountReset / Playback rewind: no identity survives.</summary>
        private void ForgetIdentities(string account)
        {
            foreach (var pair in _identities.Where(p => p.Value.Account == account).ToList())
            {
                var position = PositionQuantity(account, pair.Value.Instrument);
                if (position == 0) RemoveIdentity(pair.Key);
                else SetIdentity(pair.Key, UnknownIdentity(account, pair.Value.Instrument, position, null, "account reset"));
            }
        }

        private PositionIdentityRecord UnknownIdentity(string account, Instrument instrument, int quantity, string? lastFillId, string why)
        {
            _log.Write(LogLevel.Warning, "identity.unknown", account + " " + instrument.Symbol + " " + instrument.ContractMonth + ": " + why);
            return NewIdentity(account, instrument, PositionIdentityRecord.Unknown, quantity, lastFillId, null);
        }

        private static PositionIdentityRecord NewIdentity(string account, Instrument instrument, string identity, int quantity, string? lastFillId, string? openingFillId) =>
            new PositionIdentityRecord
            {
                Account = account,
                Instrument = instrument.Clone(),
                Identity = identity,
                Quantity = quantity,
                LastFillId = lastFillId,
                OpeningFillId = openingFillId,
            };

        private void SetIdentity(string key, PositionIdentityRecord record)
        {
            _identities[key] = record;
            Put(Collections.Identities, key, record);
        }

        private void RemoveIdentity(string key)
        {
            if (_identities.Remove(key)) _pending.Add(new Store.StoreOp(Collections.Identities, key, null));
        }

        /// <summary>
        /// Step 2 of a reduce (ADR-0004 §5): null when the reduce may go on, else command.position_changed. A reduce from a
        /// pre-1.2 envelope cannot carry refs and never passes.
        /// </summary>
        private string? CheckPositionIdentity(CommandRecord record, int position)
        {
            var command = record.Command;
            var place = command.Place!;
            string? why = null;
            if (!IsAtLeast12(record.EnvelopeVersion) || command.Refs == null) why = "no position_ref (envelope " + (record.EnvelopeVersion ?? "1.0") + ")";
            else if (!((place.Side == Side.Sell && position > 0) || (place.Side == Side.Buy && position < 0))) why = "order side does not oppose the position";
            else
            {
                var identity = IdentityOf(record.PlatformAccount, command.Instrument, position);
                if (identity != command.Refs.PositionRef)
                    why = identity == PositionIdentityRecord.Unknown || identity == PositionIdentityRecord.External ? "identity " + identity : "another position";
            }
            if (why == null) return null;
            _log.Write(LogLevel.Warning, "reduce.position_changed", record.CommandId + ": " + why);
            return ReasonCodes.CommandPositionChanged;
        }

        private static bool IsAtLeast12(string? version)
        {
            if (version == null) return false;
            var parts = version.Split('.');
            return parts.Length == 2 && parts[0] == "1" && int.TryParse(parts[1], NumberStyles.None, CultureInfo.InvariantCulture, out var minor) && minor >= 2;
        }

        // ------------------------------------------------------------------ live accounts (ADR-0005 N11)

        /// <summary>
        /// The host calls this after the opt-in changed (dialog) or was loaded. Turning it off applies at once: working
        /// All Win entries on live accounts are cancelled and protection is only shrunk (rows 2–5).
        /// </summary>
        public void OnLiveConsentChanged()
        {
            Run("live_consent", () =>
            {
                _log.Write(LogLevel.Warning, "live.consent", _consent.ToString() + " at " + UtcTimestamp.Format(_clock.UtcNow));
                EnforceLiveTable();
                RaiseChanged();
            });
        }

        private static bool IsSimulated(PlatformAccountInfo info) => info.Kind == AccountKind.Simulation || info.Kind == AccountKind.Playback;

        private static LiveAccountKey? KeyOf(PlatformAccountInfo info) =>
            string.IsNullOrEmpty(info.ConnectionName) || string.IsNullOrEmpty(info.Provider) ? null : new LiveAccountKey(info.ConnectionName!, info.Provider!, info.Name);

        /// <summary>
        /// State B needs both gates: the local opt-in for this confirmed live account and welcome.simulation_only false.
        /// <paramref name="receivedAt"/> (a command's arrival) must not precede the opt-in: it applies to later commands.
        /// </summary>
        private bool IsStateB(PlatformAccountInfo info, DateTime? receivedAt)
        {
            if (info.Kind != AccountKind.Live || !IsConfirmed(info)) return false;
            if (!_consent.Enabled || _welcome == null || _welcome.SimulationOnly) return false;
            return receivedAt == null || (_consent.EnabledAt.HasValue && receivedAt.Value >= _consent.EnabledAt.Value);
        }

        private bool IsConfirmed(PlatformAccountInfo info)
        {
            var key = KeyOf(info);
            return info.Kind == AccountKind.Live && key != null && _consent.IsConfirmed(key);
        }

        /// <summary>The table state of a non-simulation account in one instrument (release builds).</summary>
        /// <remarks>
        /// Reads the identity without persisting "unknown" (Code Review M1): only the reduce guard does that (ADR-0004 §5
        /// "at the guard"). A position ahead of its record (a fill still queued) reads as C2 for this check only.
        /// </remarks>
        private LiveState LiveStateOf(PlatformAccountInfo info, Instrument instrument, DateTime? receivedAt = null)
        {
            if (!IsConfirmed(info)) return LiveState.A;
            if (IsStateB(info, receivedAt)) return LiveState.B;
            var position = PositionQuantity(info.Name, instrument);
            if (position == 0) return LiveState.C0;
            var identity = IdentityOf(info.Name, instrument, position, readOnly: true);
            return identity != null && identity != PositionIdentityRecord.External && identity != PositionIdentityRecord.Unknown ? LiveState.C1 : LiveState.C2;
        }

        /// <summary>
        /// The live-account state that restricts automatic actions on this contract, or null when normal rules apply
        /// (simulation account, or state B). Development builds never get here for live accounts: every write is refused.
        /// </summary>
        private LiveState? RestrictingLiveState(string account, Instrument instrument)
        {
            var info = SafeAccounts().FirstOrDefault(a => a.Name == account);
            if (info == null || IsSimulated(info) || _options.Build == AdapterBuild.Development) return null;
            var state = LiveStateOf(info, instrument);
            return state == LiveState.B ? (LiveState?)null : state;
        }

        /// <summary>Account-level state for the window: C2 if any contract is C2, else C1 if any is open, else C0.</summary>
        private LiveState? AccountLiveState(PlatformAccountInfo info)
        {
            if (IsSimulated(info) || _options.Build == AdapterBuild.Development) return null;
            if (!IsConfirmed(info)) return LiveState.A;
            if (IsStateB(info, null)) return LiveState.B;
            var states = _platform.GetPositions(info.Name).Where(p => p.Quantity != 0).Select(p => LiveStateOf(info, p.Instrument)).ToList();
            if (states.Contains(LiveState.C2)) return LiveState.C2;
            return states.Count > 0 ? LiveState.C1 : LiveState.C0;
        }

        /// <summary>State A, or a C state under U19 (b), which behaves as A.</summary>
        private bool IsEffectivelyA(LiveState state) =>
            state == LiveState.A || (state != LiveState.B && _options.LiveOffPolicy == LiveOffPolicy.RefuseAll);

        private bool TableAllows(LiveRow row, LiveState state) => LiveAccountTable.Allows(row, state, _options.LiveOffPolicy, _options.ProtectiveCancelPolicy);

        /// <summary>
        /// Gate for one platform write (ADR-0005 N4 + N11): simulation accounts always; development builds nothing else;
        /// release builds by the table row, where every row must be allowed.
        /// </summary>
        /// <param name="byCore">
        /// The write comes from a Core modify/cancel. On (effective) state A the Core is accepted only for row 2, and rows 3
        /// and 5 after the plugin's own check; rows 4 and 6 there are the plugin's own initiative (ADR-0005 §N11, QA L1).
        /// </param>
        private bool WriteAllowed(string account, Instrument instrument, DateTime? receivedAt, LiveRow[] rows, out LiveState? state, bool byCore = false)
        {
            state = null;
            var info = SafeAccounts().FirstOrDefault(a => a.Name == account);
            if (info == null) return false;
            if (IsSimulated(info)) return true;
            if (_options.Build == AdapterBuild.Development) return false;
            var s = LiveStateOf(info, instrument, receivedAt);
            state = s;
            // A write that names no row cannot be classified: refused outside state B (Vigia L1, fail closed).
            if (s == LiveState.B) return true;
            if (byCore && IsEffectivelyA(s) && rows.Any(r => r != LiveRow.CancelEntry && r != LiveRow.CancelOrphanLeg && r != LiveRow.CancelWrongSideLeg))
                return false;
            return rows.Length > 0 && rows.All(r => TableAllows(r, s));
        }

        /// <summary>Command-level check (consent first, then the table, then the command's own checks).</summary>
        private bool CommandAllowedOnAccount(PlatformAccountInfo info, CommandPayload command, DateTime receivedAt)
        {
            if (IsSimulated(info)) return true;
            if (_options.Build == AdapterBuild.Development) return false;
            var state = LiveStateOf(info, command.Instrument, receivedAt);
            if (state == LiveState.B) return true;
            switch (command.Kind)
            {
                case CommandKind.Place:
                    return command.Place!.PositionEffect == PositionEffect.Reduce && TableAllows(LiveRow.Reduce, state);
                case CommandKind.Flatten:
                    return TableAllows(LiveRow.Flatten, state);
                default:
                    return true; // modify / cancel: decided per target (rows 2–5, 7, 8, 11)
            }
        }

        /// <summary>A working exit of a reduce (place) command; flatten closes are handled by their flatten.</summary>
        private bool IsPlaceExit(TrackedOrder order) =>
            order.Letter == 'x' && !(_commands.TryGetValue(order.CommandId!, out var record) && record.Command.Kind == CommandKind.Flatten);

        /// <summary>Row of a cancel of one tracked All Win order, from the platform's own state (never the Core's word).</summary>
        private LiveRow CancelRowOf(TrackedOrder order)
        {
            if (!order.IsProtective) return LiveRow.CancelEntry;
            var position = PositionQuantity(order.Account, order.Order.Instrument);
            if (position == 0) return LiveRow.CancelOrphanLeg;
            var addsToPosition = (order.Order.Side == Side.Buy) == (position > 0);
            return addsToPosition ? LiveRow.CancelWrongSideLeg : LiveRow.CancelProtectingLeg;
        }

        /// <summary>An automatic action on a live account outside state B: a log line and, on A, the window alert.</summary>
        private void NoteLiveAction(string account, LiveState state, LiveRow row, int orders)
        {
            _log.Write(LogLevel.Warning, "live.action", account + " state=" + state + " row=" + ((int)row).ToString(CultureInfo.InvariantCulture)
                + " orders=" + orders.ToString(CultureInfo.InvariantCulture));
            var effective = state != LiveState.A && _options.LiveOffPolicy == LiveOffPolicy.RefuseAll ? LiveState.A : state;
            if (effective == LiveState.A) AddLiveNotice(account, NoticeUnconsentedActions);
        }

        /// <summary>
        /// §N11 transient unknown (Vigia I1): a mapped account whose kind stays unknown for more than 10 s gets the window
        /// alert; it clears when the kind is known again. Called from the engine timer.
        /// </summary>
        private void CheckUnknownKinds(DateTime now)
        {
            if (_options.Build == AdapterBuild.Development) return;
            var unknown = new HashSet<string>(SafeAccounts().Where(a => a.Kind == AccountKind.Unknown && MappingOf(a.Name) != null).Select(a => a.Name), StringComparer.Ordinal);
            foreach (var account in _unknownSince.Keys.Where(a => !unknown.Contains(a)).ToList())
            {
                _unknownSince.Remove(account);
                if (_liveNotices.TryGetValue(account, out var set) && set.Remove(NoticeKindUnknown)) RaiseChanged();
            }
            foreach (var account in unknown)
            {
                if (!_unknownSince.TryGetValue(account, out var since)) _unknownSince[account] = since = now;
                if (now - since > UnknownKindAlertAfter) AddLiveNotice(account, NoticeKindUnknown);
            }
        }

        private void AddLiveNotice(string account, string notice)
        {
            if (!_liveNotices.TryGetValue(account, out var set))
            {
                set = new HashSet<string>(StringComparer.Ordinal);
                _liveNotices[account] = set;
            }
            if (set.Add(notice)) RaiseChanged();
        }

        /// <summary>
        /// Rows 2–5 on the plugin's own initiative for every live account outside state B (ADR-0005 §N11 "How the rows
        /// are applied"): working All Win entries are cancelled, orphan legs cancelled, legs shrunk to the position and
        /// legs on the wrong side cancelled. Only orders with an aw: Name are touched. A confirmation whose account kind
        /// changed is invalidated first.
        /// </summary>
        private void EnforceLiveTable(string? onlyAccount = null)
        {
            if (_options.Build == AdapterBuild.Development) return; // never writes to a live account anyway (N4)
            var accounts = SafeAccounts();
            foreach (var info in accounts)
            {
                // Stateless rule (ADR-0005 §N11, Vigia L1): only a KNOWN non-live kind (simulation or playback) invalidates the
                // confirmation, whether or not the account is mapped or connected (QA M1). A transient unknown never does.
                var key = KeyOf(info);
                if (key != null && IsSimulated(info) && _consent.IsConfirmed(key))
                {
                    _consent.Invalidate(key);
                    _log.Write(LogLevel.Warning, "live.confirmation_invalidated", key + " kind=" + info.Kind);
                }
            }
            foreach (var info in accounts)
            {
                if (onlyAccount != null && info.Name != onlyAccount) continue;
                if (IsSimulated(info) || MappingOf(info.Name) == null) continue;
                if (!info.Connected || IsStateB(info, null)) continue;

                var account = info.Name;
                // While the kind is unknown, row-2 orders are not self-cancelled: they were submitted with consent (§N11
                // transient unknown). Rows 3–6 below still apply.
                var selfCancel = info.Kind != AccountKind.Unknown;
                foreach (var group in _orders.Values.Where(o => selfCancel && o.Account == account && o.CommandId != null && !o.Order.IsTerminal && !o.CancelRequested
                        && (o.Letter == 'e' || IsPlaceExit(o)))
                    .GroupBy(o => o.Order.Instrument).ToList())
                {
                    var state = LiveStateOf(info, group.Key);
                    // Working exits are undone too where exits are refused (effective state A): if the position changes by
                    // hand, a working exit could open or flip it (I1). With U19 (a) a C account keeps its exits.
                    var effectiveA = state == LiveState.A || _options.LiveOffPolicy == LiveOffPolicy.RefuseAll;
                    var entries = group.Where(o => o.Letter == 'e' || effectiveA).ToList();
                    if (entries.Count == 0) continue;
                    foreach (var o in entries)
                    {
                        o.CancelRequested = true;
                        o.CancelReason = ReasonCodes.OrderLiveConsentCancelled;
                    }
                    try
                    {
                        GuardedCancel(account, group.Key, entries.Select(o => o.Handle).ToList(), new[] { LiveRow.CancelEntry });
                        NoteLiveAction(account, state, LiveRow.CancelEntry, entries.Count);
                    }
                    catch (PlatformException e)
                    {
                        _log.Write(LogLevel.Error, "live.cancel_entries_failed", account + ": " + e.Message);
                        foreach (var o in entries)
                        {
                            o.CancelRequested = false;
                            o.CancelReason = null;
                            ReportOrderIssue(o, ReasonCodes.OrderCancelFailed);
                        }
                    }
                }

                var positions = _platform.GetPositions(account);
                var legInstruments = _orders.Values.Where(o => o.Account == account && o.IsProtective && !o.Order.IsTerminal).Select(o => o.Order.Instrument);
                foreach (var instrument in positions.Select(p => p.Instrument).Concat(legInstruments).Distinct().ToList())
                {
                    var position = positions.FirstOrDefault(p => p.Instrument.Equals(instrument));
                    if ((position?.Quantity ?? 0) == 0)
                        CancelOrphans(account, LiveOrders(account, instrument).Where(o => o.IsProtective).ToList(), LiveRow.CancelOrphanLeg);
                    else
                        ReconcileProtection(account, instrument, position, null, live: false, allowInitial: false);
                }
            }
        }
    }
}
