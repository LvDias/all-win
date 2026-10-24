// All Win — NT8 host: ITradingPlatform over the NinjaTrader 8 Account API, and the event bridge (ADR-0005 N6).
// Compiled by NinjaTrader on import (C# 7.3). Members the profile documents are used directly; connection
// details are read by reflection, so an unexpected shape degrades to "unknown" (refused) instead of a compile error.
#region Using declarations
using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Reflection;
using System.Threading;
using NinjaTrader.Cbi;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Platform;
// NinjaTrader.Cbi.LogLevel exists too: this alias picks ours (CS0104 in the user's NT8 on the 0.1.0-min import).
using LogLevel = AllWin.Adapter.Core.Engine.LogLevel;
using AwInstrument = AllWin.Adapter.Core.Protocol.Instrument;
using AwSide = AllWin.Adapter.Core.Protocol.Side;
using AwOrderType = AllWin.Adapter.Core.Protocol.OrderType;
using AwTimeInForce = AllWin.Adapter.Core.Protocol.TimeInForce;
using AwFillOperation = AllWin.Adapter.Core.Protocol.FillOperation;
using AwAccountKind = AllWin.Adapter.Core.Protocol.AccountKind;
#endregion

namespace NinjaTrader.NinjaScript.AddOns
{
    public sealed class AllWinPlatform : ITradingPlatform, IDisposable
    {
        private readonly object gate = new object();
        private readonly Dictionary<Order, string> handles = new Dictionary<Order, string>(ReferenceComparer<Order>.Instance);
        private readonly Dictionary<string, Order> orders = new Dictionary<string, Order>(StringComparer.Ordinal);
        private readonly Dictionary<string, DateTime> terminalSince = new Dictionary<string, DateTime>(StringComparer.Ordinal);

        /// <summary>
        /// Pêndulo L2: a terminal order's handle is dropped only after this delay, never at once. A late event of the same
        /// order (an execution after the Filled update) must keep its handle, or the engine would see another order.
        /// </summary>
        private static readonly TimeSpan HandleRetention = TimeSpan.FromMinutes(10);
        private readonly HashSet<Account> subscribed = new HashSet<Account>(ReferenceComparer<Account>.Instance);
        private readonly Dictionary<string, bool> lastConnected = new Dictionary<string, bool>(StringComparer.Ordinal);
        private readonly IAdapterLog log;
        private Action<PlatformEvent> sink;
        private Timer poll;
        private long nextHandle;
        private string lastAccountSet = "";

        public AllWinPlatform(IAdapterLog log)
        {
            this.log = log;
        }

        /// <summary>Starts forwarding platform events to <paramref name="eventSink"/> (the engine queue).</summary>
        public void Attach(Action<PlatformEvent> eventSink)
        {
            sink = eventSink;
            PollAccounts();
            // Accounts and connection states are polled once a second: simpler and safer than relying on the exact
            // shape of the static connection events (spike E4 records them for later).
            poll = new Timer(_ => PollAccounts(), null, 1000, 1000);
        }

        public void Dispose()
        {
            Timer t = poll;
            poll = null;
            if (t != null) t.Dispose();
            lock (gate)
            {
                foreach (Account a in subscribed)
                {
                    a.OrderUpdate -= OnOrderUpdate;
                    a.ExecutionUpdate -= OnExecutionUpdate;
                    a.PositionUpdate -= OnPositionUpdate;
                }
                subscribed.Clear();
            }
        }

        // ------------------------------------------------------------------ ITradingPlatform (called on the engine thread)

        public IReadOnlyList<PlatformAccountInfo> GetAccounts()
        {
            return AllAccounts().Select(Info).ToList();
        }

        public DateTime? GetPlatformTime(string account)
        {
            // Sim101 + Simulated Data Feed run on the PC clock. Playback time (E5) comes later.
            return DateTime.UtcNow;
        }

        public decimal? GetTickSize(AwInstrument instrument)
        {
            NinjaTrader.Cbi.Instrument nt = NtInstrument(instrument);
            if (nt == null) return null;
            return (decimal)nt.MasterInstrument.TickSize;
        }

