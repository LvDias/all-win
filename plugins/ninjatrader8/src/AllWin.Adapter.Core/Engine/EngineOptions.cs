using System;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Engine
{
    public sealed class EngineOptions
    {
        public string AdapterType { get; set; } = "ninjatrader8";
        public string AdapterVersion { get; set; } = "0.1.0";

        /// <summary>Development builds act only on simulation/playback accounts, whatever the Core says (ADR-0005 N4).</summary>
        public AdapterBuild Build { get; set; } = AdapterBuild.Development;

        public string? PlatformVersion { get; set; }

        /// <summary>
        /// U19 (ADR-0005 §N11, [Proposed]): what a confirmed live account may still do after "off". Build/server
        /// configuration only; <see cref="LiveAccountTable.ParseOffPolicy"/> maps missing/invalid to the recommended (a).
        /// </summary>
        public LiveOffPolicy LiveOffPolicy { get; set; } = LiveOffPolicy.CancelEntriesKeepExits;

        /// <summary>U19.1 (ADR-0005 §N11 row 11, [Proposed]); default the recommended (b).</summary>
        public ProtectiveCancelPolicy ProtectiveCancelPolicy { get; set; } = ProtectiveCancelPolicy.Refuse;

        /// <summary>Retention of msg_id and command_id dedupe records (Architect answer 8: at least 7 days).</summary>
        public TimeSpan DedupeRetention { get; set; } = TimeSpan.FromDays(7);

        /// <summary>
        /// Commands whose envelope sent_at is older than this (skew-corrected) are rejected command.stale
        /// and never executed, with or without a dedupe record (ADR-0004 §4 "Bounded re-delivery").
        /// Invariant: retention 7 d &gt; staleness 6 d &gt; Core retransmit horizon 72 h.
        /// </summary>
        public TimeSpan StalenessLimit { get; set; } = TimeSpan.FromDays(6);

        public TimeSpan PeriodicSnapshotInterval { get; set; } = TimeSpan.FromSeconds(60);

        /// <summary>A flatten fails if its cancels are not confirmed in this time (never closes over live orders).</summary>
        public TimeSpan FlattenCancelTimeout { get; set; } = TimeSpan.FromSeconds(10);

        /// <summary>A flatten whose close order is not final after this time releases the account queue as unknown.</summary>
        public TimeSpan FlattenCloseTimeout { get; set; } = TimeSpan.FromSeconds(60);

        /// <summary>Bounded re-tries when the position changed while a flatten was closing it.</summary>
        public int FlattenMaxAttempts { get; set; } = 3;

        /// <summary>
        /// A fill older than this on the platform clock when we receive it is not "live": no bracket is placed
        /// for it (Architect R1 (c); tuned by spike E4).
        /// </summary>
        public TimeSpan MaxBracketDelay { get; set; } = TimeSpan.FromSeconds(10);

        public TimeSpan PruneInterval { get; set; } = TimeSpan.FromHours(1);

        /// <summary>Defaults used before the first welcome (ADR-0004 §4).</summary>
        public int DefaultHeartbeatIntervalMs { get; set; } = 10000;
        public int DefaultAckTimeoutMs { get; set; } = 5000;
    }
}
