using System;
using System.Collections.Generic;
using System.Linq;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Engine
{
    // Command intake, the per-account queue and the dispatch of place / modify / cancel / flatten.
    public sealed partial class AdapterEngine
    {
        /// <summary>Records the command durably (before the ack) and queues it. Returns the account to pump.</summary>
        private string? IntakeCommand(CommandPayload command, DateTime sentAt)
        {
            _commands.TryGetValue(command.CommandId, out var known);
            if (IsStale(sentAt))
            {
                // Checked before dedupe: a re-delivery that outlived its dedupe record is never executed.
                _log.Write(LogLevel.Warning, "command.stale", command.CommandId);
                if (known != null)
                {
                    var stale = NewReport(known.PlatformAccount, known.Command.Instrument, known.AccountId);
                    stale.CommandId = known.CommandId;
                    stale.Origin = OrderOrigin.AllWin;
                    stale.ChildRole = DefaultRole(known.Command);
                    stale.Status = ReportStatus.Rejected;
                    stale.ReasonCode = ReasonCodes.CommandStale;
                    EnqueueReport(stale); // the command's real latest state is kept
                    return null;
                }
            }
            else if (known != null)
            {
                // Business duplicate (new msg_id, same command_id): never executed again (ADR-0004 §4).
                ReportDuplicate(known);
                return null;
            }

            var record = new CommandRecord
            {
                CommandId = command.CommandId,
                Seq = _nextCommandSeq++,
                AccountId = command.AccountId,
                PlatformAccount = command.PlatformAccount,
                ReceivedAt = _clock.UtcNow,
                SentAt = sentAt,
                Command = command,
                Phase = CommandPhase.Queued,
            };
            _commands[record.CommandId] = record;

            if (IsStale(sentAt))
            {
                Finish(record, false, ReportStatus.Rejected, ReasonCodes.CommandStale);
                return null;
            }
            var refusal = CheckAccount(command);
            if (refusal != null)
            {
                Finish(record, false, ReportStatus.Rejected, refusal);
                return null;
            }
            if (command.IsEntry && IsExpired(command))
            {
                Finish(record, false, ReportStatus.Expired, ReasonCodes.CommandExpired);
                return null;
            }

            Save(record);
            CommandReport(record, ReportStatus.Received, null, withOrder: true);
            Queue(record.PlatformAccount).Add(record.CommandId);
            return record.PlatformAccount;
        }

        private bool IsStale(DateTime sentAt) => CoreNow - sentAt > _options.StalenessLimit;

        /// <summary>Mapping, destination role and simulation guard (S2 AC-5, ADR-0005 N4).</summary>
        private string? CheckAccount(CommandPayload command)
        {
            var mapping = _welcome?.Accounts.FirstOrDefault(a => a.AccountId == command.AccountId);
            if (mapping == null || mapping.PlatformAccount != command.PlatformAccount) return ReasonCodes.CommandAccountNotMapped;
            if (!mapping.Roles.Contains(AccountRole.Destination)) return ReasonCodes.CommandAccountNotDestination;
            var info = SafeAccounts().FirstOrDefault(a => a.Name == command.PlatformAccount);
            if (info == null) return ReasonCodes.CommandPlatformDisconnected;
            if (!IsExecutionAllowed(info)) return ReasonCodes.CommandAccountNotSimulation;
            return null;
        }

        /// <summary>
        /// Development builds: simulation/playback only, always. Release builds: simulation/playback when the
        /// Core says <c>simulation_only</c>; never an account whose kind is unknown (fail closed).
        /// </summary>
        private bool IsExecutionAllowed(PlatformAccountInfo info)
        {
            var simulated = info.Kind == AccountKind.Simulation || info.Kind == AccountKind.Playback;
            if (_options.Build == AdapterBuild.Development) return simulated;
            if (_welcome == null || _welcome.SimulationOnly) return simulated;
            return info.Kind != AccountKind.Unknown;
        }

        private bool IsExpired(CommandPayload command) => command.ExpiresAt.HasValue && CoreNow > command.ExpiresAt.Value;

        private bool IsBlocked(string accountId)
        {
            return _limits.TryGetValue(accountId, out var limits) && limits.BlockedUntil.HasValue && limits.BlockedUntil.Value > _clock.UtcNow;
        }

        private List<string> Queue(string platformAccount)
        {
            if (!_queues.TryGetValue(platformAccount, out var queue))
            {
                queue = new List<string>();
                _queues[platformAccount] = queue;
            }
            return queue;
        }

        private void PumpAll()
        {
            foreach (var account in _queues.Keys.ToList()) Pump(account);
        }

        /// <summary>
        /// Dispatches the account's commands strictly in arrival order (ADR-0004 §4). The queue advances
        /// when the head is handed to the platform; a flatten holds the queue until it is complete, and a
        /// command with depends_on waits for its prerequisite to be final.
        /// </summary>
        private void Pump(string platformAccount)
        {
            if (Session != SessionState.Ready) return; // commands only after the reconnect snapshot (answer 6)
            var queue = Queue(platformAccount);
            while (queue.Count > 0)
            {
                if (!_commands.TryGetValue(queue[0], out var record) || record.Phase == CommandPhase.Done)
                {
                    queue.RemoveAt(0);
                    continue;
                }
                if (record.Phase == CommandPhase.Dispatched) return; // a running flatten

                var command = record.Command;
                if (IsStale(record.SentAt))
                {
                    Finish(record, false, ReportStatus.Rejected, ReasonCodes.CommandStale);
                    continue;
                }
                if (command.DependsOn != null)
                {
                    if (!_commands.TryGetValue(command.DependsOn, out var prerequisite))
                    {
                        Finish(record, false, ReportStatus.Rejected, ReasonCodes.CommandPrerequisiteFailed);
                        continue;
                    }
                    if (prerequisite.Phase != CommandPhase.Done) return;
                    if (!prerequisite.Succeeded)
                    {
                        Finish(record, false, ReportStatus.Rejected, ReasonCodes.CommandPrerequisiteFailed);
                        continue;
                    }
                }
                // Answer 5: entries are re-checked right before submission (queue or depends_on wait).
                if (command.IsEntry && IsExpired(command))
                {
                    Finish(record, false, ReportStatus.Expired, ReasonCodes.CommandExpired);
                    continue;
                }
                var info = SafeAccounts().FirstOrDefault(a => a.Name == platformAccount);
                if (info == null || !info.Connected)
                {
                    Finish(record, false, ReportStatus.Rejected, ReasonCodes.CommandPlatformDisconnected);
                    continue;
                }
                if (!IsExecutionAllowed(info))
                {
                    Finish(record, false, ReportStatus.Rejected, ReasonCodes.CommandAccountNotSimulation);
                    continue;
                }
                if (command.IsEntry && IsBlocked(record.AccountId))
                {
                    Finish(record, false, ReportStatus.Rejected, ReasonCodes.RiskAccountBlocked);
                    continue;
                }

                switch (command.Kind)
                {
                    case CommandKind.Place: DispatchPlace(record); break;
                    case CommandKind.Modify: DispatchModify(record); break;
                    case CommandKind.Cancel: DispatchCancel(record); break;
                    case CommandKind.Flatten: DispatchFlatten(record); break;
                }
                if (command.Kind == CommandKind.Flatten && record.Phase != CommandPhase.Done) return;
                queue.RemoveAt(0);
            }
        }

        // ------------------------------------------------------------------ place

        private void DispatchPlace(CommandRecord record)
        {
            var command = record.Command;
            var place = command.Place!;
            if (_platform.GetTickSize(command.Instrument) == null)
            {
                Finish(record, false, ReportStatus.Rejected, ReasonCodes.CommandInstrumentUnavailable);
                return;
            }

            var quantity = place.Quantity;
            string? capReason = null;
            if (place.PositionEffect == PositionEffect.Reduce)
            {
                // A reduce never opens or flips a position, even after a race with a protective fill.
                var position = PositionQuantity(record.PlatformAccount, command.Instrument);
                var reducesLong = place.Side == Side.Sell && position > 0;
                var reducesShort = place.Side == Side.Buy && position < 0;
                if (!reducesLong && !reducesShort)
                {
                    Finish(record, false, ReportStatus.Cancelled, ReasonCodes.CommandNothingToReduce);
                    return;
                }
                if (quantity > Math.Abs(position))
                {
                    quantity = Math.Abs(position);
                    capReason = ReasonCodes.OrderQuantityCapped;
                }
            }

            var role = place.PositionEffect == PositionEffect.Open ? ChildRole.Entry : ChildRole.Exit;
            var request = new OrderRequest
            {
                Name = OrderName.Format(record.CommandId, role),
                Instrument = command.Instrument,
                Side = place.Side,
                Type = place.Order.Type,
                Quantity = quantity,
                LimitPrice = place.Order.LimitPrice,
                StopPrice = place.Order.StopPrice,
                TimeInForce = place.Order.TimeInForce ?? TimeInForce.Day,
            };
            record.EntryQuantity = quantity;
            SubmitFor(record, new[] { request }, capReason);
        }

        /// <summary>Write-ahead submit: the attempt is durable before the platform sees it, so it is never repeated.</summary>
        private void SubmitFor(CommandRecord record, IReadOnlyList<OrderRequest> requests, string? workingReason)
        {
            record.Phase = CommandPhase.Dispatched;
            record.SubmitAttempted = true;
            Save(record);
            Flush();
            try
            {
                var handles = _platform.Submit(record.PlatformAccount, requests);
                for (var i = 0; i < requests.Count && i < handles.Count; i++)
                    TrackSubmitted(record.PlatformAccount, handles[i], requests[i], workingReason);
            }
            catch (PlatformException e)
            {
                _log.Write(LogLevel.Warning, "platform.submit_failed", record.CommandId + ": " + e.Message);
                var role = OrderName.TryParse(requests[0].Name, out _, out var letter) ? OrderName.RoleOf(letter, record.Command.Kind == CommandKind.Flatten) : ChildRole.Entry;
                Finish(record, false, ReportStatus.Rejected, ReasonCodes.OrderPlatformRejected, role);
            }
        }

        // ------------------------------------------------------------------ modify / cancel

        private void DispatchModify(CommandRecord record)
        {
            var modify = record.Command.Modify!;
            record.Targets = new List<TargetRecord>();
            foreach (var target in modify.Targets)
            {
                var targetRecord = new TargetRecord { CommandId = target.CommandId, ChildRole = target.ChildRole, State = TargetState.Pending };
                record.Targets.Add(targetRecord);
                var order = FindLiveOrder(record.PlatformAccount, target.CommandId, target.ChildRole);
                if (order == null)
                {
                    FailTarget(record, targetRecord, target.PlatformOrderId, ReasonCodes.CommandTargetNotFound);
                    continue;
                }
                targetRecord.Handle = order.Handle;
                decimal? limit = null, stop = null;
                if (modify.PriceDelta.HasValue)
                {
                    var delta = modify.PriceDelta.Value;
                    if (order.Order.Type == OrderType.Market)
                    {
                        FailTarget(record, targetRecord, order.Order.PlatformOrderId, ReasonCodes.CommandTargetNotFound);
                        continue;
                    }
                    if (order.Order.LimitPrice.HasValue && order.Order.Type != OrderType.Stop) limit = TickMath.Normalize(order.Order.LimitPrice.Value + delta);
                    if (order.Order.StopPrice.HasValue && order.Order.Type != OrderType.Limit) stop = TickMath.Normalize(order.Order.StopPrice.Value + delta);
                }
                targetRecord.Quantity = modify.Quantity;
                targetRecord.LimitPrice = limit;
                targetRecord.StopPrice = stop;
            }

            record.Phase = CommandPhase.Dispatched;
            record.SubmitAttempted = true;
            Save(record);
            Flush();
            foreach (var target in record.Targets.Where(t => t.State == TargetState.Pending))
            {
                var order = _orders[target.Handle!];
                order.ChangeSeen = false;
                try
                {
                    _platform.Change(record.PlatformAccount, target.Handle!, target.Quantity, target.LimitPrice, target.StopPrice);
                }
                catch (PlatformException e)
                {
                    _log.Write(LogLevel.Warning, "platform.change_failed", record.CommandId + ": " + e.Message);
                    FailTarget(record, target, order.Order.PlatformOrderId, ReasonCodes.OrderPlatformRejected);
                }
            }
            CompleteTargetsIfDone(record);
        }

        private void DispatchCancel(CommandRecord record)
        {
            record.Targets = new List<TargetRecord>();
            foreach (var target in record.Command.Cancel!.Targets)
            {
                var targetRecord = new TargetRecord { CommandId = target.CommandId, ChildRole = target.ChildRole, State = TargetState.Pending };
                record.Targets.Add(targetRecord);
                var order = FindLiveOrder(record.PlatformAccount, target.CommandId, target.ChildRole);
                if (order == null) FailTarget(record, targetRecord, target.PlatformOrderId, ReasonCodes.CommandTargetNotFound);
                else targetRecord.Handle = order.Handle;
            }

            record.Phase = CommandPhase.Dispatched;
            record.SubmitAttempted = true;
            Save(record);
            Flush();
            var pending = record.Targets.Where(t => t.State == TargetState.Pending).ToList();
            if (pending.Count > 0)
            {
                foreach (var target in pending) _orders[target.Handle!].CancelRequested = true;
                try
                {
                    _platform.Cancel(record.PlatformAccount, pending.Select(t => t.Handle!).ToList());
                }
                catch (PlatformException e)
                {
                    _log.Write(LogLevel.Warning, "platform.cancel_failed", record.CommandId + ": " + e.Message);
                    foreach (var target in pending) FailTarget(record, target, _orders[target.Handle!].Order.PlatformOrderId, ReasonCodes.OrderCancelFailed);
                }
            }
            CompleteTargetsIfDone(record);
        }

        /// <summary>Resolves pending modify/cancel targets from a confirmed platform state (not from the API call).</summary>
        private void EvaluateTargets(TrackedOrder order)
        {
            foreach (var record in _commands.Values.Where(r => r.Phase == CommandPhase.Dispatched && r.Targets != null).ToList())
            {
                foreach (var target in record.Targets!.Where(t => t.State == TargetState.Pending && t.Handle == order.Handle))
                {
                    var o = order.Order;
                    if (record.Command.Kind == CommandKind.Cancel)
                    {
                        if (o.State == PlatformOrderState.Cancelled) ApplyTarget(record, target, order, ReportStatus.Cancelled);
                        else if (o.IsTerminal) FailTarget(record, target, o.PlatformOrderId, ReasonCodes.OrderCancelFailed);
                        continue;
                    }
                    if (o.IsTerminal)
                    {
                        FailTarget(record, target, o.PlatformOrderId, ReasonCodes.OrderPlatformRejected);
                        continue;
                    }
                    if (!IsWorkingState(o.State)) continue;
                    var matches = (!target.Quantity.HasValue || o.Quantity == target.Quantity.Value)
                        && (!target.LimitPrice.HasValue || o.LimitPrice == target.LimitPrice)
                        && (!target.StopPrice.HasValue || o.StopPrice == target.StopPrice);
                    if (matches) ApplyTarget(record, target, order, ReportStatus.Working);
                    else if (order.ChangeSeen) FailTarget(record, target, o.PlatformOrderId, ReasonCodes.OrderPlatformRejected);
                }
                CompleteTargetsIfDone(record);
            }
        }

        private void ApplyTarget(CommandRecord record, TargetRecord target, TrackedOrder order, ReportStatus status)
        {
            target.State = TargetState.Applied;
            // Outcome report under the modify/cancel command: never fill or position_after (answer 9a).
            var report = NewReport(record.PlatformAccount, record.Command.Instrument);
            report.CommandId = record.CommandId;
            report.Origin = OrderOrigin.AllWin;
            report.ChildRole = target.ChildRole;
            report.PlatformOrderId = NullIfEmpty(order.Order.PlatformOrderId);
            report.Status = status;
            report.Side = order.Order.Side;
            report.Order = SpecOf(order.Order);
            report.Quantity = order.Order.Quantity;
            Emit(record, report);
        }

        private void FailTarget(CommandRecord record, TargetRecord target, string? platformOrderId, string reason)
        {
            target.State = TargetState.Failed;
            var report = NewReport(record.PlatformAccount, record.Command.Instrument);
            report.CommandId = record.CommandId;
            report.Origin = OrderOrigin.AllWin;
            report.ChildRole = target.ChildRole;
            report.PlatformOrderId = NullIfEmpty(platformOrderId);
            report.Status = ReportStatus.Rejected;
            report.ReasonCode = reason;
            Emit(record, report);
        }

        private void CompleteTargetsIfDone(CommandRecord record)
        {
            if (record.Phase == CommandPhase.Done || record.Targets == null) return;
            if (record.Targets.Any(t => t.State == TargetState.Pending))
            {
                Save(record);
                return;
            }
            // Answer 9c: success for depends_on only when every target was applied; no rollback (9d).
            Finish(record, record.Targets.All(t => t.State == TargetState.Applied), null, null);
            Pump(record.PlatformAccount);
        }

        // ------------------------------------------------------------------ flatten

        private void DispatchFlatten(CommandRecord record)
        {
            record.Phase = CommandPhase.Dispatched;
            record.SubmitAttempted = true;
            record.FlattenStep = FlattenStep.Cancelling;
            record.StepStartedAt = _clock.UtcNow;
            Save(record);
            Flush();
            AdvanceFlatten(record);
        }

        private CommandRecord? ActiveFlatten(string platformAccount, Instrument instrument)
        {
            return _commands.Values.FirstOrDefault(r => r.Phase == CommandPhase.Dispatched && r.Command.Kind == CommandKind.Flatten
                && r.PlatformAccount == platformAccount && r.Command.Instrument.Equals(instrument));
        }

        /// <summary>
        /// flatten = cancel every working order on the contract, wait for the confirmations, then close the
        /// remaining position with a market order. Closing over still-working orders could flip the position.
        /// </summary>
        private void AdvanceFlatten(CommandRecord record)
        {
            if (record.Phase != CommandPhase.Dispatched) return;
            var account = record.PlatformAccount;
            var instrument = record.Command.Instrument;
            var closeOrder = FindOrder(account, record.CommandId, 'x');

            if (record.FlattenStep == FlattenStep.Cancelling)
            {
                var working = LiveOrders(account, instrument).Where(o => closeOrder == null || o.Handle != closeOrder.Handle).ToList();
                var toCancel = working.Where(o => !o.CancelRequested).ToList();
                if (toCancel.Count > 0)
                {
                    foreach (var o in toCancel) o.CancelRequested = true;
                    try { _platform.Cancel(account, toCancel.Select(o => o.Handle).ToList()); }
                    catch (PlatformException e)
                    {
                        _log.Write(LogLevel.Warning, "platform.cancel_failed", record.CommandId + ": " + e.Message);
                        Finish(record, false, ReportStatus.Rejected, ReasonCodes.OrderCancelFailed, ChildRole.Flatten);
                        Pump(account);
                        return;
                    }
                }
                if (working.Count > 0) return; // wait for every cancel to be confirmed

                var position = PositionQuantity(account, instrument);
                if (position == 0)
                {
                    // Answer 4: terminal success, so a reverse continues to its place.
                    Finish(record, true, record.FlattenAttempts == 0 ? ReportStatus.Cancelled : (ReportStatus?)null,
                        record.FlattenAttempts == 0 ? ReasonCodes.CommandAlreadyFlat : null, ChildRole.Flatten);
                    Pump(account);
                    return;
                }
                if (record.FlattenAttempts >= _options.FlattenMaxAttempts)
                {
                    Finish(record, false, ReportStatus.Rejected, ReasonCodes.OrderPlatformRejected, ChildRole.Flatten);
                    Pump(account);
                    return;
                }

                record.FlattenAttempts++;
                record.FlattenStep = FlattenStep.Closing;
                record.StepStartedAt = _clock.UtcNow;
                var request = new OrderRequest
                {
                    Name = OrderName.Format(record.CommandId, ChildRole.Flatten),
                    Instrument = instrument,
                    Side = position > 0 ? Side.Sell : Side.Buy,
                    Type = OrderType.Market,
                    Quantity = Math.Abs(position),
                    TimeInForce = TimeInForce.Day,
                };
                // A re-try re-uses the name: the previous close order is terminal by then.
                SubmitFor(record, new[] { request }, null);
                if (record.Phase == CommandPhase.Done) Pump(account);
                return;
            }

            // Closing
            if (closeOrder == null) return;
            var flat = PositionQuantity(account, instrument) == 0;
            if (!closeOrder.Order.IsTerminal)
            {
                if (flat && !closeOrder.CancelRequested)
                {
                    // Flat already (e.g. a protective fill raced us): withdraw the close order.
                    closeOrder.CancelRequested = true;
                    try { _platform.Cancel(account, new[] { closeOrder.Handle }); }
                    catch (PlatformException e) { _log.Write(LogLevel.Warning, "platform.cancel_failed", record.CommandId + ": " + e.Message); }
                }
                return;
            }
            if (flat)
            {
                Finish(record, true, null, null, ChildRole.Flatten);
                Pump(account);
                return;
            }
            if (closeOrder.Order.State == PlatformOrderState.Rejected)
            {
                Finish(record, false, null, null, ChildRole.Flatten); // the lifecycle report carried the rejection
                Pump(account);
                return;
            }
            // Position changed while closing: start over (bounded).
            _orders.Remove(closeOrder.Handle);
            record.FlattenStep = FlattenStep.Cancelling;
            record.StepStartedAt = _clock.UtcNow;
            Save(record);
            AdvanceFlatten(record);
        }

        private void CheckFlattenTimeouts(DateTime now)
        {
            foreach (var record in _commands.Values.Where(r => r.Phase == CommandPhase.Dispatched && r.Command.Kind == CommandKind.Flatten
                && r.FlattenStep == FlattenStep.Cancelling && r.StepStartedAt.HasValue && now - r.StepStartedAt.Value > _options.FlattenCancelTimeout).ToList())
            {
                _log.Write(LogLevel.Warning, "flatten.cancel_timeout", record.CommandId);
                Finish(record, false, ReportStatus.Rejected, ReasonCodes.OrderCancelFailed, ChildRole.Flatten);
                Pump(record.PlatformAccount);
            }
        }

        // ------------------------------------------------------------------ final states and command reports

        /// <summary>Marks the command final. With <paramref name="status"/>, also reports it under its command_id.</summary>
        private void Finish(CommandRecord record, bool success, ReportStatus? status, string? reason, ChildRole? role = null)
        {
            record.Phase = CommandPhase.Done;
            record.Succeeded = success;
            record.DoneAt = _clock.UtcNow;
            if (status.HasValue)
            {
                var report = NewReport(record.PlatformAccount, record.Command.Instrument, record.AccountId);
                report.CommandId = record.CommandId;
                report.Origin = OrderOrigin.AllWin;
                report.ChildRole = role ?? DefaultRole(record.Command);
                report.Status = status.Value;
                report.ReasonCode = reason;
                if (record.Command.Kind == CommandKind.Place)
                {
                    report.Side = record.Command.Place!.Side;
                    report.Order = record.Command.Place.Order.Clone();
                    report.Quantity = record.Command.Place.Quantity;
                }
                Emit(record, report);
            }
            Save(record);
        }

        private void CommandReport(CommandRecord record, ReportStatus status, string? reason, bool withOrder)
        {
            var report = NewReport(record.PlatformAccount, record.Command.Instrument, record.AccountId);
            report.CommandId = record.CommandId;
            report.Origin = OrderOrigin.AllWin;
            report.ChildRole = DefaultRole(record.Command);
            report.Status = status;
            report.ReasonCode = reason;
            if (withOrder && record.Command.Kind == CommandKind.Place)
            {
                report.Side = record.Command.Place!.Side;
                report.Order = record.Command.Place.Order.Clone();
                report.Quantity = record.Command.Place.Quantity;
            }
            Emit(record, report);
        }

        private void ReportDuplicate(CommandRecord record)
        {
            var last = record.Last;
            var report = NewReport(record.PlatformAccount, record.Command.Instrument, record.AccountId);
            report.CommandId = record.CommandId;
            report.Origin = OrderOrigin.AllWin;
            report.Status = ReportStatus.Duplicate;
            report.ReasonCode = ReasonCodes.CommandDuplicate;
            report.ChildRole = last.ChildRole ?? DefaultRole(record.Command);
            report.PlatformOrderId = last.PlatformOrderId;
            report.Side = last.Side;
            report.Order = last.Order;
            report.Quantity = last.Quantity;
            // Not Emit(): a duplicate answer must not overwrite the latest real state.
            EnqueueReport(report);
            _log.Write(LogLevel.Info, "command.duplicate", record.CommandId);
        }

        private static ChildRole? DefaultRole(CommandPayload command)
        {
            switch (command.Kind)
            {
                case CommandKind.Place: return command.Place!.PositionEffect == PositionEffect.Open ? ChildRole.Entry : ChildRole.Exit;
                case CommandKind.Flatten: return ChildRole.Flatten;
                default:
                    var targets = command.Modify?.Targets ?? command.Cancel?.Targets;
                    return targets != null && targets.Count == 1 ? targets[0].ChildRole : (ChildRole?)null;
            }
        }

        private ExecutionReportPayload NewReport(string platformAccount, Instrument instrument, string? accountId = null)
        {
            return new ExecutionReportPayload
            {
                EventId = Ids.New(),
                AccountId = accountId ?? MappingOf(platformAccount)?.AccountId ?? "",
                PlatformAccount = platformAccount,
                Instrument = instrument.Clone(),
                OccurredAt = _clock.UtcNow,
            };
        }

        /// <summary>Queues a report and remembers it as the command's latest state (repeated by duplicates).</summary>
        private void Emit(CommandRecord? record, ExecutionReportPayload report)
        {
            if (record != null && report.CommandId == record.CommandId)
            {
                record.Last = new LastState
                {
                    Status = report.Status,
                    ReasonCode = report.ReasonCode,
                    ChildRole = report.ChildRole,
                    PlatformOrderId = report.PlatformOrderId,
                    Side = report.Side,
                    Order = report.Order,
                    Quantity = report.Quantity,
                };
                Save(record);
            }
            EnqueueReport(report);
        }

        private void EnqueueReport(ExecutionReportPayload report)
        {
            if (report.AccountId.Length == 0)
            {
                _log.Write(LogLevel.Warning, "report.unmapped_account", report.Status.ToString());
                return;
            }
            Enqueue(report);
        }

        private WelcomeAccount? MappingOf(string platformAccount) => _welcome?.Accounts.FirstOrDefault(a => a.PlatformAccount == platformAccount);

        private static string? NullIfEmpty(string? value) => string.IsNullOrEmpty(value) ? null : value;
    }
}