        public IReadOnlyList<PlatformOrder> GetOrders(string account)
        {
            Account a = Required(account);
            List<Order> list;
            lock (a.Orders) list = a.Orders.ToList();
            return list.Select(o => ToPlatform(o, null, null, null)).Where(o => o != null).ToList();
        }

        public IReadOnlyList<PlatformPosition> GetPositions(string account)
        {
            Account a = Required(account);
            List<Position> list;
            lock (a.Positions) list = a.Positions.ToList();
            var result = new List<PlatformPosition>();
            foreach (Position p in list)
            {
                PlatformPosition mapped = ToPlatform(p);
                if (mapped != null && mapped.Quantity != 0) result.Add(mapped);
            }
            return result;
        }

        public IReadOnlyList<PlatformExecution> GetExecutions(string account)
        {
            Account a = Required(account);
            List<Execution> list;
            lock (a.Executions) list = a.Executions.ToList();
            return list.Select(x => ToPlatform(x, AwFillOperation.Add)).Where(x => x != null).ToList();
        }

        public IReadOnlyList<string> Submit(string account, IReadOnlyList<OrderRequest> requests)
        {
            Account a = Required(account);
            var created = new List<Order>();
            var result = new List<string>();
            try
            {
                foreach (OrderRequest r in requests)
                {
                    NinjaTrader.Cbi.Instrument instrument = NtInstrument(r.Instrument);
                    if (instrument == null) throw new PlatformException("instrument not found: " + NtInstrumentName.Format(r.Instrument));
                    Order order = a.CreateOrder(instrument, NtAction(a, r), NtType(r.Type), OrderEntry.Automated, NtTif(r.TimeInForce), r.Quantity,
                        r.LimitPrice.HasValue ? (double)r.LimitPrice.Value : 0, r.StopPrice.HasValue ? (double)r.StopPrice.Value : 0,
                        r.OcoId ?? "", r.Name, NinjaTrader.Core.Globals.MaxDate, null);
                    if (order == null) throw new PlatformException("CreateOrder returned null for " + r.Name);
                    created.Add(order);
                    result.Add(HandleOf(order));
                }
            }
            catch (PlatformException) { throw; }
            catch (Exception e) { throw new PlatformException("create failed: " + e.Message, e); }
            try { a.Submit(created); }
            catch (Exception e) { throw new PlatformOutcomeUnknownException(e); }
            return result;
        }

        public void Change(string account, string orderHandle, int? quantity, decimal? limitPrice, decimal? stopPrice)
        {
            Account a = Required(account);
            Order order = OrderOf(orderHandle);
            try
            {
                // Order.QuantityChanged / LimitPriceChanged / StopPriceChanged, then Account.Change (profile §6).
                if (quantity.HasValue) SetProp(order, "QuantityChanged", quantity.Value);
                if (limitPrice.HasValue) SetProp(order, "LimitPriceChanged", (double)limitPrice.Value);
                if (stopPrice.HasValue) SetProp(order, "StopPriceChanged", (double)stopPrice.Value);
                MethodInfo change = typeof(Account).GetMethod("Change", new[] { typeof(IEnumerable<Order>) });
                if (change == null) throw new PlatformException("Account.Change not found");
                change.Invoke(a, new object[] { new[] { order } });
            }
            catch (PlatformException) { throw; }
            catch (Exception e) { throw new PlatformException("change failed: " + Inner(e).Message, e); }
        }

        public void Cancel(string account, IReadOnlyList<string> orderHandles)
        {
            Account a = Required(account);
            var list = orderHandles.Select(OrderOf).ToList();
            try { a.Cancel(list); }
            catch (Exception e) { throw new PlatformException("cancel failed: " + e.Message, e); }
        }

        // ------------------------------------------------------------------ events (NT threads → engine queue)

