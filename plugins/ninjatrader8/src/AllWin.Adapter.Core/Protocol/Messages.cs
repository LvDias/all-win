using System;
using System.Collections.Generic;
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

// Typed payloads of adapter protocol v1 (contracts/adapter-protocol/v1/messages.schema.json).
// Property names map to snake_case; optional members are null when absent. Ids stay strings
// (the schema validates them), decimals are exact, timestamps are UTC DateTime.
namespace AllWin.Adapter.Core.Protocol
{
    public static class MessageTypes
    {
        public const string Hello = "hello";
        public const string Welcome = "welcome";
        public const string Error = "error";
        public const string Ack = "ack";
        public const string Heartbeat = "heartbeat";
        public const string Command = "command";
        public const string Limits = "limits";
        public const string InstructionResult = "instruction_result";
        public const string ExecutionReport = "execution_report";
        public const string PositionSnapshot = "position_snapshot";
        public const string Instruction = "instruction";
        public const string RiskEvent = "risk_event";
        public const string RiskState = "risk_state";

        /// <summary>Reliable messages stay in the sender's outbox until acked (ADR-0004 §3).</summary>
        public static bool IsReliable(string type) =>
            type == Command || type == Limits || type == InstructionResult || type == ExecutionReport
            || type == PositionSnapshot || type == Instruction || type == RiskEvent;
    }

    public enum Side { Buy, Sell }
    public enum OrderType { Market, Limit, Stop, StopLimit }
    public enum TimeInForce { Day, Gtc }
    public enum AccountKind { Simulation, Playback, Live, Unknown }
    public enum OcoLocation { Local, Server, Unknown }
    public enum ChildRole { Entry, Stop, Target, Exit, Flatten }
    public enum AdapterBuild { Release, Development }
    public enum AccountRole { Destination, Master }
    public enum SyncState { InSync, OutOfSync }
    public enum CommandKind { Place, Modify, Cancel, Flatten }
    public enum PositionEffect { Open, Reduce }
    public enum ProtectionMode { Individual, Shared }
    public enum FlattenReason { Close, Reverse, OrphanFill }
    public enum OrderOrigin { AllWin, LocalEnforcement, External }
    public enum FillOperation { Add, Amend, Remove }
    public enum SnapshotTrigger { Reconnect, Periodic, Change, Requested, AccountReset }
    public enum WorkingOrderState { Accepted, Working, TriggerPending, Unknown }
    public enum InstructionStatus { Accepted, Duplicate, Rejected }
    public enum DrawdownType { TrailingIntraday, TrailingEndOfDay, Static }
    public enum ProfitTargetAction { Notify, FlattenAndBlock }
    public enum RiskEventKind { Warning, LimitReached, Flattened, OrdersCancelled, Blocked, Unblocked, BlockedOrderCancelled }
    public enum RiskMetric { DailyLoss, Drawdown, DailyProfitTarget, MaxPosition }

    public enum Locale
    {
        [EnumMember(Value = "pt-BR")] PtBr,
        [EnumMember(Value = "en")] En,
    }

    public enum ReportStatus
    {
        Received, Working, PartiallyFilled, Filled, Cancelled, Rejected, Expired, Duplicate, Unknown,
    }

    public sealed class Instrument : IEquatable<Instrument>
    {
        public Instrument() { }

        public Instrument(string symbol, string contractMonth)
        {
            Symbol = symbol;
            ContractMonth = contractMonth;
        }

        public string Symbol { get; set; } = "";
        public string ContractMonth { get; set; } = "";

        public bool Equals(Instrument? other) => other != null && Symbol == other.Symbol && ContractMonth == other.ContractMonth;
        public override bool Equals(object? obj) => Equals(obj as Instrument);
        public override int GetHashCode() => (Symbol.GetHashCode() * 397) ^ ContractMonth.GetHashCode();
        public override string ToString() => Symbol + " " + ContractMonth;
        public Instrument Clone() => new Instrument(Symbol, ContractMonth);
    }

    public sealed class OrderSpec
    {
        public OrderType Type { get; set; }
        public decimal? LimitPrice { get; set; }
        public decimal? StopPrice { get; set; }
        public TimeInForce? TimeInForce { get; set; }

        public OrderSpec Clone() => (OrderSpec)MemberwiseClone();
    }

    // ---- handshake ----

    public sealed class HelloPayload
    {
        public string InstanceId { get; set; } = "";
        public AdapterInfo Adapter { get; set; } = new AdapterInfo();
        public List<string> SupportedProtocolVersions { get; set; } = new List<string>();
        public List<HelloAccount> Accounts { get; set; } = new List<HelloAccount>();
        public int? PendingOutbound { get; set; }
    }

