using System;
using System.Collections.Generic;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Platform
{
    /// <summary>
    /// What the NT8 host read from one Account and its Connection (ADR-0005 N4, rule approved 2026-10-24), as plain values.
    /// The host only reads; <see cref="AccountClassifier.Kind(AccountMarks)"/> decides. Any read that threw sets
    /// <see cref="ReadFailed"/>; a value that could not be read is null.
    /// </summary>
    public sealed class AccountMarks
    {
        /// <summary>Some property read threw: the account is unknown, whatever else was read.</summary>
        public bool ReadFailed { get; set; }

        /// <summary>Account.Provider enum name (e.g. "Simulator", "Playback").</summary>
        public string? AccountProvider { get; set; }

        /// <summary>Account.Connection.Options.Provider enum name.</summary>
        public string? OptionsProvider { get; set; }

        /// <summary>Account.Connection.Options.Mode enum name ("Live" or "Simulation"). Logged; never grants simulation.</summary>
        public string? Mode { get; set; }

        /// <summary>options.GetType() == typeof(NinjaTrader.Cbi.SimulatorOptions): exact type, a subclass does not count.</summary>
        public bool OptionsExactlySimulator { get; set; }

        /// <summary>options.GetType() == typeof(NinjaTrader.Cbi.PlaybackOptions): exact type, a subclass does not count.</summary>
        public bool OptionsExactlyPlayback { get; set; }

        /// <summary>Account.Connection is the same object as the static Connection.PlaybackConnection.</summary>
        public bool IsPlaybackConnection { get; set; }

        /// <summary>ConnectOptions.IsDemo, logged only: a broker demo account is never local simulation.</summary>
        public bool? IsDemo { get; set; }
    }

    /// <summary>Pure rules the NT8 host applies to each account (ADR-0005 N4, N8).</summary>
    public static class AccountClassifier
    {
        private const string SimulatorProvider = "Simulator";
        private const string PlaybackProvider = "Playback";

        /// <summary>NT8 profile §7.3: OCO simulated on the local PC.</summary>
        private static readonly HashSet<string> LocalOcoProviders = new HashSet<string>(StringComparer.OrdinalIgnoreCase)
        {
            "NinjaTrader", "Rithmic", "Simulator", "Playback",
        };

        /// <summary>NT8 profile §7.3: OCO mostly native on the server.</summary>
        private static readonly HashSet<string> ServerOcoProviders = new HashSet<string>(StringComparer.OrdinalIgnoreCase)
        {
            "Cqg", "InteractiveBrokers",
        };

        /// <summary>
        /// ADR-0005 N4 (2026-10-24). Simulation needs ALL three local-simulator marks; Playback needs ALL four playback marks.
        /// In the real NT8 8.1.x metadata Mode defaults to Live and SimulatorOptions does not set it, so Mode never grants
        /// simulation. Unreadable, a failed read, an unexpected Mode, a partial simulator/playback mark set or Mode Simulation
        /// on any other provider (demo, evaluation or prop accounts) is Unknown. Everything else is Live. Only Simulation and
        /// Playback ever execute in a development build; the engine re-reads this before every command.
        /// </summary>
        public static AccountKind Kind(AccountMarks? marks)
        {
            if (marks == null || marks.ReadFailed) return AccountKind.Unknown;
            if (string.IsNullOrEmpty(marks.AccountProvider) || string.IsNullOrEmpty(marks.OptionsProvider) || string.IsNullOrEmpty(marks.Mode))
                return AccountKind.Unknown;
            if (marks.Mode != "Live" && marks.Mode != "Simulation") return AccountKind.Unknown;

            if (marks.AccountProvider == SimulatorProvider && marks.OptionsProvider == SimulatorProvider && marks.OptionsExactlySimulator)
                return AccountKind.Simulation;
            if (marks.AccountProvider == PlaybackProvider && marks.OptionsProvider == PlaybackProvider && marks.OptionsExactlyPlayback
                && marks.IsPlaybackConnection)
                return AccountKind.Playback;

            // Local simulator/playback options or connection without the full mark set: inconsistent, never Live, never sim.
            if (marks.OptionsProvider == SimulatorProvider || marks.OptionsProvider == PlaybackProvider
                || marks.OptionsExactlySimulator || marks.OptionsExactlyPlayback || marks.IsPlaybackConnection)
                return AccountKind.Unknown;
            if (marks.Mode == "Simulation") return AccountKind.Unknown;
            // Broker connection (including Sim101 used under a broker data feed: ADR-0005 N4 open case, fail closed).
            return AccountKind.Live;
        }

        public static OcoLocation Oco(string? provider)
        {
            if (string.IsNullOrEmpty(provider)) return OcoLocation.Unknown;
            if (LocalOcoProviders.Contains(provider!)) return OcoLocation.Local;
            if (ServerOcoProviders.Contains(provider!)) return OcoLocation.Server;
            return OcoLocation.Unknown;
        }
    }
}