        private void OnOrderUpdate(object sender, OrderEventArgs e)
        {
            try
            {
                Account a = sender as Account;
                // ADR-0005 N6 / profile §5: the event-args values give the sequence; the order object may be ahead.
                PlatformOrder order = ToPlatform(e.Order, e.OrderState, e.Filled, e.Time);
                if (a == null || order == null) return;
                Post(new OrderUpdateEvent { Account = a.Name, Order = order });
            }
            catch (Exception ex) { log.Write(LogLevel.Error, "host.order_event_failed", ex.GetType().Name + ": " + ex.Message); }
        }

        private void OnExecutionUpdate(object sender, ExecutionEventArgs e)
        {
            try
            {
                Account a = sender as Account;
                string operation = Str(Prop(e, "Operation"));
                AwFillOperation op = operation == "Remove" ? AwFillOperation.Remove : operation == "Update" ? AwFillOperation.Amend : AwFillOperation.Add;
                PlatformExecution execution = ToPlatform(e.Execution, op);
                if (a == null || execution == null) return;
                Post(new ExecutionUpdateEvent { Account = a.Name, Execution = execution });
            }
            catch (Exception ex) { log.Write(LogLevel.Error, "host.execution_event_failed", ex.GetType().Name + ": " + ex.Message); }
        }

        private void OnPositionUpdate(object sender, PositionEventArgs e)
        {
            try
            {
                Account a = sender as Account;
                PlatformPosition position = ToPlatform(e.Position);
                if (a == null || position == null) return;
                if (Str(Prop(e, "Operation")) == "Remove") position.Quantity = 0;
                Post(new PositionUpdateEvent { Account = a.Name, Position = position });
            }
            catch (Exception ex) { log.Write(LogLevel.Error, "host.position_event_failed", ex.GetType().Name + ": " + ex.Message); }
        }

        private void PruneHandles()
        {
            DateTime cutoff = DateTime.UtcNow - HandleRetention;
            lock (gate)
            {
                foreach (string h in terminalSince.Where(p => p.Value < cutoff).Select(p => p.Key).ToList())
                {
                    Order o;
                    if (orders.TryGetValue(h, out o)) handles.Remove(o);
                    orders.Remove(h);
                    terminalSince.Remove(h);
                }
            }
        }

        private void PollAccounts()
        {
            PruneHandles();
            try
            {
                List<Account> accounts = AllAccounts();
                string set = string.Join("|", accounts.Select(x => x.Name).OrderBy(x => x, StringComparer.Ordinal).ToArray());
                lock (gate)
                {
                    foreach (Account a in accounts)
                    {
                        if (subscribed.Contains(a)) continue;
                        a.OrderUpdate += OnOrderUpdate;
                        a.ExecutionUpdate += OnExecutionUpdate;
                        a.PositionUpdate += OnPositionUpdate;
                        subscribed.Add(a);
                    }
                }
                if (set != lastAccountSet)
                {
                    lastAccountSet = set;
                    // Guarita (Medium): only simulation/playback accounts are described in the log; the others are counted.
                    List<PlatformAccountInfo> infos = accounts.Select(Info).ToList();
                    List<PlatformAccountInfo> simulated = infos.Where(x => x.Kind == AwAccountKind.Simulation || x.Kind == AwAccountKind.Playback).ToList();
                    IEnumerable<string> others = infos.Where(x => x.Kind != AwAccountKind.Simulation && x.Kind != AwAccountKind.Playback)
                        .Select(x => "account=" + MaskName(x.Name) + " kind=" + x.Kind);
                    log.Write(LogLevel.Info, "host.accounts", string.Join("; ", simulated.Select(Describe).Concat(others).ToArray()));
                    Post(new AccountsChangedEvent());
                }
                foreach (Account a in accounts)
                {
                    bool connected = Info(a).Connected;
                    bool before;
                    bool known;
                    lock (gate) known = lastConnected.TryGetValue(a.Name, out before);
                    if (known && before == connected) continue;
                    lock (gate) lastConnected[a.Name] = connected;
                    if (known) Post(new AccountConnectionEvent { Account = a.Name, Connected = connected });
                }
            }
            catch (Exception ex) { log.Write(LogLevel.Error, "host.poll_failed", ex.GetType().Name + ": " + ex.Message); }
        }

