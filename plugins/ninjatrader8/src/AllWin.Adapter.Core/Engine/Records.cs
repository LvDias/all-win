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
        public const string Identities = "pid";
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

        /// <summary>
        /// Envelope protocol_version. A reduce from a 1.0/1.1 envelope cannot carry refs and is refused by the
        /// position-identity guard; a record written before 1.2 has none, which counts as "1.0" (fail closed).
        /// </summary>
        public string? EnvelopeVersion { get; set; }

        /// <summary>
        /// The entry was submitted while its live account was in state B (ADR-0005 §N11 row 6): its own fills may
        /// still be protected after "off".
        /// </summary>
        public bool ConsentedLive { get; set; }

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

        /// <summary>The bracket was deliberately not submitted after a catch-up fill (answer A): never later.</summary>
        public bool ProtectionWithheld { get; set; }

        /// <summary>Connection-loss epoch of the account when the entry was submitted (R1 (b)).</summary>
        public long SubmitEpoch { get; set; }

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

        /// <summary>The requested quantity was capped so protection stays within the position.</summary>
        public bool Capped { get; set; }
    }

    /// <summary>
    /// Identity of the position in one (account, instrument) (ADR-0004 §5 position-identity guard, U13). Written
    /// in the same journal batch as the fill that changes it; removed when the position goes flat.
    /// </summary>
    public sealed class PositionIdentityRecord
    {
        public const string External = "external";
        public const string Unknown = "unknown";

        public string Account { get; set; } = "";
        public Instrument Instrument { get; set; } = new Instrument();

        /// <summary>The opening All Win place command_id, <see cref="External"/> or <see cref="Unknown"/>.</summary>
        public string Identity { get; set; } = Unknown;

        /// <summary>Signed quantity the record describes.</summary>
        public int Quantity { get; set; }

        public string? LastFillId { get; set; }

        /// <summary>The fill that set the identity; a correction of it makes the identity unknown.</summary>
        public string? OpeningFillId { get; set; }

        [Newtonsoft.Json.JsonIgnore]
        public bool IsAllWin => Identity != External && Identity != Unknown;
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