    public sealed class AdapterInfo
    {
        public string Type { get; set; } = "";
        public string Version { get; set; } = "";
        public AdapterBuild Build { get; set; }
        public string? PlatformVersion { get; set; }
    }

    public sealed class HelloAccount
    {
        public string PlatformAccount { get; set; } = "";
        public string? ConnectionName { get; set; }
        public AccountKind Kind { get; set; }
        public OcoLocation OcoLocation { get; set; }
    }

    public sealed class WelcomePayload
    {
        public string SessionId { get; set; } = "";
        public string ConnectionId { get; set; } = "";
        public string ProtocolVersion { get; set; } = "";
        public int HeartbeatIntervalMs { get; set; }
        public int AckTimeoutMs { get; set; }
        public bool SimulationOnly { get; set; }
        public Locale Locale { get; set; }
        public List<WelcomeAccount> Accounts { get; set; } = new List<WelcomeAccount>();
    }

    public sealed class WelcomeAccount
    {
        public string AccountId { get; set; } = "";
        public string PlatformAccount { get; set; } = "";
        public List<AccountRole> Roles { get; set; } = new List<AccountRole>();
        public SyncState SyncState { get; set; }
        public List<string>? SyncReasons { get; set; }
    }

    public sealed class ErrorPayload
    {
        public string Code { get; set; } = "";
        public string? RefMsgId { get; set; }
        public string? Detail { get; set; }
    }

    public sealed class AckPayload
    {
        /// <summary>Contract limit per ack message.</summary>
        public const int MaxIds = 500;

        public List<string> MsgIds { get; set; } = new List<string>();
    }

    public sealed class HeartbeatPayload { }

    // ---- commands (C→A) ----

    public sealed class CommandPayload
    {
        public string CommandId { get; set; } = "";
        public string InstructionId { get; set; } = "";
        public string AccountId { get; set; } = "";
        public string PlatformAccount { get; set; } = "";
        public Instrument Instrument { get; set; } = new Instrument();
        public CommandKind Kind { get; set; }
        public DateTime? ExpiresAt { get; set; }
        public string? DependsOn { get; set; }
        public PlaceSpec? Place { get; set; }
        public ModifySpec? Modify { get; set; }
        public CancelSpec? Cancel { get; set; }
        public FlattenSpec? Flatten { get; set; }
        public string? ReasonCode { get; set; }

        /// <summary>Only position-increasing commands expire (ADR-0004 §5, T1).</summary>
        [JsonIgnore]
        public bool IsEntry => Kind == CommandKind.Place && Place != null && Place.PositionEffect == PositionEffect.Open;
    }

    public sealed class PlaceSpec
    {
        public Side Side { get; set; }
        public int Quantity { get; set; }
        public OrderSpec Order { get; set; } = new OrderSpec();
        public PositionEffect PositionEffect { get; set; }
        public ProtectionSpec? Protection { get; set; }
    }

    public sealed class ProtectionSpec
    {
        public ProtectionMode Mode { get; set; }
        public int? StopTicks { get; set; }
        public int? TargetTicks { get; set; }
    }

    public sealed class ModifySpec
    {
        public List<OrderTarget> Targets { get; set; } = new List<OrderTarget>();
        public decimal? PriceDelta { get; set; }
        public int? Quantity { get; set; }
    }

    public sealed class CancelSpec
    {
        public List<OrderTarget> Targets { get; set; } = new List<OrderTarget>();
    }

    public sealed class FlattenSpec
    {
        public FlattenReason Reason { get; set; }
    }

    public sealed class OrderTarget
    {
        public string CommandId { get; set; } = "";
        public ChildRole ChildRole { get; set; }
        public string? PlatformOrderId { get; set; }
    }

    public sealed class LimitsPayload
    {
        public List<AccountLimits> Accounts { get; set; } = new List<AccountLimits>();
    }

    public sealed class AccountLimits
    {
        public string AccountId { get; set; } = "";
        public string PlatformAccount { get; set; } = "";
        public long Version { get; set; }
        public bool EnforcementEnabled { get; set; }
        public int MaxPosition { get; set; }
        public decimal? DailyLossLimit { get; set; }
        public ProfitTarget? DailyProfitTarget { get; set; }
        public DrawdownSpec? Drawdown { get; set; }
        public TradingDaySpec TradingDay { get; set; } = new TradingDaySpec();
        public int WarningThresholdPct { get; set; }
        public DateTime? BlockedUntil { get; set; }
    }

