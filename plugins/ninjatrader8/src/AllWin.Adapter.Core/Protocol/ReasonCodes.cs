namespace AllWin.Adapter.Core.Protocol
{
    /// <summary>
    /// Adapter-originated stable codes, approved by the Architect for V1-S2 (2026-10-23). The registry
    /// will live in contracts/adapter-protocol/v1/reason-codes.md. protocol.* codes appear only in
    /// <c>error</c> messages, never in execution reports.
    /// </summary>
    public static class ReasonCodes
    {
        public const string CommandDuplicate = "command.duplicate";

        /// <summary>Envelope sent_at older than the staleness limit: rejected, recorded, never executed.</summary>
        public const string CommandStale = "command.stale";
        public const string CommandExpired = "command.expired";
        public const string CommandPrerequisiteFailed = "command.prerequisite_failed";
        public const string CommandAccountNotMapped = "command.account_not_mapped";
        public const string CommandAccountNotDestination = "command.account_not_destination";
        public const string CommandAccountNotSimulation = "command.account_not_simulation";
        public const string CommandInstrumentUnavailable = "command.instrument_unavailable";
        public const string CommandTargetNotFound = "command.target_not_found";
        public const string CommandAlreadyFlat = "command.already_flat";
        public const string CommandPlatformDisconnected = "command.platform_disconnected";

        /// <summary>Reduce found nothing to reduce (flat or same side): status cancelled, not a success (answer 11).</summary>
        public const string CommandNothingToReduce = "command.nothing_to_reduce";

        /// <summary>Informational: a reduce was capped to the current position (answer 11).</summary>
        public const string OrderQuantityCapped = "order.quantity_capped";
        public const string OrderPlatformRejected = "order.platform_rejected";
        public const string OrderOrphanCancelled = "order.orphan_cancelled";
        public const string OrderProtectionFailed = "order.protection_failed";
        public const string OrderCancelFailed = "order.cancel_failed";
        public const string RiskAccountBlocked = "risk.account_blocked";
        public const string ProtocolInvalidMessage = "protocol.invalid_message";
        public const string ProtocolUnsupportedValue = "protocol.unsupported_value";
        public const string ProtocolUnsupportedVersion = "protocol.unsupported_version";

        // Sync reasons shown in the plugin (Core is authoritative; see ADR-0004 §6.1).
        public const string SyncBracketIntegrityUnknown = "sync.bracket_integrity_unknown";
        public const string SyncBracketOrphanLeg = "sync.bracket_orphan_leg";
        public const string SyncBracketDoubleFill = "sync.bracket_double_fill";
        public const string SyncBracketIncomplete = "sync.bracket_incomplete";
        public const string SyncSimulationReset = "sync.simulation_reset";
    }
}
