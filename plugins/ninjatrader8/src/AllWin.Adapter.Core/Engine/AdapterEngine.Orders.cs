using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Engine
{
    // Platform events: order lifecycle, executions (add/amend/remove), positions, brackets and
    // orphan-order protection (vision §8, S9).
    public sealed partial class AdapterEngine
    {
        /// <summary>Entry point for events copied out of platform callbacks (ADR-0005 N6).</summary>
        public void OnPlatformEvent(PlatformEvent platformEvent)
        {
            Run("platform." + platformEvent.GetType().Name, () =>
            {
                if (!_started) return;
                switch (platformEvent)
                {
                    case AccountsChangedEvent _:
                        RefreshTrackedOrders();
                        RaiseChanged();
                        return;
                    case AccountConnectionEvent connection:
                        HandleConnection(connection);
                        return;
                }
                // Platform accounts the user has not mapped are ignored entirely (ADR-0004 §3 step 5).
                if (MappingOf(platformEvent.Account) == null) return;
                switch (platformEvent)
                {
                    case OrderUpdateEvent order: HandleOrderUpdate(platformEvent.Account, order.Order); break;
                    case ExecutionUpdateEvent execution: HandleExecution(platformEvent.Account, execution.Execution); break;
                    case PositionUpdateEvent position: HandlePosition(platformEvent.Account, position); break;
                    case AccountResetEvent _: HandleAccountReset(platformEvent.Account); break;
                }
            });
        }

        // ------------------------------------------------------------------ orders

        private void HandleOrderUpdate(string account, PlatformOrder order)
        {
            var tracked = Track(account, order);
            if (order.State == PlatformOrderState.ChangePending || order.State == PlatformOrderState.ChangeSubmitted) tracked.ChangeSeen = true;
            ReportLifecycle(tracked);

            if (tracked.CommandId != null && _commands.TryGetValue(tracked.CommandId, out var record)
                && record.Command.Kind == CommandKind.Place && (tracked.Letter == 'e' || tracked.Letter == 'x')
                && order.IsTerminal && record.Phase != CommandPhase.Done)
            {
                record.EntryTerminal = true;
                var success = order.State == PlatformOrderState.Filled || (order.State == PlatformOrderState.Cancelled && order.FilledQuantity > 0);
                Finish(record, success, null, null);
                Pump(account);
            }

            EvaluateTargets(tracked);
            var flatten = ActiveFlatten(account, order.Instrument);
            if (flatten != null) AdvanceFlatten(flatten);
        }

        private TrackedOrder Track(string account, PlatformOrder order)
        {
            if (_orders.TryGetValue(order.Handle, out var tracked))
            {
                tracked.Order = order.Clone();
                return tracked;
            }
            tracked = new TrackedOrder(order.Handle, account, order.Clone());
            if (OrderName.TryParse(order.Name, out var commandId, out var letter))
            {
                tracked.CommandId = commandId;
                tracked.Letter = letter;
                tracked.Origin = OrderOrigin.AllWin;
            }
            _orders[order.Handle] = tracked;
            return tracked;
        }

        private void TrackSubmitted(string account, string handle, OrderRequest request, string? workingReason)
        {
            var order = new PlatformOrder
            {
                Handle = handle,
                Name = request.Name,
                Instrument = request.Instrument,
                Side = request.Side,
                Type = request.Type,
                Quantity = request.Quantity,
                LimitPrice = request.LimitPrice,
                StopPrice = request.StopPrice,
                TimeInForce = request.TimeInForce,
                State = PlatformOrderState.Initialized,
                Time = _clock.UtcNow,
            };
            var tracked = Track(account, order);
            tracked.WorkingReason = workingReason;
        }

        /// <summary>Reports a lifecycle change once (a signature per order survives restarts for All Win orders).</summary>
        private void ReportLifecycle(TrackedOrder tracked)
        {
            var order = tracked.Order;
            ReportStatus status;
            switch (order.State)
            {
                case PlatformOrderState.Accepted:
                case PlatformOrderState.Working:
                case PlatformOrderState.TriggerPending: status = ReportStatus.Working; break;
                case PlatformOrderState.Cancelled: status = ReportStatus.Cancelled; break;
                case PlatformOrderState.Rejected: status = ReportStatus.Rejected; break;
                case PlatformOrderState.Unknown: status = ReportStatus.Unknown; break;
                default: return; // transitional states; fills come from executions
            }

            var signature = string.Join("|", status, order.Quantity.ToString(CultureInfo.InvariantCulture),
                order.LimitPrice.HasValue ? DecimalText.Format(order.LimitPrice.Value) : "", order.StopPrice.HasValue ? DecimalText.Format(order.StopPrice.Value) : "");
            CommandRecord? record = null;
            if (tracked.CommandId != null) _commands.TryGetValue(tracked.CommandId, out record);
            var letterKey = tracked.Letter.ToString();
            var previous = record != null ? (record.Reported.TryGetValue(letterKey, out var s) ? s : null) : tracked.LastSignature;
            if (previous == signature) return;
            if (record != null) record.Reported[letterKey] = signature;
            else tracked.LastSignature = signature;

            var report = NewReport(tracked.Account, order.Instrument);
            report.CommandId = tracked.CommandId;
            report.Origin = tracked.Origin;
            report.ChildRole = tracked.CommandId != null ? RoleOf(tracked) : (ChildRole?)null;
            report.PlatformOrderId = NullIfEmpty(order.PlatformOrderId);
            report.Status = status;
            report.Side = order.Side;
            report.Order = SpecOf(order);
            report.Quantity = order.Quantity;
            report.PlatformTime = order.Time == default ? (DateTime?)null : order.Time;
            switch (status)
            {
                case ReportStatus.Rejected:
                    report.ReasonCode = tracked.IsProtective ? ReasonCodes.OrderProtectionFailed : ReasonCodes.OrderPlatformRejected;
                    break;
                case ReportStatus.Cancelled:
                    report.ReasonCode = tracked.CancelReason;
                    break;
                case ReportStatus.Working:
                    report.ReasonCode = tracked.WorkingReason;
                    tracked.WorkingReason = null;
                    break;
            }
            Emit(record, report);
        }

        // ------------------------------------------------------------------ executions

        private void HandleExecution(string account, PlatformExecution execution)
        {
            var key = account + "|" + execution.ExecutionId;
            _fills.TryGetValue(key, out var existing);
            var live = existing != null && !existing.Removed ? existing : null;
            var operation = execution.Operation;
            if (operation == FillOperation.Add && existing != null)
            {
                if (live != null && live.Quantity == execution.Quantity && live.Price == execution.Price) return; // replay after restart
                operation = FillOperation.Amend;
            }
            if (operation == FillOperation.Amend)
            {
                if (live != null && live.Quantity == execution.Quantity && live.Price == execution.Price) return;
                if (existing == null) operation = FillOperation.Add; // never saw the original: it is new to the Core too
            }
            if (operation == FillOperation.Remove && live == null)
            {
                _log.Write(LogLevel.Info, "fill.remove_unknown", execution.ExecutionId);
                return;
            }

            var tracked = FindOrderForExecution(account, execution);
            string? commandId = tracked?.CommandId;
            var letter = tracked?.Letter ?? '\0';
            if (tracked == null && OrderName.TryParse(execution.OrderName, out var parsedId, out var parsedLetter))
            {
                commandId = parsedId;
                letter = parsedLetter;
            }

            var oldQuantity = live?.Quantity ?? 0;
            var oldPrice = live?.Price ?? 0m;
            int deltaQuantity;
            decimal deltaNotional;
            switch (operation)
            {
                case FillOperation.Remove:
                    deltaQuantity = -oldQuantity;
                    deltaNotional = -oldQuantity * oldPrice;
                    break;
                case FillOperation.Amend:
                    deltaQuantity = execution.Quantity - oldQuantity;
                    deltaNotional = execution.Quantity * execution.Price - oldQuantity * oldPrice;
                    break;
                default:
                    deltaQuantity = execution.Quantity;
                    deltaNotional = execution.Quantity * execution.Price;
                    break;
            }
            if (tracked != null) tracked.ExecFilled += deltaQuantity;

            CommandRecord? record = null;
            if (commandId != null) _commands.TryGetValue(commandId, out record);
            if (record != null && letter == 'e' && record.Command.Kind == CommandKind.Place)
            {
                record.EntryFilled += deltaQuantity;
                record.EntryNotional += deltaNotional;
                Save(record);
            }

            var position = execution.PositionAfter ?? CurrentPosition(account, execution.Instrument);
            var report = NewReport(account, execution.Instrument);
            report.CommandId = commandId;
            report.Origin = commandId != null ? OrderOrigin.AllWin : OrderOrigin.External;
            report.ChildRole = commandId != null ? OrderName.RoleOf(letter, record?.Command.Kind == CommandKind.Flatten) : (ChildRole?)null;
            report.PlatformOrderId = NullIfEmpty(tracked?.Order.PlatformOrderId ?? execution.PlatformOrderId);
            // Answer 10: the label is recomputed after an amend/remove, even when the filled quantity drops to 0.
            report.Status = tracked == null || (tracked.ExecFilled > 0 && tracked.ExecFilled >= tracked.Order.Quantity)
                ? ReportStatus.Filled : ReportStatus.PartiallyFilled;
            report.Side = execution.Side;
            if (tracked != null)
            {
                report.Order = SpecOf(tracked.Order);
                report.Quantity = tracked.Order.Quantity;
            }
            report.Fill = new FillInfo
            {
                FillId = execution.ExecutionId,
                Quantity = operation == FillOperation.Remove ? oldQuantity : execution.Quantity,
                Price = operation == FillOperation.Remove ? oldPrice : execution.Price,
                Commission = execution.Commission,
                Operation = operation == FillOperation.Add ? (FillOperation?)null : operation,
            };
            report.PositionAfter = new PositionInfo
            {
                Quantity = position?.Quantity ?? 0,
                AveragePrice = position != null && position.Quantity != 0 && position.AveragePrice > 0 ? position.AveragePrice : null,
            };
            report.PlatformTime = execution.PlatformTime ?? (execution.Time == default ? (DateTime?)null : execution.Time);

            var fillRecord = new FillRecord
            {
                FillId = execution.ExecutionId,
                Quantity = operation == FillOperation.Remove ? oldQuantity : execution.Quantity,
                Price = operation == FillOperation.Remove ? oldPrice : execution.Price,
                Removed = operation == FillOperation.Remove,
                CommandId = commandId,
                RoleLetter = letter == '\0' ? null : letter.ToString(),
                RecordedAt = _clock.UtcNow,
            };
            _fills[key] = fillRecord;
            Put(Collections.Fills, key, fillRecord);
            Emit(record, report);

            if (record != null && (letter == 'e' || letter == 's' || letter == 't')) SyncBracket(record);
            var flatten = ActiveFlatten(account, execution.Instrument);
            if (flatten != null) AdvanceFlatten(flatten);
        }

        private TrackedOrder? FindOrderForExecution(string account, PlatformExecution execution)
        {
            if (execution.OrderHandle != null && _orders.TryGetValue(execution.OrderHandle, out var byHandle)) return byHandle;
            if (string.IsNullOrEmpty(execution.OrderName)) return null;
            return _orders.Values.Where(o => o.Account == account && o.Order.Name == execution.OrderName)
                .OrderBy(o => o.Order.IsTerminal).FirstOrDefault();
        }

        // ------------------------------------------------------------------ brackets (ADR-0005 N1)

        /// <summary>Submits, resizes or re-prices the stop/target that protect <paramref name="record"/>'s position.</summary>
        private void SyncBracket(CommandRecord record)
        {
            var protection = record.Command.Place?.Protection;
            if (record.Command.Kind != CommandKind.Place || protection == null) return;
            if (protection.Mode == ProtectionMode.Shared) SyncSharedBracket(record);
            else SyncIndividualBracket(record);
        }

        private void SyncIndividualBracket(CommandRecord record)
        {
            if (record.EntryFilled <= 0) return;
            var account = record.PlatformAccount;
            var legsFilled = LegFilled(account, record.CommandId, 's') + LegFilled(account, record.CommandId, 't');
            var open = record.EntryFilled - legsFilled;
            // Answer 7: prices from the VWAP of this entry order, re-priced on every partial fill.
            var basePrice = TickMath.Vwap(record.EntryNotional, record.EntryFilled);
            ApplyBracket(record, record.Command.Place!.Side, basePrice, open);
        }

        /// <summary>
        /// Shared mode (answer 1): the command that opened the position owns one stop/target sized to the
        /// whole position and priced from its average; adds resize and re-price it with the owner's ticks.
        /// </summary>
        private void SyncSharedBracket(CommandRecord candidate)
        {
            var account = candidate.PlatformAccount;
            var instrument = candidate.Command.Instrument;
            var key = OwnerKey(account, instrument);
            CommandRecord? owner = null;
            if (_owners.TryGetValue(key, out var ownerId)) _commands.TryGetValue(ownerId, out owner);
            if (owner == null)
            {
                if (candidate.EntryFilled <= 0) return;
                owner = candidate;
                _owners[key] = owner.CommandId;
                Put(Collections.Owners, key, owner.CommandId);
            }

            var position = CurrentPosition(account, instrument);
            if (position == null || position.Quantity == 0) return; // flat: orphan protection handles it
            var ownerSide = owner.Command.Place!.Side;
            if ((ownerSide == Side.Buy) != (position.Quantity > 0)) return; // flipped (double fill): alert and wait, the Core decides
            var basePrice = position.AveragePrice ?? TickMath.Vwap(owner.EntryNotional, Math.Max(owner.EntryFilled, 1));
            ApplyBracket(owner, ownerSide, basePrice, Math.Abs(position.Quantity));
        }

        private void ApplyBracket(CommandRecord owner, Side entrySide, decimal basePrice, int openQuantity)
        {
            var account = owner.PlatformAccount;
            var instrument = owner.Command.Instrument;
            var protection = owner.Command.Place!.Protection!;
            var tick = _platform.GetTickSize(instrument);
            if (tick == null)
            {
                _log.Write(LogLevel.Error, "bracket.no_tick_size", owner.CommandId);
                return;
            }
            var (stopPrice, targetPrice) = TickMath.Protective(entrySide, basePrice, tick.Value, protection.StopTicks, protection.TargetTicks);
            var exitSide = entrySide == Side.Buy ? Side.Sell : Side.Buy;

            if (!owner.BracketSubmitted)
            {
                if (openQuantity <= 0) return;
                var oco = stopPrice.HasValue && targetPrice.HasValue ? OrderName.Prefix + owner.CommandId : null;
                var requests = new List<OrderRequest>();
                if (stopPrice.HasValue)
                    requests.Add(ProtectiveRequest(owner.CommandId, ChildRole.Stop, instrument, exitSide, OrderType.Stop, openQuantity, null, stopPrice, oco));
                if (targetPrice.HasValue)
                    requests.Add(ProtectiveRequest(owner.CommandId, ChildRole.Target, instrument, exitSide, OrderType.Limit, openQuantity, targetPrice, null, oco));
                owner.BracketSubmitted = true;
                Save(owner);
                Flush();
                try
                {
                    var handles = GuardedSubmit(account, requests);
                    for (var i = 0; i < requests.Count && i < handles.Count; i++) TrackSubmitted(account, handles[i], requests[i], null);
                }
                catch (PlatformException e)
                {
                    _log.Write(LogLevel.Error, "bracket.submit_failed", owner.CommandId + ": " + e.Message);
                    foreach (var request in requests)
                    {
                        var report = NewReport(account, instrument);
                        report.CommandId = owner.CommandId;
                        report.Origin = OrderOrigin.AllWin;
                        report.ChildRole = OrderName.RoleOf(request.Name[request.Name.Length - 1], false);
                        report.Status = ReportStatus.Rejected;
                        report.ReasonCode = ReasonCodes.OrderProtectionFailed;
                        report.Side = request.Side;
                        report.Order = new OrderSpec { Type = request.Type, LimitPrice = request.LimitPrice, StopPrice = request.StopPrice, TimeInForce = request.TimeInForce };
                        report.Quantity = request.Quantity;
                        Emit(owner, report);
                    }
                }
                return;
            }

            foreach (var letter in new[] { 's', 't' })
            {
                var leg = FindOrder(account, owner.CommandId, letter);
                if (leg == null || leg.Order.IsTerminal || leg.CancelRequested) continue;
                if (openQuantity <= 0)
                {
                    // AC-2 individual mode: the entry is fully closed by the other leg; cancel this one.
                    CancelOrphans(account, new[] { leg });
                    continue;
                }
                var desired = leg.ExecFilled + openQuantity;
                var price = letter == 's' ? stopPrice : targetPrice;
                var current = letter == 's' ? leg.Order.StopPrice : leg.Order.LimitPrice;
                int? newQuantity = desired != leg.Order.Quantity ? desired : (int?)null;
                decimal? newPrice = price.HasValue && price != current ? price : null;
                if (!newQuantity.HasValue && !newPrice.HasValue) continue;
                try
                {
                    // One change for quantity and price together (answer 7).
                    GuardedChange(account, leg.Handle, newQuantity, letter == 't' ? newPrice : null, letter == 's' ? newPrice : null);
                }
                catch (PlatformException e)
                {
                    _log.Write(LogLevel.Error, "bracket.change_failed", owner.CommandId + ": " + e.Message);
                    ReportOrderIssue(leg, ReasonCodes.OrderProtectionFailed);
                }
            }
        }

        private static OrderRequest ProtectiveRequest(string ownerId, ChildRole role, Instrument instrument, Side side, OrderType type, int quantity,
            decimal? limit, decimal? stop, string? oco)
        {
            return new OrderRequest
            {
                Name = OrderName.Format(ownerId, role),
                Instrument = instrument,
                Side = side,
                Type = type,
                Quantity = quantity,
                LimitPrice = limit,
                StopPrice = stop,
                TimeInForce = TimeInForce.Gtc, // a DAY protective order would drop protection at the session end
                OcoId = oco,
            };
        }

        private int LegFilled(string account, string commandId, char letter) =>
            _orders.Values.Where(o => o.Account == account && o.CommandId == commandId && o.Letter == letter).Sum(o => o.ExecFilled);

        /// <summary>A working order kept its old values (e.g. a failed resize): reported as is, with the reason.</summary>
        private void ReportOrderIssue(TrackedOrder order, string reason)
        {
            CommandRecord? record = null;
            if (order.CommandId != null) _commands.TryGetValue(order.CommandId, out record);
            var report = NewReport(order.Account, order.Order.Instrument);
            report.CommandId = order.CommandId;
            report.Origin = order.Origin;
            report.ChildRole = order.CommandId != null ? RoleOf(order) : (ChildRole?)null;
            report.PlatformOrderId = NullIfEmpty(order.Order.PlatformOrderId);
            report.Status = ReportStatus.Working;
            report.ReasonCode = reason;
            report.Side = order.Order.Side;
            report.Order = SpecOf(order.Order);
            report.Quantity = order.Order.Quantity;
            Emit(record, report);
        }

        // ------------------------------------------------------------------ positions and orphan protection

        private void HandlePosition(string account, PositionUpdateEvent update)
        {
            var instrument = update.Position.Instrument;
            if (update.IsFlat)
            {
                var key = OwnerKey(account, instrument);
                if (_owners.Remove(key)) _pending.Add(new Store.StoreOp(Collections.Owners, key, null)); // ownership ends when flat
                CancelOrphans(account, LiveOrders(account, instrument).Where(o => o.IsProtective).ToList());
            }
            var flatten = ActiveFlatten(account, instrument);
            if (flatten != null) AdvanceFlatten(flatten);
        }

        /// <summary>Whenever a position becomes flat, remaining All Win stop/target orders are cancelled (vision §8).</summary>
        private void CancelOrphans(string account, IReadOnlyList<TrackedOrder> legs)
        {
            var toCancel = legs.Where(o => !o.CancelRequested && !o.Order.IsTerminal).ToList();
            if (toCancel.Count == 0) return;
            foreach (var leg in toCancel)
            {
                leg.CancelRequested = true;
                leg.CancelReason = ReasonCodes.OrderOrphanCancelled;
            }
            _log.Write(LogLevel.Info, "orphan.cancel", account + " orders=" + toCancel.Count.ToString(CultureInfo.InvariantCulture));
            try
            {
                GuardedCancel(account, toCancel.Select(o => o.Handle).ToList());
            }
            catch (PlatformException e)
            {
                // S9 AC-3: a failed cancellation is reported; the Core flags the account out of sync.
                _log.Write(LogLevel.Error, "orphan.cancel_failed", e.Message);
                foreach (var leg in toCancel) ReportOrderIssue(leg, ReasonCodes.OrderCancelFailed);
            }
        }

        private void HandleConnection(AccountConnectionEvent connection)
        {
            var mapped = MappingOf(connection.Account) != null;
            if (mapped)
            {
                if (Session == SessionState.Ready) EnqueueSnapshot(SnapshotTrigger.Change, connection.Account);
                // "Same treatment when it returns" (S11 AC-5 iv): reconcile the account with the platform.
                if (connection.Connected)
                {
                    RefreshTrackedOrders(connection.Account);
                    EvaluateIntegrity(connection.Account);
                }
            }
            RaiseChanged();
        }

        private void HandleAccountReset(string account)
        {
            // A discontinuity, never fills (S11 AC-6): forget local order state and re-read the platform.
            AddStickyReason(account, ReasonCodes.SyncSimulationReset);
            foreach (var key in _owners.Keys.Where(k => k.StartsWith(account + "|", StringComparison.Ordinal)).ToList())
            {
                _owners.Remove(key);
                _pending.Add(new Store.StoreOp(Collections.Owners, key, null));
            }
            foreach (var handle in _orders.Values.Where(o => o.Account == account).Select(o => o.Handle).ToList()) _orders.Remove(handle);
            foreach (var execution in SafeExecutions(account))
            {
                var key = account + "|" + execution.ExecutionId;
                if (_fills.ContainsKey(key)) continue;
                // Executions that exist after the reset are platform history, not new fills.
                var record = new FillRecord { FillId = execution.ExecutionId, Quantity = execution.Quantity, Price = execution.Price, RecordedAt = _clock.UtcNow };
                _fills[key] = record;
                Put(Collections.Fills, key, record);
            }
            foreach (var order in SafeOrders(account))
            {
                var tracked = Track(account, order);
                tracked.ExecFilled = order.FilledQuantity;
                if (tracked.CommandId == null) tracked.LastSignature = "reset";
            }
            EnqueueSnapshot(SnapshotTrigger.AccountReset, account);
            RaiseChanged();
        }

        // ------------------------------------------------------------------ guarded platform writes

        /// <summary>
        /// Every order action passes the simulation guard at the moment it is made (ADR-0005 N4), not only
        /// when the command was accepted: automatic actions (orphan cancels, bracket resizes, flatten steps)
        /// included. A refused account gets no order activity at all.
        /// </summary>
        private void EnsureWritable(string account)
        {
            var info = SafeAccounts().FirstOrDefault(a => a.Name == account);
            if (info == null || !IsExecutionAllowed(info))
            {
                _log.Write(LogLevel.Error, "guard.refused", account);
                throw new PlatformException("account refused by the simulation guard");
            }
        }

        private IReadOnlyList<string> GuardedSubmit(string account, IReadOnlyList<OrderRequest> orders)
        {
            EnsureWritable(account);
            return _platform.Submit(account, orders);
        }

        private void GuardedChange(string account, string handle, int? quantity, decimal? limit, decimal? stop)
        {
            EnsureWritable(account);
            _platform.Change(account, handle, quantity, limit, stop);
        }

        private void GuardedCancel(string account, IReadOnlyList<string> handles)
        {
            EnsureWritable(account);
            _platform.Cancel(account, handles);
        }

        // ------------------------------------------------------------------ lookups

        private TrackedOrder? FindOrder(string account, string commandId, char letter)
        {
            return _orders.Values.Where(o => o.Account == account && o.CommandId == commandId && o.Letter == letter)
                .OrderBy(o => o.Order.IsTerminal).ThenByDescending(o => o.Order.Time).FirstOrDefault();
        }

        private TrackedOrder? FindLiveOrder(string account, string commandId, ChildRole role)
        {
            var order = FindOrder(account, commandId, OrderName.Letter(role));
            return order == null || order.Order.IsTerminal ? null : order;
        }

        private List<TrackedOrder> LiveOrders(string account, Instrument instrument) =>
            _orders.Values.Where(o => o.Account == account && o.Order.Instrument.Equals(instrument) && !o.Order.IsTerminal).ToList();

        private PlatformPosition? CurrentPosition(string account, Instrument instrument) =>
            _platform.GetPositions(account).FirstOrDefault(p => p.Instrument.Equals(instrument));

        private int PositionQuantity(string account, Instrument instrument) => CurrentPosition(account, instrument)?.Quantity ?? 0;

        private ChildRole RoleOf(TrackedOrder order)
        {
            var isFlatten = order.CommandId != null && _commands.TryGetValue(order.CommandId, out var record) && record.Command.Kind == CommandKind.Flatten;
            return OrderName.RoleOf(order.Letter, isFlatten);
        }

        private static bool IsWorkingState(PlatformOrderState state) =>
            state == PlatformOrderState.Accepted || state == PlatformOrderState.Working || state == PlatformOrderState.TriggerPending;

        private static OrderSpec SpecOf(PlatformOrder order)
        {
            var spec = new OrderSpec { Type = order.Type, TimeInForce = order.TimeInForce };
            if (order.Type == OrderType.Limit || order.Type == OrderType.StopLimit) spec.LimitPrice = order.LimitPrice;
            if (order.Type == OrderType.Stop || order.Type == OrderType.StopLimit) spec.StopPrice = order.StopPrice;
            return spec;
        }

        private static string OwnerKey(string account, Instrument instrument) => account + "|" + instrument.Symbol + "|" + instrument.ContractMonth;

        private IReadOnlyList<PlatformOrder> SafeOrders(string account)
        {
            try { return _platform.GetOrders(account); }
            catch (PlatformException e)
            {
                _log.Write(LogLevel.Error, "platform.orders_failed", e.Message);
                return new PlatformOrder[0];
            }
        }

        private IReadOnlyList<PlatformExecution> SafeExecutions(string account)
        {
            try { return _platform.GetExecutions(account); }
            catch (PlatformException e)
            {
                _log.Write(LogLevel.Error, "platform.executions_failed", e.Message);
                return new PlatformExecution[0];
            }
        }

        /// <summary>In-memory view of one platform order; rebuilt from <c>Order.Name</c> after a restart.</summary>
        private sealed class TrackedOrder
        {
            public TrackedOrder(string handle, string account, PlatformOrder order)
            {
                Handle = handle;
                Account = account;
                Order = order;
            }

            public string Handle { get; }
            public string Account { get; }
            public PlatformOrder Order { get; set; }
            public string? CommandId { get; set; }
            public char Letter { get; set; }
            public OrderOrigin Origin { get; set; } = OrderOrigin.External;

            /// <summary>Filled quantity from executions (event order between OrderUpdate and ExecutionUpdate is not guaranteed).</summary>
            public int ExecFilled { get; set; }

            public bool CancelRequested { get; set; }
            public string? CancelReason { get; set; }
            public bool ChangeSeen { get; set; }
            public string? WorkingReason { get; set; }
            public string? LastSignature { get; set; }

            public bool IsProtective => CommandId != null && (Letter == 's' || Letter == 't');
        }
    }
}
