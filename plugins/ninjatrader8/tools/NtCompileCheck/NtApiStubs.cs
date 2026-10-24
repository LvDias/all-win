// Hand-written stubs of the NinjaTrader 8 API surface that the All Win host and probe use, for a Linux compile
// check only (ADR-0002: NinjaTrader DLLs are never committed or used here). Signatures follow the official
// NinjaScript help pages (docs/architecture/platforms/ninjatrader8-profile.md §5–§6); they are NOT the real
// assemblies, so a pass here does not prove the code compiles in NinjaTrader. Bodies throw on purpose.
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Windows;
using System.Windows.Controls;

namespace NinjaTrader.NinjaScript
{
    public enum State { SetDefaults, Configure, Active, DataLoaded, Historical, Transition, Realtime, Terminated, Finalized }

    public abstract class NinjaScript
    {
        public string Name { get; set; }
        public string Description { get; set; }
        public State State { get; protected set; }
        protected virtual void OnStateChange() { }
    }

    public abstract class AddOnBase : NinjaScript
    {
        protected virtual void OnWindowCreated(Window window) { }
        protected virtual void OnWindowDestroyed(Window window) { }
    }
}

namespace NinjaTrader.Core
{
    public static class Globals
    {
        public static DateTime MaxDate { get { throw new NotImplementedException(); } }
        public static System.Windows.Threading.Dispatcher RandomDispatcher { get { throw new NotImplementedException(); } }
    }
}

namespace NinjaTrader.Gui.Tools
{
    public class NTMenuItem : MenuItem { }

    public class NTWindow : Window
    {
        public object FindFirst(string name) { throw new NotImplementedException(); }
    }
}

namespace NinjaTrader.Gui
{
    public class ControlCenter : NinjaTrader.Gui.Tools.NTWindow { }
}

namespace NinjaTrader.Cbi
{
    public enum OrderAction { Buy, BuyToCover, Sell, SellShort }
    public enum OrderType { Limit, Market, MIT, StopMarket, StopLimit }
    public enum OrderEntry { Automated, Manual }
    public enum TimeInForce { Day, Gtc, Gtd, Ioc, Opg }
    public enum OrderState { Initialized, Submitted, Accepted, TriggerPending, Working, ChangePending, ChangeSubmitted, CancelPending, CancelSubmitted, Cancelled, Rejected, PartFilled, Filled, Unknown }

    public enum MarketPosition { Flat, Long, Short }

    public class CustomOrder { }

    public class MasterInstrument
    {
        public double TickSize { get { throw new NotImplementedException(); } }
        public double RoundToTickSize(double price) { throw new NotImplementedException(); }
    }

    public class Instrument
    {
        public string FullName { get { throw new NotImplementedException(); } }
        public MasterInstrument MasterInstrument { get { throw new NotImplementedException(); } }
        public static Instrument GetInstrument(string name) { throw new NotImplementedException(); }
    }

    public class Order
    {
        public string Name { get { throw new NotImplementedException(); } }
        public string OrderId { get { throw new NotImplementedException(); } }
        public Instrument Instrument { get { throw new NotImplementedException(); } }
        public OrderState OrderState { get { throw new NotImplementedException(); } }
        public OrderAction OrderAction { get { throw new NotImplementedException(); } }
        public OrderType OrderType { get { throw new NotImplementedException(); } }
        public TimeInForce TimeInForce { get { throw new NotImplementedException(); } }
        public int Quantity { get { throw new NotImplementedException(); } }
        public int Filled { get { throw new NotImplementedException(); } }
        public double LimitPrice { get { throw new NotImplementedException(); } }
        public double StopPrice { get { throw new NotImplementedException(); } }
        public DateTime Time { get { throw new NotImplementedException(); } }
    }

    public class Execution
    {
        public string ExecutionId { get { throw new NotImplementedException(); } }
        public Order Order { get { throw new NotImplementedException(); } }
        public Instrument Instrument { get { throw new NotImplementedException(); } }
        public MarketPosition MarketPosition { get { throw new NotImplementedException(); } }
        public int Quantity { get { throw new NotImplementedException(); } }
        public double Price { get { throw new NotImplementedException(); } }
        public double Commission { get { throw new NotImplementedException(); } }
        public DateTime Time { get { throw new NotImplementedException(); } }
    }

    public class Position
    {
        public Instrument Instrument { get { throw new NotImplementedException(); } }
        public MarketPosition MarketPosition { get { throw new NotImplementedException(); } }
        public int Quantity { get { throw new NotImplementedException(); } }
        public double AveragePrice { get { throw new NotImplementedException(); } }
    }

    public class OrderEventArgs : EventArgs
    {
        public Order Order { get { throw new NotImplementedException(); } }
        public OrderState OrderState { get { throw new NotImplementedException(); } }
        public double AverageFillPrice { get { throw new NotImplementedException(); } }
        public int Filled { get { throw new NotImplementedException(); } }
        public DateTime Time { get { throw new NotImplementedException(); } }
    }

    public class ExecutionEventArgs : EventArgs
    {
        public Execution Execution { get { throw new NotImplementedException(); } }
    }

    public class PositionEventArgs : EventArgs
    {
        public Position Position { get { throw new NotImplementedException(); } }
    }

    public class Account
    {
        public static Collection<Account> All { get { throw new NotImplementedException(); } }
        public string Name { get { throw new NotImplementedException(); } }
        public Collection<Execution> Executions { get { throw new NotImplementedException(); } }
        public Collection<Order> Orders { get { throw new NotImplementedException(); } }
        public Collection<Position> Positions { get { throw new NotImplementedException(); } }

        public event EventHandler<OrderEventArgs> OrderUpdate { add { } remove { } }
        public event EventHandler<ExecutionEventArgs> ExecutionUpdate { add { } remove { } }
        public event EventHandler<PositionEventArgs> PositionUpdate { add { } remove { } }

        public Order CreateOrder(Instrument instrument, OrderAction action, OrderType orderType, OrderEntry orderEntry, TimeInForce timeInForce,
            int quantity, double limitPrice, double stopPrice, string oco, string name, DateTime gtd, CustomOrder customOrder)
        { throw new NotImplementedException(); }

        public void Submit(IEnumerable<Order> orders) { throw new NotImplementedException(); }
        public void Flatten(ICollection<Instrument> instruments) { throw new NotImplementedException(); }
        public void Cancel(IEnumerable<Order> orders) { throw new NotImplementedException(); }
    }
}
