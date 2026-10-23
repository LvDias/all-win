using System;
using System.Collections.Generic;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Platform
{
    /// <summary>
    /// Port to the trading platform. The NT8 AddOn host implements it with the Account API; tests and
    /// the Linux harness use <c>SimulatedPlatform</c>. All calls happen on the engine worker thread.
    /// Prices are already exact decimals: the host converts NT doubles with tick rounding.
    /// Methods throw <see cref="PlatformException"/> when the platform refuses a call.
    /// </summary>
    public interface ITradingPlatform
    {
        IReadOnlyList<PlatformAccountInfo> GetAccounts();

        /// <summary>Tick size of the contract, or null when the platform does not know it.</summary>
        decimal? GetTickSize(Instrument instrument);

        /// <summary>All orders the platform knows for the account in this session, terminal ones included.</summary>
        IReadOnlyList<PlatformOrder> GetOrders(string account);

        /// <summary>Non-flat positions of the account.</summary>
        IReadOnlyList<PlatformPosition> GetPositions(string account);

        /// <summary>Executions of the account the platform still holds (used to catch up after a restart).</summary>
        IReadOnlyList<PlatformExecution> GetExecutions(string account);

        /// <summary>Submits orders together (an OCO pair shares <see cref="OrderRequest.OcoId"/>). Returns one handle per request.</summary>
        IReadOnlyList<string> Submit(string account, IReadOnlyList<OrderRequest> orders);

        /// <summary>One change of quantity and/or prices of a working order (null = unchanged).</summary>
        void Change(string account, string orderHandle, int? quantity, decimal? limitPrice, decimal? stopPrice);

        void Cancel(string account, IReadOnlyList<string> orderHandles);
    }

    public class PlatformException : Exception
    {
        public PlatformException(string message) : base(message) { }
        public PlatformException(string message, Exception inner) : base(message, inner) { }
    }

    /// <summary>A Submit call died without a platform verdict: the order may or may not exist.</summary>
    public sealed class PlatformOutcomeUnknownException : PlatformException
    {
        public PlatformOutcomeUnknownException(Exception inner) : base("submit outcome unknown: " + inner.GetType().Name, inner) { }
    }

    public sealed class PlatformAccountInfo
    {
        public string Name { get; set; } = "";
        public string? ConnectionName { get; set; }

        /// <summary>Derived by the host from Connection.Options.Mode and the provider (ADR-0005 N4); unknown on any doubt.</summary>
        public AccountKind Kind { get; set; } = AccountKind.Unknown;

        /// <summary>Derived by the host from the provider (ADR-0005 N8).</summary>
        public OcoLocation OcoLocation { get; set; } = OcoLocation.Unknown;

        /// <summary>Order feed of the account's connection is connected.</summary>
        public bool Connected { get; set; }
    }

    /// <summary>Platform order state, mirroring NT8 OrderState.</summary>
    public enum PlatformOrderState
    {
        Initialized, Submitted, Accepted, TriggerPending, Working, ChangePending, ChangeSubmitted,
        CancelPending, CancelSubmitted, Cancelled, Rejected, PartFilled, Filled, Unknown,
    }

    public sealed class OrderRequest
    {
        public string Name { get; set; } = "";
        public Instrument Instrument { get; set; } = new Instrument();
        public Side Side { get; set; }
        public OrderType Type { get; set; }
        public int Quantity { get; set; }
        public decimal? LimitPrice { get; set; }
        public decimal? StopPrice { get; set; }
        public TimeInForce TimeInForce { get; set; } = TimeInForce.Day;
        public string? OcoId { get; set; }
    }

    /// <summary>Copy of an order state, taken in the platform callback (event-args values for the state).</summary>
    public sealed class PlatformOrder
    {
        /// <summary>Stable id the host assigns per order object (NT Order.OrderId changes over time).</summary>
        public string Handle { get; set; } = "";

        /// <summary>Current platform order id, reported to the Core.</summary>
        public string PlatformOrderId { get; set; } = "";

        public string Name { get; set; } = "";
        public Instrument Instrument { get; set; } = new Instrument();
        public Side Side { get; set; }
        public OrderType Type { get; set; }
        public int Quantity { get; set; }
        public int FilledQuantity { get; set; }
        public decimal? LimitPrice { get; set; }
        public decimal? StopPrice { get; set; }
        public TimeInForce TimeInForce { get; set; } = TimeInForce.Day;
        public PlatformOrderState State { get; set; }
        public DateTime Time { get; set; }

        public bool IsTerminal => State == PlatformOrderState.Cancelled || State == PlatformOrderState.Rejected || State == PlatformOrderState.Filled;

        public PlatformOrder Clone() => (PlatformOrder)MemberwiseClone();
    }

    public sealed class PlatformPosition
    {
        public Instrument Instrument { get; set; } = new Instrument();

        /// <summary>Signed: positive long, negative short.</summary>
        public int Quantity { get; set; }

        public decimal? AveragePrice { get; set; }
    }

    public sealed class PlatformExecution
    {
        /// <summary>Platform execution id; becomes <c>fill_id</c>.</summary>
        public string ExecutionId { get; set; } = "";

        public string? OrderHandle { get; set; }
        public string? OrderName { get; set; }
        public string? PlatformOrderId { get; set; }
        public Instrument Instrument { get; set; } = new Instrument();
        public Side Side { get; set; }
        public int Quantity { get; set; }
        public decimal Price { get; set; }
        public decimal? Commission { get; set; }
        public FillOperation Operation { get; set; } = FillOperation.Add;
        public DateTime Time { get; set; }
        public DateTime? PlatformTime { get; set; }

        /// <summary>Position after this execution if the host could read it consistently; else the engine queries it.</summary>
        public PlatformPosition? PositionAfter { get; set; }
    }

    /// <summary>Events copied out of platform callbacks into the engine queue (ADR-0005 N6).</summary>
    public abstract class PlatformEvent
    {
        public string Account { get; set; } = "";
    }

    public sealed class OrderUpdateEvent : PlatformEvent
    {
        public PlatformOrder Order { get; set; } = new PlatformOrder();
    }

    public sealed class ExecutionUpdateEvent : PlatformEvent
    {
        public PlatformExecution Execution { get; set; } = new PlatformExecution();
    }

    /// <summary>NT8 PositionUpdate; <see cref="IsFlat"/> is Operation.Remove (ADR-0005 N7).</summary>
    public sealed class PositionUpdateEvent : PlatformEvent
    {
        public PlatformPosition Position { get; set; } = new PlatformPosition();
        public bool IsFlat => Position.Quantity == 0;
    }

    public sealed class AccountConnectionEvent : PlatformEvent
    {
        public bool Connected { get; set; }
    }

    /// <summary>NT8 SimulationAccountReset (Sim reset or Playback rewind): a discontinuity, never fills.</summary>
    public sealed class AccountResetEvent : PlatformEvent { }

    /// <summary>Accounts were added or removed.</summary>
    public sealed class AccountsChangedEvent : PlatformEvent { }
}