        private void Post(PlatformEvent e)
        {
            Action<PlatformEvent> s = sink;
            if (s != null) s(e);
        }

        // ------------------------------------------------------------------ mapping

        private static List<Account> AllAccounts()
        {
            lock (Account.All) return Account.All.ToList();
        }

        private static Account Find(string name)
        {
            lock (Account.All) return Account.All.FirstOrDefault(x => x.Name == name);
        }

        private static Account Required(string name)
        {
            Account a = Find(name);
            if (a == null) throw new PlatformException("account not available " + name);
            return a;
        }

        /// <summary>
        /// ADR-0005 N4 (2026-10-24): the host only reads plain values (names from the real NT8 8.1.x metadata, exact Options
        /// type, the PlaybackConnection identity); AccountClassifier decides. Any read that throws makes the account unknown.
        /// </summary>
        public static PlatformAccountInfo Info(Account a)
        {
            var marks = new AccountMarks();
            string connectionName = null;
            string optionsType = null;
            bool connected = false;
            try
            {
                NinjaTrader.Cbi.Connection connection = a.Connection;
                NinjaTrader.Cbi.ConnectOptions options = connection == null ? null : connection.Options;
                marks.AccountProvider = a.Provider.ToString();
                if (options != null)
                {
                    Type type = options.GetType();
                    optionsType = type.FullName;
                    // Exact type only: a subclass of SimulatorOptions/PlaybackOptions is not the local simulator.
                    marks.OptionsExactlySimulator = type == typeof(NinjaTrader.Cbi.SimulatorOptions);
                    marks.OptionsExactlyPlayback = type == typeof(NinjaTrader.Cbi.PlaybackOptions);
                    marks.OptionsProvider = options.Provider.ToString();
                    marks.Mode = options.Mode.ToString();
                    marks.IsDemo = options.IsDemo;
                    connectionName = options.Name;
                }
                marks.IsPlaybackConnection = connection != null && ReferenceEquals(connection, NinjaTrader.Cbi.Connection.PlaybackConnection);
                connected = connection != null && connection.Status == NinjaTrader.Cbi.ConnectionStatus.Connected;
            }
            catch (Exception)
            {
                marks.ReadFailed = true;
            }
            return new PlatformAccountInfo
            {
                Name = a.Name,
                ConnectionName = connectionName,
                Provider = marks.OptionsProvider,
                AccountProvider = marks.AccountProvider,
                OptionsType = optionsType,
                Mode = marks.Mode,
                Kind = AccountClassifier.Kind(marks),
                OcoLocation = AccountClassifier.Oco(marks.OptionsProvider),
                Connected = connected,
            };
        }

        /// <summary>Crivo M1: a non-simulation account name in our logs keeps only its last 3 characters.</summary>
        public static string MaskName(string name)
        {
            if (string.IsNullOrEmpty(name)) return "***";
            return name.Length <= 3 ? "***" : "***" + name.Substring(name.Length - 3);
        }

        public static string Describe(PlatformAccountInfo i)
        {
            // Names only, never balances (Pêndulo condition 3): enough to prove the N4 decision from the log.
            return "account=" + i.Name + " connection=" + (i.ConnectionName ?? "?") + " account_provider=" + (i.AccountProvider ?? "?")
                + " options_provider=" + (i.Provider ?? "?") + " options_type=" + (i.OptionsType ?? "?") + " mode=" + (i.Mode ?? "?")
                + " kind=" + i.Kind + " oco=" + i.OcoLocation + " connected=" + i.Connected;
        }

        private string HandleOf(Order order)
        {
            lock (gate)
            {
                string handle;
                if (handles.TryGetValue(order, out handle)) return handle;
                handle = "nt" + Interlocked.Increment(ref nextHandle).ToString(CultureInfo.InvariantCulture);
                handles[order] = handle;
                orders[handle] = order;
                return handle;
            }
        }

        private Order OrderOf(string handle)
        {
            lock (gate)
            {
                Order order;
                if (orders.TryGetValue(handle, out order)) return order;
            }
            throw new PlatformException("unknown order handle " + handle);
        }

