using System;
using System.Collections.Generic;
using AllWin.Adapter.Core.Protocol;

// Durable records of the engine (see StoreJson). They are versioned by adding optional members only.
namespace AllWin.Adapter.Core.Engine
{
    internal static class Collections
    {
        public const string Outbox = "out";
        public const string Inbound = "in";
        public const string Commands = "cmd";
        public const string Fills = "fill";
        public const string Limits = "lim";
        public const string Owners = "own";
        public const string Meta = "meta";
    }

    public enum CommandPhase
    {
        /// <summary>Recorded and waiting in the account queue.</summary>
        Queued,

        /// <summary>Sent to the platform (or running, for flatten); outcome pending.</summary>
        Dispatched,

        /// <summary>Final; <see cref="CommandRecord.Succeeded"/> tells success for depends_on.</summary>
        Done,
    }

    public enum FlattenStep { None, Cancelling, Closing }

    public enum TargetState { Pending, Applied, Failed }

    public sealed class CommandRecord
    {
        public string CommandId { get; set; } = "";

        /// <summary>Arrival order; rebuilds the per-account queue after a restart.</summary>
        public long Seq { get; set; }

        public string AccountId { get; set; } = "";
        public string PlatformAccount { get; set; } = "";
        public DateTime ReceivedAt { get; set; }

        /// <summary>Envelope sent_at: the Core's creation time, identical on every retransmission.</summary>
        public DateTime SentAt { get; set; }

        public DateTime? DoneAt { get; set; }
        public CommandPayload Command { get; set; } = new CommandPayload();
        public CommandPhase Phase { get; set; }
        public bool Succeeded { get; set; }

        /// <summary>Platform submit was attempted. Never re-submitted after a restart (never execute twice).</summary>
        public bool SubmitAttempted { get; set; }

        // Latest known state, repeated in `duplicate` answers (ADR-0004 §4).
        public LastState Last { get; set; } = new LastState();

        // place: entry fills drive the bracket (VWAP of this entry order, Architect answer 7).
        public int EntryFilled { get; set; }
        public decimal EntryNotional { get; set; }
        public int EntryQuantity { get; set; }
        public bool EntryTerminal { get; set; }
        public bool BracketSubmitted { get; set; }

        // flatten
        public FlattenStep FlattenStep { get; set; }
        public int FlattenAttempts { get; set; }
        public DateTime? StepStartedAt { get; set; }

        // modify / cancel: one outcome per target (Architect answer 9).
        public List<TargetRecord>? Targets { get; set; }

        /// <summary>Signature of the last lifecycle report per role letter, so a restart does not repeat it.</summary>
        public Dictionary<string, string> Reported { get; set; } = new Dictionary<string, string>();
    }

    public sealed class LastState
    {
        public ReportStatus? Status { get; set; }
        public string? ReasonCode { get; set; }
        public ChildRole? ChildRole { get; set; }
        public string? PlatformOrderId { get; set; }
        public Side? Side { get; set; }
        public OrderSpec? Order { get; set; }
        public int? Quantity { get; set; }
    }

    public sealed class TargetRecord
    {
        public string CommandId { get; set; } = "";
        public ChildRole ChildRole { get; set; }
        public string? Handle { get; set; }
        public TargetState State { get; set; }
        public int? Quantity { get; set; }
        public decimal? LimitPrice { get; set; }
        public decimal? StopPrice { get; set; }
    }

    public sealed class FillRecord
    {
        public string FillId { get; set; } = "";
        public int Quantity { get; set; }
        public decimal Price { get; set; }
        public bool Removed { get; set; }
        public string? CommandId { get; set; }
        public string? RoleLetter { get; set; }
        public DateTime RecordedAt { get; set; }
    }

    public sealed class OutboxEntry
    {
        public long Seq { get; set; }
        public string MsgId { get; set; } = "";
        public string Type { get; set; } = "";
        public string Frame { get; set; } = "";
        public DateTime CreatedAt { get; set; }

        [Newtonsoft.Json.JsonIgnore]
        public DateTime? LastSentAt { get; set; }
    }
}
