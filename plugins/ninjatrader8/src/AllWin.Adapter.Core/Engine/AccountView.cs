using System.Collections.Generic;
using System.Linq;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Engine
{
    /// <summary>
    /// What the plugin window shows per platform account. The Core's sync state arrives in
    /// <c>welcome</c> today and in <c>account_state</c> from protocol 1.1 (Architect answer 3); both go
    /// through <see cref="AdapterEngine.ApplyCoreAccountState"/>. Local reasons are what the adapter
    /// observes itself, e.g. bracket integrity unknown while the Core session is down.
    /// </summary>
    public sealed class AccountView
    {
        public string PlatformAccount { get; set; } = "";
        public string? AccountId { get; set; }
        public IReadOnlyList<AccountRole> Roles { get; set; } = new AccountRole[0];
        public AccountKind Kind { get; set; }
        public OcoLocation OcoLocation { get; set; }
        public bool PlatformConnected { get; set; }

        /// <summary>Allowed by the simulation guard (ADR-0005 N4).</summary>
        public bool ExecutionAllowed { get; set; }

        /// <summary>
        /// Release builds, non-simulation accounts: the ADR-0005 §N11 state (B, A, or C0/C1/C2 for the worst contract).
        /// Null for simulation accounts and in development builds.
        /// </summary>
        public LiveState? LiveState { get; set; }

        /// <summary>Plugin alerts for this account (AdapterTexts keys), e.g. live.unconsented_actions.</summary>
        public IReadOnlyList<string> Notices { get; set; } = new string[0];

        public SyncState? CoreSyncState { get; set; }
        public IReadOnlyList<string> CoreSyncReasons { get; set; } = new string[0];
        public IReadOnlyList<string> LocalSyncReasons { get; set; } = new string[0];

        /// <summary>Brackets are only fully protected while NT8 runs (ADR-0004 §6.1 permanent notice).</summary>
        public bool ShowLocalOcoNotice => OcoLocation != OcoLocation.Server;

        public IEnumerable<string> AllSyncReasons => CoreSyncReasons.Concat(LocalSyncReasons).Distinct();
    }
}
