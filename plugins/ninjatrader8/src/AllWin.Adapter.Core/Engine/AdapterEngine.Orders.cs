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
                // Registry §3: a place succeeds for depends_on only when its order is filled.
                var success = order.State == PlatformOrderState.Filled;
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

            // Every fill can change what the brackets must cover: entries, protective legs, exits, external fills.
            ReconcileProtection(account, execution.Instrument, position, letter == 'e' ? record : null);
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

        /// <summary>
        /// Keeps every bracket on the contract consistent with the position after a fill. Invariant: the
        /// protective quantity never exceeds the position it protects (answer B), so a stop or target can never
        /// open or flip a position. Individual mode reduces protection oldest entry first (P2, FIFO); shared mode
        /// sizes the owner's bracket to the position. Legs left with nothing to protect, or on the wrong side
        /// after a flip, are cancelled with order.orphan_cancelled. <paramref name="position"/> must be the
        /// position right after the fill being processed (execution PositionAfter), not a later one.
        /// </summary>
        private void ReconcileProtection(string account, Instrument instrument, PlatformPosition? position, CommandRecord? filledEntry)
        {
            var quantity = position?.Quantity ?? 0;
            if (quantity == 0) return; // flat: the orphan rule (HandlePosition) cancels every leg

            var key = OwnerKey(account, instrument);
            CommandRecord? owner = null;
            if (_owners.TryGetValue(key, out var ownerId)) _commands.TryGetValue(ownerId, out owner);
            if (owner == null && filledEntry?.Command.Place?.Protection?.Mode == ProtectionMode.Shared && filledEntry.EntryFilled > 0)
            {
                owner = filledEntry; // answer 1: the command that opened the position owns the shared bracket
                _owners[key] = owner.CommandId;
                Put(Collections.Owners, key, owner.CommandId);
            }
            if (owner != null)
            {
                var ownerSide = owner.Command.Place!.Side;
                var covered = (ownerSide == Side.Buy) == (quantity > 0) ? Math.Abs(quantity) : 0;
                var basePrice = position!.AveragePrice ?? TickMath.Vwap(owner.EntryNotional, Math.Max(owner.EntryFilled, 1));
                ApplyBracket(owner, ownerSide, basePrice, covered);
            }

            foreach (var allocation in IndividualAllocations(account, instrument, quantity))
            {
                var record = allocation.Key;
                // Answer 7: prices from the VWAP of this entry order.
                ApplyBracket(record, record.Command.Place!.Side, TickMath.Vwap(record.EntryNotional, record.EntryFilled), allocation.Value);
            }
        }

        /// <summary>
        /// How much of the position each individual bracket may still protect. Protection is taken away from the
        /// oldest entries first (FIFO), i.e. the newest entries keep theirs; brackets on the other side get 0.
        /// </summary>
        private List<KeyValuePair<CommandRecord, int>> IndividualAllocations(string account, Instrument instrument, int positionQuantity)
        {
            var result = new List<KeyValuePair<CommandRecord, int>>();
            var brackets = _commands.Values.Where(r => r.PlatformAccount == account && r.Command.Kind == CommandKind.Place
                && r.Command.Place!.Protection?.Mode == ProtectionMode.Individual && r.Command.Instrument.Equals(instrument) && r.EntryFilled > 0)
                .OrderByDescending(r => r.Seq).ToList();
            foreach (var side in new[] { Side.Buy, Side.Sell })
            {
                var remaining = side == Side.Buy ? Math.Max(positionQuantity, 0) : Math.Max(-positionQuantity, 0);
                foreach (var record in brackets.Where(r => r.Command.Place!.Side == side))
                {
                    var open = Math.Max(0, record.EntryFilled - LegFilled(account, record.CommandId, 's') - LegFilled(account, record.CommandId, 't'));
                    var allocated = Math.Min(open, remaining);
                    remaining -= allocated;
                    result.Add(new KeyValuePair<CommandRecord, int>(record, allocated));
                }
            }
            return result;
        }

        /// <summary>The largest total quantity a protective leg may have now (filled part included).</summary>
        private int AllowedLegQuantity(TrackedOrder leg)
        {
            if (leg.CommandId == null || !_commands.TryGetValue(leg.CommandId, out var record) || record.Command.Place?.Protection == null)
                return leg.Order.Quantity;
            var position = PositionQuantity(leg.Account, leg.Order.Instrument);
            int covered;
            if (record.Command.Place.Protection.Mode == ProtectionMode.Shared)
                covered = (record.Command.Place.Side == Side.Buy) == (position > 0) ? Math.Abs(position) : 0;
            else
                covered = IndividualAllocations(leg.Account, leg.Order.Instrument, position).Where(a => a.Key == record).Select(a => a.Value).FirstOrDefault();
            return leg.ExecFilled + covered;
        }

        /// <summary>
        /// Submits the bracket, or resizes/re-prices its legs to <paramref name="covered"/>. During the restart
        /// catch-up (answer A) nothing is submitted late and legs are only resized down: anything else is left
        /// to the user as sync.bracket_incomplete (alert and wait).
        /// </summary>
        private void ApplyBracket(CommandRecord owner, Side entrySide, decimal basePrice, int covered)
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
                if (covered <= 0) return;
                if (owner.ProtectionWithheld) return; // decided once: alert and wait, never submitted later
                var oco = stopPrice.HasValue && targetPrice.HasValue ? OrderName.Prefix + owner.CommandId : null;
                var requests = new List<OrderRequest>();
                if (stopPrice.HasValue)
                    requests.Add(ProtectiveRequest(owner.CommandId, ChildRole.Stop, instrument, exitSide, OrderType.Stop, covered, null, stopPrice, oco));
                if (targetPrice.HasValue)
                    requests.Add(ProtectiveRequest(owner.CommandId, ChildRole.Target, instrument, exitSide, OrderType.Limit, covered, targetPrice, null, oco));
                if (_catchUp)
                {
                    // Answer A: no late bracket after a catch-up fill. Each withheld leg is reported; the Core
                    // raises sync.bracket_incomplete (sync.* codes are Core-only; the local hint is display only).
                    _log.Write(LogLevel.Warning, "bracket.not_submitted_late", owner.CommandId);
                    owner.ProtectionWithheld = true;
                    Save(owner);
                    foreach (var request in requests) ReportLeg(owner, request, ReasonCodes.OrderProtectionNotSubmitted);
                    AddStickyReason(account, ReasonCodes.SyncBracketIncomplete);
                    return;
                }
                owner.BracketSubmitted = true;
                Save(owner);
                Flush();
                // Answer G: a live leg that already exists under its exact reference is adopted, never submitted
                // again; an ambiguous reference submits nothing.
                var toSubmit = new List<OrderRequest>();
                foreach (var request in requests)
                {
                    switch (AdoptExisting(account, request.Name))
                    {
                        case Adoption.None: toSubmit.Add(request); break;
                        case Adoption.Ambiguous: ReportLeg(owner, request, ReasonCodes.OrderAmbiguousReference); break;
                    }
                }
                if (toSubmit.Count == 0) return;
                try
                {
                    var handles = GuardedSubmit(account, toSubmit);
                    for (var i = 0; i < toSubmit.Count && i < handles.Count; i++) TrackSubmitted(account, handles[i], toSubmit[i], null);
                }
                catch (PlatformException e)
                {
                    _log.Write(LogLevel.Error, "bracket.submit_failed", owner.CommandId + ": " + e.GetType().Name + " " + e.Message);
                    foreach (var request in toSubmit)
                    {
                        var adoption = e is PlatformOutcomeUnknownException ? AdoptExisting(account, request.Name) : Adoption.None;
                        if (adoption == Adoption.Adopted) continue;
                        ReportLeg(owner, request, adoption == Adoption.Ambiguous ? ReasonCodes.OrderAmbiguousReference : ReasonCodes.OrderProtectionFailed);
                    }
                }
                return;
            }

            foreach (var letter in new[] { 's', 't' })
            {
                var leg = FindOrder(account, owner.CommandId, letter);
                if (leg == null || leg.Order.IsTerminal || leg.CancelRequested) continue;
                if (covered <= 0)
                {
                    // Nothing left to protect on this side (fully reduced, or flipped): cancel (answer B).
                    CancelOrphans(account, new[] { leg });
                    continue;
                }
                var desired = leg.ExecFilled + covered;
                var price = letter == 's' ? stopPrice : targetPrice;
                var current = letter == 's' ? leg.Order.StopPrice : leg.Order.LimitPrice;
                int? newQuantity = desired != leg.Order.Quantity ? desired : (int?)null;
                decimal? newPrice = price.HasValue && price != current ? price : null;
                if (_catchUp)
                {
                    if (newQuantity > leg.Order.Quantity)
                    {
                        // The position grew while we were down: do not extend protection on our own.
                        AddStickyReason(account, ReasonCodes.SyncBracketIncomplete);
                        newQuantity = null;
                    }
                    newPrice = null;
                }
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

        /// <summary>A leg that does not exist: its submission failed (answer C), was withheld (A) or was ambiguous.</summary>
        private void ReportLeg(CommandRecord owner, OrderRequest request, string reason)
        {
            var report = NewReport(owner.PlatformAccount, owner.Command.Instrument);
            report.CommandId = owner.CommandId;
            report.Origin = OrderOrigin.AllWin;
            report.ChildRole = OrderName.RoleOf(request.Name[request.Name.Length - 1], false);
            report.Status = ReportStatus.Rejected;
            report.ReasonCode = reason;
            report.Side = request.Side;
            report.Order = new OrderSpec { Type = request.Type, LimitPrice = request.LimitPrice, StopPrice = request.StopPrice, TimeInForce = request.TimeInForce };
            report.Quantity = request.Quantity;
            Emit(owner, report);
        }

        private enum Adoption { None, Adopted, Ambiguous }

        /// <summary>
        /// Adopts the platform order carrying the exact reference <paramref name="name"/> (command_id + role) as
        /// the submission and reports its last state; it is never submitted again (answer G as corrected). For
        /// place orders and protective legs any match counts, live or terminal, which covers submit → immediate
        /// fill → crash before "submitted" was persisted. Only a flatten close (<paramref name="liveOnly"/>)
        /// ignores terminal matches: its re-tries re-use aw:&lt;cmd&gt;:x and the position is re-evaluated first.
        /// Two or more adoptable matches are ambiguous: none is adopted and nothing is submitted.
        /// </summary>
        private Adoption AdoptExisting(string account, string name, bool liveOnly = false)
        {
            var matches = SafeOrders(account).Where(o => o.Name == name && (!liveOnly || !o.IsTerminal)).ToList();
            if (matches.Count == 0) return Adoption.None;
            if (matches.Count > 1)
            {
                _log.Write(LogLevel.Error, "order.ambiguous_reference", name + " count=" + matches.Count.ToString(CultureInfo.InvariantCulture));
                return Adoption.Ambiguous;
            }
            _log.Write(LogLevel.Warning, "order.adopted", name + " state=" + matches[0].State);
            AdoptPlatformOrder(account, matches[0]);
            return Adoption.Adopted;
        }

        private void AdoptPlatformOrder(string account, PlatformOrder order)
        {
            if (!_orders.ContainsKey(order.Handle))
            {
                var tracked = Track(account, order);
                // Fills already reported are not replayed; start from what the store knows for our orders.
                tracked.ExecFilled = tracked.CommandId != null
                    ? _fills.Values.Where(f => !f.Removed && f.CommandId == tracked.CommandId && f.RoleLetter == tracked.Letter.ToString()).Sum(f => f.Quantity)
                    : order.FilledQuantity;
            }
            HandleOrderUpdate(account, order);
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

        /// <summary>
        /// An operation on a working order failed (resize, re-price, cancel): rejected + reason under the owning
        /// reference. The order keeps its last lifecycle state; the Core does not read this as "order gone" (answer C).
        /// </summary>
        private void ReportOrderIssue(TrackedOrder order, string reason)
        {
            CommandRecord? record = null;
            if (order.CommandId != null) _commands.TryGetValue(order.CommandId, out record);
            var report = NewReport(order.Account, order.Order.Instrument);
            report.CommandId = order.CommandId;
            report.Origin = order.Origin;
            report.ChildRole = order.CommandId != null ? RoleOf(order) : (ChildRole?)null;
            report.PlatformOrderId = NullIfEmpty(order.Order.PlatformOrderId);
            report.Status = ReportStatus.Rejected;
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

        // Hosts should report refusals as PlatformException. Anything else thrown by a platform call is a call
        // that died without a verdict: for Submit the outcome is unknown (never re-submitted blindly), for
        // Change/Cancel it is treated as a failed operation and reported.

        private IReadOnlyList<string> GuardedSubmit(string account, IReadOnlyList<OrderRequest> orders)
        {
            EnsureWritable(account);
            try { return _platform.Submit(account, orders); }
            catch (PlatformException) { throw; }
            catch (Exception e) { throw new PlatformOutcomeUnknownException(e); }
        }

        private void GuardedChange(string account, string handle, int? quantity, decimal? limit, decimal? stop)
        {
            EnsureWritable(account);
            try { _platform.Change(account, handle, quantity, limit, stop); }
            catch (PlatformException) { throw; }
            catch (Exception e) { throw new PlatformException("change failed: " + e.GetType().Name, e); }
        }

        private void GuardedCancel(string account, IReadOnlyList<string> handles)
        {
            EnsureWritable(account);
            try { _platform.Cancel(account, handles); }
            catch (PlatformException) { throw; }
            catch (Exception e) { throw new PlatformException("cancel failed: " + e.GetType().Name, e); }
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
