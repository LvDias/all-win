using System.Collections.Generic;
using System.Linq;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Engine
{
    // Snapshots, restart recovery, bracket-integrity evaluation (ADR-0004 §6.1) and the account view.
    public sealed partial class AdapterEngine
    {
        // ------------------------------------------------------------------ snapshots

        private void EnqueueSnapshot(SnapshotTrigger trigger, string? onlyAccount)
        {
            var snapshot = new PositionSnapshotPayload { Trigger = trigger, TakenAt = _clock.UtcNow };
            foreach (var mapping in _welcome?.Accounts ?? new List<WelcomeAccount>())
            {
                if (onlyAccount != null && mapping.PlatformAccount != onlyAccount) continue;
                snapshot.Accounts.Add(SnapshotOf(mapping));
            }
            if (onlyAccount != null && snapshot.Accounts.Count == 0) return;
            Enqueue(snapshot);
        }

        private SnapshotAccount SnapshotOf(WelcomeAccount mapping)
        {
            var account = mapping.PlatformAccount;
            var info = SafeAccounts().FirstOrDefault(a => a.Name == account);
            var result = new SnapshotAccount { AccountId = mapping.AccountId, PlatformAccount = account, Connected = info?.Connected ?? false };
            if (info == null) return result;

            foreach (var position in _platform.GetPositions(account).Where(p => p.Quantity != 0))
            {
                result.Positions.Add(new SnapshotPosition
                {
                    Instrument = position.Instrument.Clone(),
                    Position = new PositionInfo { Quantity = position.Quantity, AveragePrice = position.AveragePrice > 0 ? position.AveragePrice : null },
                });
            }
            foreach (var order in SafeOrders(account).Where(o => !o.IsTerminal))
            {
                var working = new SnapshotWorkingOrder
                {
                    PlatformOrderId = order.PlatformOrderId,
                    Instrument = order.Instrument.Clone(),
                    Side = order.Side,
                    Quantity = System.Math.Max(1, order.Quantity - order.FilledQuantity),
                    Order = SpecOf(order),
                    Origin = OrderOrigin.External,
                    State = SnapshotState(order.State),
                };
                if (OrderName.TryParse(order.Name, out var commandId, out var letter))
                {
                    working.CommandId = commandId;
                    working.Origin = OrderOrigin.AllWin;
                    working.ChildRole = OrderName.RoleOf(letter, _commands.TryGetValue(commandId, out var record) && record.Command.Kind == CommandKind.Flatten);
                }
                result.WorkingOrders.Add(working);
            }
            return result;
        }

        private static WorkingOrderState SnapshotState(PlatformOrderState state)
        {
            switch (state)
            {
                case PlatformOrderState.Initialized:
                case PlatformOrderState.Submitted:
                case PlatformOrderState.Accepted: return WorkingOrderState.Accepted;
                case PlatformOrderState.TriggerPending: return WorkingOrderState.TriggerPending;
                case PlatformOrderState.Unknown: return WorkingOrderState.Unknown;
                default: return WorkingOrderState.Working;
            }
        }

        // ------------------------------------------------------------------ restart recovery

        /// <summary>
        /// After a restart: rebuild the queues, re-link platform orders by <c>Order.Name</c>, report what
        /// changed while we were down, catch up executions (deduped by fill_id) and evaluate brackets.
        /// A command that was handed to the platform is never handed again.
        /// </summary>
        private void Recover()
        {
            foreach (var record in _commands.Values.Where(r => r.Phase != CommandPhase.Done).OrderBy(r => r.Seq))
            {
                if (record.Phase == CommandPhase.Queued || record.Command.Kind == CommandKind.Flatten)
                    Queue(record.PlatformAccount).Add(record.CommandId);
            }

            RefreshTrackedOrders();

            foreach (var record in _commands.Values.Where(r => r.Phase == CommandPhase.Dispatched).OrderBy(r => r.Seq).ToList())
            {
                switch (record.Command.Kind)
                {
                    case CommandKind.Place:
                        var letter = record.Command.Place!.PositionEffect == PositionEffect.Open ? 'e' : 'x';
                        if (FindOrder(record.PlatformAccount, record.CommandId, letter) == null)
                        {
                            // Submitted (or about to be) when we stopped, and the platform does not know the order:
                            // its fate is unknown. Never re-submit (never execute twice).
                            _log.Write(LogLevel.Warning, "recovery.order_missing", record.CommandId);
                            Finish(record, false, ReportStatus.Unknown, null);
                        }
                        break;
                    case CommandKind.Modify:
                    case CommandKind.Cancel:
                        foreach (var target in record.Targets?.Where(t => t.State == TargetState.Pending) ?? Enumerable.Empty<TargetRecord>())
                        {
                            // Handles are per process: re-resolve by (command_id, role).
                            var order = FindOrder(record.PlatformAccount, target.CommandId, OrderName.Letter(target.ChildRole));
                            if (order == null) FailTarget(record, target, null, ReasonCodes.CommandTargetNotFound);
                            else target.Handle = order.Handle;
                        }
                        foreach (var order in _orders.Values.Where(o => record.Targets != null && record.Targets.Any(t => t.Handle == o.Handle)).ToList())
                            EvaluateTargets(order);
                        CompleteTargetsIfDone(record);
                        break;
                }
            }

            // Catch-up (answer A): fills that happened while we were down are reported, but no bracket is
            // submitted late and no leg is extended or re-priced; the user decides (alert and wait).
            _catchUp = true;
            try
            {
                foreach (var mapping in _welcome?.Accounts ?? new List<WelcomeAccount>())
                {
                    foreach (var execution in SafeExecutions(mapping.PlatformAccount).OrderBy(e => e.Time))
                        HandleExecution(mapping.PlatformAccount, execution);
                    EvaluateIntegrity(mapping.PlatformAccount);
                }
            }
            finally { _catchUp = false; }

            foreach (var record in _commands.Values.Where(r => r.Phase == CommandPhase.Dispatched && r.Command.Kind == CommandKind.Flatten).OrderBy(r => r.Seq).ToList())
            {
                // An account we cannot observe yet is decided when its connection is up (never on guesses).
                if (IsObservable(record.PlatformAccount)) RecoverFlatten(record);
                else _deferredFlattens.Add(record.CommandId);
            }
            RaiseChanged();
        }

        /// <summary>
        /// A flatten that was running when the process stopped. Cancelling resumes (cancels are idempotent).
        /// Closing never sends another close order after a restart: we cannot tell whether the last one reached
        /// the platform, so the outcome is decided from what the platform shows.
        /// </summary>
        private void RecoverDeferredFlattens(string account)
        {
            foreach (var id in _deferredFlattens.ToList())
            {
                if (!_commands.TryGetValue(id, out var record) || record.PlatformAccount != account) continue;
                _deferredFlattens.Remove(id);
                if (record.Phase == CommandPhase.Dispatched) RecoverFlatten(record);
            }
        }

        private void RecoverFlatten(CommandRecord record)
        {
            if (record.FlattenStep != FlattenStep.Closing)
            {
                record.StepStartedAt = _clock.UtcNow;
                AdvanceFlatten(record);
                return;
            }
            var close = FindOrder(record.PlatformAccount, record.CommandId, 'x');
            if (close != null && !close.Order.IsTerminal) return; // still working: AdvanceFlatten follows its events
            if (close == null) ConcludeLostClose(record);
            else if (PositionQuantity(record.PlatformAccount, record.Command.Instrument) == 0) Finish(record, true, null, null, ChildRole.Flatten);
            else Finish(record, false, ReportStatus.Unknown, null, ChildRole.Flatten);
        }

        /// <summary>Re-reads platform orders of mapped accounts and reports any lifecycle change not yet reported.</summary>
        private void RefreshTrackedOrders(string? onlyAccount = null)
        {
            foreach (var mapping in _welcome?.Accounts ?? new List<WelcomeAccount>())
            {
                var account = mapping.PlatformAccount;
                if (onlyAccount != null && account != onlyAccount) continue;
                if (SafeAccounts().All(a => a.Name != account)) continue;
                foreach (var order in SafeOrders(account)) AdoptPlatformOrder(account, order);
            }
        }

        // ------------------------------------------------------------------ bracket integrity (ADR-0004 §6.1)

        /// <summary>
        /// The adapter acts only on the orphan leg (cancel it); every other case is surfaced and left to the
        /// user and the Core (alert and wait; Q-O1 b flatten comes from the Core as a command).
        /// </summary>
        private void EvaluateIntegrity(string account)
        {
            var info = SafeAccounts().FirstOrDefault(a => a.Name == account);
            if (info == null || !info.Connected) return;
            var positions = _platform.GetPositions(account);
            var legs = _orders.Values.Where(o => o.Account == account && o.IsProtective && !o.Order.IsTerminal).ToList();
            var instruments = positions.Select(p => p.Instrument).Concat(legs.Select(l => l.Order.Instrument)).Distinct().ToList();
            foreach (var instrument in instruments)
            {
                var quantity = positions.FirstOrDefault(p => p.Instrument.Equals(instrument))?.Quantity ?? 0;
                var instrumentLegs = legs.Where(l => l.Order.Instrument.Equals(instrument)).ToList();
                var protectedCommands = _commands.Values.Where(r => r.PlatformAccount == account && r.Command.Kind == CommandKind.Place
                    && r.Command.Place!.Protection != null && r.Command.Instrument.Equals(instrument) && r.EntryFilled > 0).ToList();
                var doubleFill = protectedCommands.Any(r => LegFilled(account, r.CommandId, 's') > 0 && LegFilled(account, r.CommandId, 't') > 0);

                if (doubleFill) AddStickyReason(account, ReasonCodes.SyncBracketDoubleFill);
                if (quantity == 0 && instrumentLegs.Count > 0)
                {
                    CancelOrphans(account, instrumentLegs);
                    AddStickyReason(account, ReasonCodes.SyncBracketOrphanLeg);
                }
                else if (quantity != 0)
                {
                    var unknown = instrumentLegs.Any(l => l.Order.State == PlatformOrderState.Unknown);
                    var shouldBeProtected = _owners.ContainsKey(OwnerKey(account, instrument))
                        || protectedCommands.Any(r => r.Command.Place!.Protection!.Mode == ProtectionMode.Individual
                            && r.EntryFilled - LegFilled(account, r.CommandId, 's') - LegFilled(account, r.CommandId, 't') > 0);
                    if (unknown || (instrumentLegs.Count == 0 && shouldBeProtected && !doubleFill))
                        AddStickyReason(account, ReasonCodes.SyncBracketIncomplete);
                }
            }
        }

        private void AddStickyReason(string account, string reason)
        {
            if (!_stickyLocalReasons.TryGetValue(account, out var set))
            {
                set = new HashSet<string>();
                _stickyLocalReasons[account] = set;
            }
            if (!set.Add(reason)) return;
            _log.Write(LogLevel.Warning, "sync.local_reason", account + " " + reason);
            SaveStickyReasons();
        }

        /// <summary>Local hints are shown until the next welcome, across plugin restarts.</summary>
        private void SaveStickyReasons()
        {
            Put(Collections.Meta, "local_reasons", _stickyLocalReasons.ToDictionary(p => p.Key, p => p.Value.OrderBy(r => r, System.StringComparer.Ordinal).ToList()));
        }

        // ------------------------------------------------------------------ view

        /// <summary>Accounts as the plugin window shows them (call on the engine thread or via EngineLoop).</summary>
        public IReadOnlyList<AccountView> GetAccountViews()
        {
            var views = new List<AccountView>();
            foreach (var info in SafeAccounts())
            {
                var mapping = MappingOf(info.Name);
                var local = new List<string>();
                if (_stickyLocalReasons.TryGetValue(info.Name, out var sticky)) local.AddRange(sticky);
                var hasLegs = _orders.Values.Any(o => o.Account == info.Name && o.IsProtective && !o.Order.IsTerminal);
                if (mapping != null && hasLegs && info.OcoLocation != OcoLocation.Server && (Session != SessionState.Ready || !info.Connected))
                    local.Add(ReasonCodes.SyncBracketIntegrityUnknown);
                _coreState.TryGetValue(info.Name, out var core);
                views.Add(new AccountView
                {
                    PlatformAccount = info.Name,
                    AccountId = mapping?.AccountId,
                    Roles = mapping?.Roles.ToArray() ?? new AccountRole[0],
                    Kind = info.Kind,
                    OcoLocation = info.OcoLocation,
                    PlatformConnected = info.Connected,
                    ExecutionAllowed = IsExecutionAllowed(info),
                    CoreSyncState = core?.State,
                    CoreSyncReasons = core?.Reasons ?? new string[0],
                    LocalSyncReasons = local.Distinct().ToArray(),
                });
            }
            return views;
        }
    }
}
