using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Sim
{
    /// <summary>
    /// A small deterministic broker: accounts, orders, OCO groups, fills, positions and executions.
    /// It raises events through <see cref="EventSink"/> instead of calling the engine directly, like NT8
    /// raises them on its own threads, so the engine is never re-entered from inside a platform call.
    /// Prices are exact decimals. Thread-safe: every member locks.
    /// </summary>
    public sealed class SimulatedPlatform : ITradingPlatform
    {
        private readonly object _gate = new object();
        private readonly Dictionary<string, SimAccount> _accounts = new Dictionary<string, SimAccount>(StringComparer.Ordinal);
        private readonly Dictionary<string, decimal> _tickSizes = new Dictionary<string, decimal>(StringComparer.Ordinal);
        private readonly Dictionary<string, decimal> _prices = new Dictionary<string, decimal>(StringComparer.Ordinal);
        private readonly IClock _clock;
        private int _nextHandle = 1;
        private int _nextOrderId = 1;
        private int _nextExecution = 1;
        private int _handleGeneration = 1;

        public SimulatedPlatform(IClock? clock = null)
        {
            _clock = clock ?? SystemClock.Instance;
        }

        /// <summary>Receives platform events (tests: a queue; harness: the engine loop).</summary>
        public Action<PlatformEvent> EventSink { get; set; } = _ => { };

        /// <summary>When false, OCO groups are not enforced: NT8 down with a locally simulated OCO (ADR-0004 §6.1).</summary>
        public bool OcoEnforced { get; set; } = true;

        /// <summary>When false, executions carry no PositionAfter and the engine queries positions instead.</summary>
        public bool IncludePositionAfter { get; set; } = true;

        /// <summary>Market orders fill in this many equal slices (partial fills).</summary>
        public int MarketFillSlices { get; set; } = 1;

        /// <summary>The next Submit call throws (platform refuses the call).</summary>
        public bool FailNextSubmit { get; set; }

        /// <summary>The next Submit dies abruptly (not a platform refusal): models a crash after the engine's write-ahead.</summary>
        public bool CrashNextSubmit { get; set; }

        /// <summary>The next Submit creates the orders and then dies before returning (a crash after the platform accepted).</summary>
        public bool CrashAfterSubmit { get; set; }

        /// <summary>Orders are accepted but rejected right after (e.g. broker rejection).</summary>
        public bool RejectNextOrder { get; set; }

        public bool FailNextCancel { get; set; }

        /// <summary>Cancels stay in CancelSubmitted until <see cref="ConfirmCancels"/> (a slow broker).</summary>
        public bool DeferCancels { get; set; }
        public bool FailNextChange { get; set; }

        public List<string> CallLog { get; } = new List<string>();

        // ------------------------------------------------------------------ setup

        public void AddAccount(string name, AccountKind kind = AccountKind.Simulation, OcoLocation oco = OcoLocation.Local, string connection = "Simulated Data Feed",
            string provider = "Simulator")
        {
            lock (_gate) _accounts[name] = new SimAccount(new PlatformAccountInfo { Name = name, Kind = kind, OcoLocation = oco, ConnectionName = connection, Provider = provider, Connected = true });
        }

        /// <summary>
        /// The kind the host derives for an account changes (e.g. the connection now reports another mode). In-memory
        /// test double only: it never talks to a broker.
        /// </summary>
        public void SetKind(string account, AccountKind kind)
        {
            lock (_gate)
            {
                Account(account).Info.Kind = kind;
                EventSink(new AccountsChangedEvent { Account = account });
            }
        }

        public void SetTickSize(string symbol, decimal tickSize)
        {
            lock (_gate) _tickSizes[symbol] = tickSize;
        }

        // ------------------------------------------------------------------ ITradingPlatform

        public IReadOnlyList<PlatformAccountInfo> GetAccounts()
        {
            lock (_gate) return _accounts.Values.Where(a => a.Visible).Select(a => Copy(a.Info)).ToList();
        }

        /// <summary>Hides an account from GetAccounts, like NT8 starting before the account's connection is up.</summary>
        public void SetVisible(string account, bool visible)
        {
            lock (_gate) Account(account).Visible = visible;
        }

        /// <summary>Playback accounts report no replay time (NT8 might not expose it).</summary>
        public bool PlaybackTimeUnavailable { get; set; }

        /// <summary>Execution times are reported with this DateTimeKind (a host that forgot to convert to UTC).</summary>
        public DateTimeKind ExecutionTimeKind { get; set; } = DateTimeKind.Utc;

        public DateTime? GetPlatformTime(string account)
        {
            lock (_gate)
            {
                var a = Visible(account);
                if (PlaybackTimeUnavailable && a.Info.Kind == AccountKind.Playback) return null;
                return _clock.UtcNow;
            }
        }

        /// <summary>Executions are stamped this much in the past (a fill delivered late by the platform).</summary>
        public TimeSpan ExecutionDelay { get; set; }

        public decimal? GetTickSize(Instrument instrument)
        {
            lock (_gate) return _tickSizes.TryGetValue(instrument.Symbol, out var t) ? t : (decimal?)null;
        }

        /// <summary>The next GetOrders call fails (the platform cannot answer the lookup).</summary>
        public bool FailNextGetOrders { get; set; }

        public IReadOnlyList<PlatformOrder> GetOrders(string account)
        {
            lock (_gate)
            {
                if (FailNextGetOrders)
                {
                    FailNextGetOrders = false;
                    throw new PlatformException("simulated lookup failure");
                }
                return Visible(account).Orders.Values.Select(o => o.Clone()).ToList();
            }
        }

        public IReadOnlyList<PlatformPosition> GetPositions(string account)
        {
            lock (_gate) return Visible(account).Positions.Values.Where(p => p.Quantity != 0).Select(Copy).ToList();
        }

        public IReadOnlyList<PlatformExecution> GetExecutions(string account)
        {
            lock (_gate) return Visible(account).Executions.Select(Copy).ToList();
        }

        public IReadOnlyList<string> Submit(string account, IReadOnlyList<OrderRequest> orders)
        {
            lock (_gate)
            {
                CallLog.Add("submit " + string.Join(",", orders.Select(o => o.Name)));
                var a = Visible(account);
                if (!a.Info.Connected) throw new PlatformException("account not connected");
                if (CrashNextSubmit)
                {
                    CrashNextSubmit = false;
                    throw new InvalidOperationException("simulated process death");
                }
                if (FailNextSubmit)
                {
                    FailNextSubmit = false;
                    throw new PlatformException("simulated submit failure");
                }
                var handles = new List<string>();
                foreach (var request in orders)
                {
                    if (request.Name.Length > OrderName.MaxLength) throw new PlatformException("name too long");
                    var order = new PlatformOrder
                    {
                        Handle = "h" + _handleGeneration.ToString(CultureInfo.InvariantCulture) + "-" + (_nextHandle++).ToString(CultureInfo.InvariantCulture),
                        PlatformOrderId = "o" + (_nextOrderId++).ToString(CultureInfo.InvariantCulture),
                        Name = request.Name,
                        Instrument = request.Instrument.Clone(),
                        Side = request.Side,
                        Type = request.Type,
                        Quantity = request.Quantity,
                        LimitPrice = request.LimitPrice,
                        StopPrice = request.StopPrice,
                        TimeInForce = request.TimeInForce,
                        State = PlatformOrderState.Submitted,
                        Time = _clock.UtcNow,
                    };
                    a.Orders[order.Handle] = order;
                    if (request.OcoId != null) a.Oco[order.Handle] = request.OcoId;
                    handles.Add(order.Handle);
                    RaiseOrder(account, order);
                    if (RejectNextOrder)
                    {
                        RejectNextOrder = false;
                        order.State = PlatformOrderState.Rejected;
                        RaiseOrder(account, order);
                        continue;
                    }
                    order.State = order.Type == OrderType.Stop || order.Type == OrderType.StopLimit ? PlatformOrderState.Accepted : PlatformOrderState.Working;
                    RaiseOrder(account, order);
                }
                foreach (var handle in handles) TryFill(account, a.Orders[handle]);
                if (CrashAfterSubmit)
                {
                    CrashAfterSubmit = false;
                    throw new InvalidOperationException("simulated death after the platform accepted");
                }
                return handles;
            }
        }

        public void Change(string account, string orderHandle, int? quantity, decimal? limitPrice, decimal? stopPrice)
        {
            lock (_gate)
            {
                CallLog.Add("change " + orderHandle);
                var a = Account(account);
                if (!a.Orders.TryGetValue(orderHandle, out var order) || order.IsTerminal) throw new PlatformException("order not working");
                if (FailNextChange)
                {
                    FailNextChange = false;
                    throw new PlatformException("simulated change failure");
                }
                var previous = order.State;
                order.State = PlatformOrderState.ChangeSubmitted;
                RaiseOrder(account, order);
                if (quantity.HasValue) order.Quantity = quantity.Value;
                if (limitPrice.HasValue) order.LimitPrice = limitPrice;
                if (stopPrice.HasValue) order.StopPrice = stopPrice;
                order.State = previous == PlatformOrderState.PartFilled ? PlatformOrderState.PartFilled : (order.Type == OrderType.Stop ? PlatformOrderState.Accepted : PlatformOrderState.Working);
                RaiseOrder(account, order);
                TryFill(account, order);
            }
        }

        public void Cancel(string account, IReadOnlyList<string> orderHandles)
        {
            lock (_gate)
            {
                CallLog.Add("cancel " + string.Join(",", orderHandles));
                var a = Account(account);
                if (FailNextCancel)
                {
                    FailNextCancel = false;
                    throw new PlatformException("simulated cancel failure");
                }
                foreach (var handle in orderHandles)
                {
                    if (!a.Orders.TryGetValue(handle, out var order) || order.IsTerminal) continue;
                    order.State = PlatformOrderState.CancelSubmitted;
                    RaiseOrder(account, order);
                    if (DeferCancels) continue;
                    order.State = PlatformOrderState.Cancelled;
                    RaiseOrder(account, order);
                }
            }
        }

        // ------------------------------------------------------------------ market and scenario controls

        /// <summary>Moves the market; working stops/limits that are touched fill at their price (market at last).</summary>
        public void SetPrice(string symbol, decimal price)
        {
            lock (_gate)
            {
                _prices[symbol] = price;
                foreach (var account in _accounts.Keys.ToList())
                    foreach (var order in Account(account).Orders.Values.Where(o => o.Instrument.Symbol == symbol && !o.IsTerminal).OrderBy(o => o.Time).ToList())
                        TryFill(account, order);
            }
        }

        /// <summary>No market data for the symbol: market orders rest instead of filling.</summary>
        public void ClearPrice(string symbol)
        {
            lock (_gate) _prices.Remove(symbol);
        }

        /// <summary>Fills part or all of a working order at <paramref name="price"/>.</summary>
        public void FillOrder(string account, string handle, int quantity, decimal price)
        {
            lock (_gate) ExecuteFill(account, Account(account).Orders[handle], quantity, price);
        }

        /// <summary>An execution placed outside All Win (manual order in NT8).</summary>
        public string ExternalFill(string account, Instrument instrument, Side side, int quantity, decimal price)
        {
            lock (_gate)
            {
                var order = new PlatformOrder
                {
                    Handle = "h" + _handleGeneration.ToString(CultureInfo.InvariantCulture) + "-" + (_nextHandle++).ToString(CultureInfo.InvariantCulture),
                    PlatformOrderId = "o" + (_nextOrderId++).ToString(CultureInfo.InvariantCulture),
                    Name = "Entry",
                    Instrument = instrument.Clone(),
                    Side = side,
                    Type = OrderType.Market,
                    Quantity = quantity,
                    State = PlatformOrderState.Working,
                    Time = _clock.UtcNow,
                };
                Account(account).Orders[order.Handle] = order;
                RaiseOrder(account, order);
                ExecuteFill(account, order, quantity, price);
                return order.Handle;
            }
        }

        /// <summary>The broker corrects (amend) or removes an earlier execution (NT8 ExecutionUpdate).</summary>
        public void AmendExecution(string account, string executionId, int quantity, decimal price)
        {
            lock (_gate)
            {
                var a = Account(account);
                var execution = a.Executions.Single(e => e.ExecutionId == executionId);
                ApplyToPosition(a, execution.Instrument, execution.Side, -execution.Quantity, execution.Price);
                execution.Quantity = quantity;
                execution.Price = price;
                ApplyToPosition(a, execution.Instrument, execution.Side, quantity, price);
                RaiseExecution(account, execution, FillOperation.Amend);
            }
        }

        public void RemoveExecution(string account, string executionId)
        {
            lock (_gate)
            {
                var a = Account(account);
                var execution = a.Executions.Single(e => e.ExecutionId == executionId);
                a.Executions.Remove(execution);
                ApplyToPosition(a, execution.Instrument, execution.Side, -execution.Quantity, execution.Price);
                RaiseExecution(account, execution, FillOperation.Remove);
            }
        }

        /// <summary>The platform no longer lists earlier executions (NT8 retention across a restart is unverified, E4/E9).</summary>
        public void ForgetExecutions(string account)
        {
            lock (_gate) Account(account).Executions.Clear();
        }

        public void SetConnected(string account, bool connected)
        {
            lock (_gate)
            {
                Account(account).Info.Connected = connected;
                EventSink(new AccountConnectionEvent { Account = account, Connected = connected });
            }
        }

        /// <summary>Sim reset / Playback rewind: orders and positions vanish, SimulationAccountReset fires.</summary>
        public void ResetAccount(string account)
        {
            lock (_gate)
            {
                var a = Account(account);
                a.Orders.Clear();
                a.Positions.Clear();
                a.Executions.Clear();
                a.Oco.Clear();
                EventSink(new AccountResetEvent { Account = account });
            }
        }

        public void ConfirmCancels()
        {
            lock (_gate)
            {
                foreach (var account in _accounts.Keys.ToList())
                    foreach (var order in Account(account).Orders.Values.Where(o => o.State == PlatformOrderState.CancelSubmitted).ToList())
                    {
                        order.State = PlatformOrderState.Cancelled;
                        RaiseOrder(account, order);
                    }
            }
        }

        /// <summary>The platform never received this order (e.g. the process died inside Submit).</summary>
        public void ForgetOrder(string account, string name)
        {
            lock (_gate)
            {
                var a = Account(account);
                foreach (var handle in a.Orders.Values.Where(o => o.Name == name).Select(o => o.Handle).ToList()) a.Orders.Remove(handle);
            }
        }

        /// <summary>Marks working stop orders Unknown, like simulated orders that could not be recovered.</summary>
        public void MarkUnknown(string account, string handle)
        {
            lock (_gate) Account(account).Orders[handle].State = PlatformOrderState.Unknown;
        }

        /// <summary>A new NT8 process: order objects (and therefore handles) are new; names and executions remain.</summary>
        public void RenewHandles()
        {
            lock (_gate)
            {
                _handleGeneration++;
                foreach (var a in _accounts.Values)
                {
                    var renewed = new Dictionary<string, PlatformOrder>(StringComparer.Ordinal);
                    var oco = new Dictionary<string, string>(StringComparer.Ordinal);
                    foreach (var order in a.Orders.Values)
                    {
                        var old = order.Handle;
                        order.Handle = "h" + _handleGeneration.ToString(CultureInfo.InvariantCulture) + "-" + (_nextHandle++).ToString(CultureInfo.InvariantCulture);
                        renewed[order.Handle] = order;
                        if (a.Oco.TryGetValue(old, out var group)) oco[order.Handle] = group;
                    }
                    a.Orders.Clear();
                    foreach (var pair in renewed) a.Orders[pair.Key] = pair.Value;
                    a.Oco.Clear();
                    foreach (var pair in oco) a.Oco[pair.Key] = pair.Value;
                    foreach (var execution in a.Executions) execution.OrderHandle = a.Orders.Values.FirstOrDefault(o => o.Name == execution.OrderName)?.Handle;
                }
            }
        }

        public PlatformOrder? OrderByName(string account, string name)
        {
            lock (_gate) return Account(account).Orders.Values.Where(o => o.Name == name).OrderBy(o => o.IsTerminal).Select(o => o.Clone()).FirstOrDefault();
        }

        public int Position(string account, string symbol)
        {
            lock (_gate) return Account(account).Positions.Values.Where(p => p.Instrument.Symbol == symbol).Sum(p => p.Quantity);
        }

        // ------------------------------------------------------------------ matching

        private void TryFill(string account, PlatformOrder order)
        {
            if (order.IsTerminal || order.State == PlatformOrderState.Submitted) return;
            if (!_prices.TryGetValue(order.Instrument.Symbol, out var last)) return;
            var buy = order.Side == Side.Buy;
            decimal? price = null;
            switch (order.Type)
            {
                case OrderType.Market: price = last; break;
                case OrderType.Limit: if (buy ? last <= order.LimitPrice : last >= order.LimitPrice) price = order.LimitPrice; break;
                case OrderType.Stop: if (buy ? last >= order.StopPrice : last <= order.StopPrice) price = order.StopPrice; break;
                case OrderType.StopLimit:
                    if ((buy ? last >= order.StopPrice : last <= order.StopPrice) && (buy ? last <= order.LimitPrice : last >= order.LimitPrice)) price = order.LimitPrice;
                    break;
            }
            if (!price.HasValue) return;
            var remaining = order.Quantity - order.FilledQuantity;
            if (order.Type == OrderType.Market && MarketFillSlices > 1)
            {
                var slice = Math.Max(1, order.Quantity / MarketFillSlices);
                while (remaining > 0)
                {
                    var q = Math.Min(slice, remaining);
                    ExecuteFill(account, order, q, price.Value);
                    remaining -= q;
                }
                return;
            }
            ExecuteFill(account, order, remaining, price.Value);
        }

        private void ExecuteFill(string account, PlatformOrder order, int quantity, decimal price)
        {
            var a = Account(account);
            order.FilledQuantity += quantity;
            order.State = order.FilledQuantity >= order.Quantity ? PlatformOrderState.Filled : PlatformOrderState.PartFilled;
            var execution = new PlatformExecution
            {
                ExecutionId = "x-" + (_nextExecution++).ToString(CultureInfo.InvariantCulture),
                OrderHandle = order.Handle,
                OrderName = order.Name,
                PlatformOrderId = order.PlatformOrderId,
                Instrument = order.Instrument.Clone(),
                Side = order.Side,
                Quantity = quantity,
                Price = price,
                Commission = 0.37m * quantity,
                Time = DateTime.SpecifyKind(_clock.UtcNow - ExecutionDelay, ExecutionTimeKind),
            };
            a.Executions.Add(execution);
            ApplyToPosition(a, order.Instrument, order.Side, quantity, price);
            RaiseOrder(account, order);
            RaiseExecution(account, execution, FillOperation.Add);

            if (OcoEnforced && order.State == PlatformOrderState.Filled && a.Oco.TryGetValue(order.Handle, out var group))
            {
                foreach (var other in a.Oco.Where(p => p.Value == group && p.Key != order.Handle).Select(p => a.Orders[p.Key]).Where(o => !o.IsTerminal).ToList())
                {
                    other.State = PlatformOrderState.Cancelled;
                    RaiseOrder(account, other);
                }
            }
        }

        private void ApplyToPosition(SimAccount a, Instrument instrument, Side side, int quantity, decimal price)
        {
            var position = PositionOf(a, instrument);
            var signed = side == Side.Buy ? quantity : -quantity;
            var before = position.Quantity;
            var after = before + signed;
            if (before == 0 || Math.Sign(before) == Math.Sign(signed))
            {
                // Opening or adding: exact VWAP of the position.
                var notional = (position.AveragePrice ?? 0m) * Math.Abs(before) + price * Math.Abs(signed);
                position.AveragePrice = after == 0 ? (decimal?)null : TickMath.Vwap(notional, Math.Abs(after));
            }
            else if (after != 0 && Math.Sign(after) != Math.Sign(before))
            {
                position.AveragePrice = price; // flipped
            }
            else if (after == 0)
            {
                position.AveragePrice = null;
            }
            position.Quantity = after;
            EventSink(new PositionUpdateEvent { Account = a.Info.Name, Position = Copy(position) });
        }

        private static PlatformPosition PositionOf(SimAccount a, Instrument instrument)
        {
            var key = instrument.ToString();
            if (!a.Positions.TryGetValue(key, out var position))
            {
                position = new PlatformPosition { Instrument = instrument.Clone() };
                a.Positions[key] = position;
            }
            return position;
        }

        private void RaiseOrder(string account, PlatformOrder order) => EventSink(new OrderUpdateEvent { Account = account, Order = order.Clone() });

        private void RaiseExecution(string account, PlatformExecution execution, FillOperation operation)
        {
            var copy = Copy(execution);
            copy.Operation = operation;
            if (IncludePositionAfter) copy.PositionAfter = Copy(PositionOf(Account(account), execution.Instrument));
            EventSink(new ExecutionUpdateEvent { Account = account, Execution = copy });
        }

        /// <summary>Platform API access: a hidden account does not exist for the caller (NT8 Account.All).</summary>
        private SimAccount Visible(string name)
        {
            var account = Account(name);
            if (!account.Visible) throw new PlatformException("account not available " + name);
            return account;
        }

        private SimAccount Account(string name)
        {
            if (!_accounts.TryGetValue(name, out var account)) throw new PlatformException("unknown account " + name);
            return account;
        }

        private static PlatformAccountInfo Copy(PlatformAccountInfo i) =>
            new PlatformAccountInfo { Name = i.Name, ConnectionName = i.ConnectionName, Provider = i.Provider, Kind = i.Kind, OcoLocation = i.OcoLocation, Connected = i.Connected };

        private static PlatformPosition Copy(PlatformPosition p) =>
            new PlatformPosition { Instrument = p.Instrument.Clone(), Quantity = p.Quantity, AveragePrice = p.AveragePrice };

        private static PlatformExecution Copy(PlatformExecution e) => new PlatformExecution
        {
            ExecutionId = e.ExecutionId, OrderHandle = e.OrderHandle, OrderName = e.OrderName, PlatformOrderId = e.PlatformOrderId,
            Instrument = e.Instrument.Clone(), Side = e.Side, Quantity = e.Quantity, Price = e.Price, Commission = e.Commission,
            Operation = e.Operation, Time = e.Time, PlatformTime = e.PlatformTime,
        };

        private sealed class SimAccount
        {
            public SimAccount(PlatformAccountInfo info) => Info = info;
            public PlatformAccountInfo Info { get; }
            public bool Visible { get; set; } = true;
            public Dictionary<string, PlatformOrder> Orders { get; } = new Dictionary<string, PlatformOrder>(StringComparer.Ordinal);
            public Dictionary<string, string> Oco { get; } = new Dictionary<string, string>(StringComparer.Ordinal);
            public Dictionary<string, PlatformPosition> Positions { get; } = new Dictionary<string, PlatformPosition>(StringComparer.Ordinal);
            public List<PlatformExecution> Executions { get; } = new List<PlatformExecution>();
        }
    }
}