        private PlatformOrder ToPlatform(Order o, OrderState? state, int? filled, DateTime? time)
        {
            if (o == null) return null;
            OrderState current = state ?? o.OrderState;
            if (current == OrderState.Filled || current == OrderState.Cancelled || current == OrderState.Rejected)
            {
                string h = HandleOf(o);
                lock (gate) if (!terminalSince.ContainsKey(h)) terminalSince[h] = DateTime.UtcNow;
            }
            AwInstrument instrument;
            if (!NtInstrumentName.TryParse(o.Instrument == null ? null : o.Instrument.FullName, out instrument)) return null;
            decimal tick = TickOf(o.Instrument);
            return new PlatformOrder
            {
                Handle = HandleOf(o),
                PlatformOrderId = o.OrderId ?? "",
                Name = o.Name ?? "",
                Instrument = instrument,
                Side = o.OrderAction == OrderAction.Buy || o.OrderAction == OrderAction.BuyToCover ? AwSide.Buy : AwSide.Sell,
                Type = AwType(o.OrderType),
                Quantity = o.Quantity,
                FilledQuantity = filled ?? o.Filled,
                LimitPrice = o.OrderType == OrderType.Limit || o.OrderType == OrderType.StopLimit ? TickMath.FromPlatformDouble(o.LimitPrice, tick) : (decimal?)null,
                StopPrice = o.OrderType == OrderType.StopMarket || o.OrderType == OrderType.StopLimit ? TickMath.FromPlatformDouble(o.StopPrice, tick) : (decimal?)null,
                TimeInForce = o.TimeInForce == TimeInForce.Gtc ? AwTimeInForce.Gtc : AwTimeInForce.Day,
                State = StateOf(state ?? o.OrderState),
                Time = ToUtc(time ?? o.Time),
            };
        }

        private PlatformExecution ToPlatform(Execution x, AwFillOperation operation)
        {
            if (x == null) return null;
            AwInstrument instrument;
            if (!NtInstrumentName.TryParse(x.Instrument == null ? null : x.Instrument.FullName, out instrument)) return null;
            decimal tick = TickOf(x.Instrument);
            Order order = x.Order;
            AwSide side = order != null
                ? (order.OrderAction == OrderAction.Buy || order.OrderAction == OrderAction.BuyToCover ? AwSide.Buy : AwSide.Sell)
                : (x.MarketPosition == MarketPosition.Long ? AwSide.Buy : AwSide.Sell);
            DateTime utc = ToUtc(x.Time);
            return new PlatformExecution
            {
                ExecutionId = x.ExecutionId ?? "",
                OrderHandle = order == null ? null : HandleOf(order),
                OrderName = order == null ? null : order.Name,
                PlatformOrderId = order == null ? null : order.OrderId,
                Instrument = instrument,
                Side = side,
                Quantity = x.Quantity,
                Price = TickMath.FromPlatformDouble(x.Price, tick),
                // Pêndulo M1: protocol ABSENT = nothing reported (the Core applies its floor), 0 = really zero. NT8 gives a
                // double that is 0 both when there is no commission template and before a late commission (E14), so 0 is
                // sent as absent. PROVISIONAL until E14 shows when NT8 fills it.
                Commission = x.Commission > 0 ? (decimal?)TickMath.Normalize((decimal)x.Commission) : null,
                Operation = operation,
                Time = utc,
                PlatformTime = utc,
            };
        }

        private static PlatformPosition ToPlatform(Position p)
        {
            if (p == null) return null;
            AwInstrument instrument;
            if (!NtInstrumentName.TryParse(p.Instrument == null ? null : p.Instrument.FullName, out instrument)) return null;
            int quantity = p.MarketPosition == MarketPosition.Long ? p.Quantity : p.MarketPosition == MarketPosition.Short ? -p.Quantity : 0;
            return new PlatformPosition
            {
                Instrument = instrument,
                Quantity = quantity,
                AveragePrice = quantity == 0 ? (decimal?)null : TickMath.Normalize((decimal)p.AveragePrice),
            };
        }