    public sealed class ProfitTarget
    {
        public decimal Amount { get; set; }
        public ProfitTargetAction OnReach { get; set; }
    }

    public sealed class DrawdownSpec
    {
        public DrawdownType Type { get; set; }
        public decimal Amount { get; set; }
        public decimal? FloorBalance { get; set; }
        public decimal? HighWaterBalance { get; set; }
        public decimal? LockAtBalance { get; set; }
    }

    public sealed class TradingDaySpec
    {
        public string ResetLocalTime { get; set; } = "";
        public string TimeZone { get; set; } = "";
    }

    public sealed class InstructionResultPayload
    {
        public string MessageId { get; set; } = "";
        public InstructionStatus Status { get; set; }
        public string? InstructionId { get; set; }
        public string? ErrorCode { get; set; }
        public string? Pointer { get; set; }
    }

    // ---- reports (A→C) ----

    public sealed class ExecutionReportPayload
    {
        public string EventId { get; set; } = "";
        public string? CommandId { get; set; }
        public string AccountId { get; set; } = "";
        public string PlatformAccount { get; set; } = "";
        public Instrument Instrument { get; set; } = new Instrument();
        public OrderOrigin Origin { get; set; }
        public ChildRole? ChildRole { get; set; }
        public string? PlatformOrderId { get; set; }
        public ReportStatus Status { get; set; }
        public Side? Side { get; set; }
        public OrderSpec? Order { get; set; }
        public int? Quantity { get; set; }
        public FillInfo? Fill { get; set; }
        public PositionInfo? PositionAfter { get; set; }
        public string? ReasonCode { get; set; }
        public DateTime OccurredAt { get; set; }
        public DateTime? PlatformTime { get; set; }
    }

    public sealed class FillInfo
    {
        public string FillId { get; set; } = "";
        public int Quantity { get; set; }
        public decimal Price { get; set; }
        public decimal? Commission { get; set; }
        public FillOperation? Operation { get; set; }
    }

    public sealed class PositionInfo
    {
        /// <summary>Signed: positive long, negative short, 0 flat.</summary>
        public int Quantity { get; set; }
        public decimal? AveragePrice { get; set; }
    }

    public sealed class PositionSnapshotPayload
    {
        public SnapshotTrigger Trigger { get; set; }
        public DateTime TakenAt { get; set; }
        public List<SnapshotAccount> Accounts { get; set; } = new List<SnapshotAccount>();
    }

    public sealed class SnapshotAccount
    {
        public string AccountId { get; set; } = "";
        public string PlatformAccount { get; set; } = "";
        public bool Connected { get; set; }
        public List<SnapshotPosition> Positions { get; set; } = new List<SnapshotPosition>();
        public List<SnapshotWorkingOrder> WorkingOrders { get; set; } = new List<SnapshotWorkingOrder>();
    }

    public sealed class SnapshotPosition
    {
        public Instrument Instrument { get; set; } = new Instrument();
        public PositionInfo Position { get; set; } = new PositionInfo();
    }

    public sealed class SnapshotWorkingOrder
    {
        public string PlatformOrderId { get; set; } = "";
        public string? CommandId { get; set; }
        public ChildRole? ChildRole { get; set; }
        public Instrument Instrument { get; set; } = new Instrument();
        public Side Side { get; set; }
        public int Quantity { get; set; }
        public OrderSpec Order { get; set; } = new OrderSpec();
        public OrderOrigin Origin { get; set; }
        public WorkingOrderState? State { get; set; }
    }

    public sealed class InstructionPayload
    {
        public string AccountId { get; set; } = "";

        /// <summary>A standard instruction (ADR-0003), validated by instruction.schema.json.</summary>
        public JObject Instruction { get; set; } = new JObject();
    }

    public sealed class RiskEventPayload
    {
        public string EventId { get; set; } = "";
        public string AccountId { get; set; } = "";
        public RiskEventKind Kind { get; set; }
        public RiskMetric? Metric { get; set; }
        public decimal? Value { get; set; }
        public decimal? Limit { get; set; }
        public long LimitsVersion { get; set; }
        public bool? WhileDisconnected { get; set; }
        public DateTime OccurredAt { get; set; }
        public DateTime? PlatformTime { get; set; }
    }

    public sealed class RiskStatePayload
    {
        public string AccountId { get; set; } = "";
        public DateTime AsOf { get; set; }
        public decimal RealizedPnl { get; set; }
        public decimal UnrealizedPnl { get; set; }
        public decimal? DailyLossRemaining { get; set; }
        public decimal? DrawdownRemaining { get; set; }
        public decimal? Balance { get; set; }
    }
}