        private static PlatformOrderState StateOf(OrderState s)
        {
            PlatformOrderState mapped;
            return Enum.TryParse(s.ToString(), out mapped) ? mapped : PlatformOrderState.Unknown;
        }

        /// <summary>NT8 times are local wall-clock times (Kind Unspecified or Local); the engine works in UTC.</summary>
        private static DateTime ToUtc(DateTime t)
        {
            if (t.Kind == DateTimeKind.Utc) return t;
            return DateTime.SpecifyKind(t, DateTimeKind.Local).ToUniversalTime();
        }

        private static decimal TickOf(NinjaTrader.Cbi.Instrument instrument)
        {
            return (decimal)instrument.MasterInstrument.TickSize;
        }

        private static NinjaTrader.Cbi.Instrument NtInstrument(AwInstrument instrument)
        {
            return NinjaTrader.Cbi.Instrument.GetInstrument(NtInstrumentName.Format(instrument));
        }

        /// <summary>Buy/BuyToCover/Sell/SellShort from the request side and the current position (profile §6).</summary>
        private static OrderAction NtAction(Account a, OrderRequest r)
        {
            int position = 0;
            List<Position> list;
            lock (a.Positions) list = a.Positions.ToList();
            foreach (Position p in list)
            {
                PlatformPosition mapped = ToPlatform(p);
                if (mapped != null && mapped.Instrument.Equals(r.Instrument)) position = mapped.Quantity;
            }
            if (r.Side == AwSide.Buy) return position < 0 ? OrderAction.BuyToCover : OrderAction.Buy;
            return position > 0 ? OrderAction.Sell : OrderAction.SellShort;
        }

        private static OrderType NtType(AwOrderType t)
        {
            switch (t)
            {
                case AwOrderType.Limit: return OrderType.Limit;
                case AwOrderType.Stop: return OrderType.StopMarket;
                case AwOrderType.StopLimit: return OrderType.StopLimit;
                default: return OrderType.Market;
            }
        }

        private static AwOrderType AwType(OrderType t)
        {
            switch (t)
            {
                case OrderType.Limit: return AwOrderType.Limit;
                case OrderType.StopMarket: return AwOrderType.Stop;
                case OrderType.StopLimit: return AwOrderType.StopLimit;
                default: return AwOrderType.Market;
            }
        }

        private static TimeInForce NtTif(AwTimeInForce t)
        {
            return t == AwTimeInForce.Gtc ? TimeInForce.Gtc : TimeInForce.Day;
        }

        // ------------------------------------------------------------------ reflection helpers

        internal static object Prop(object o, string name)
        {
            if (o == null) return null;
            try
            {
                PropertyInfo p = o.GetType().GetProperty(name, BindingFlags.Public | BindingFlags.Instance);
                return p == null ? null : p.GetValue(o, null);
            }
            catch (Exception) { return null; }
        }

        internal static string Str(object o)
        {
            return o == null ? null : Convert.ToString(o, CultureInfo.InvariantCulture);
        }

        private static void SetProp(object o, string name, object value)
        {
            PropertyInfo p = o.GetType().GetProperty(name, BindingFlags.Public | BindingFlags.Instance);
            if (p == null || !p.CanWrite) throw new PlatformException("Order." + name + " not found");
            p.SetValue(o, value, null);
        }

        private static Exception Inner(Exception e)
        {
            return e is TargetInvocationException && e.InnerException != null ? e.InnerException : e;
        }
    }

    /// <summary>Reference identity for NT objects (Order.OrderId changes over an order's life, profile §6).</summary>
    public sealed class ReferenceComparer<T> : IEqualityComparer<T> where T : class
    {
        public static readonly ReferenceComparer<T> Instance = new ReferenceComparer<T>();
        public bool Equals(T x, T y) { return ReferenceEquals(x, y); }
        public int GetHashCode(T obj) { return System.Runtime.CompilerServices.RuntimeHelpers.GetHashCode(obj); }
    }
}
