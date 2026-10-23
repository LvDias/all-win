using System;
using System.Collections.Generic;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Platform
{
    /// <summary>
    /// Pure rules the NT8 host applies to each Account's Connection (ADR-0005 N4, N8). The host passes
    /// the raw values it read; any value it could not read is null, which classifies as unknown and is
    /// therefore refused (fail closed). Provider names are NT8 <c>Provider</c> enum names; the exact
    /// names for local simulation are confirmed in spike experiment E4.
    /// </summary>
    public static class AccountClassifier
    {
        /// <summary>Providers whose accounts are local simulation (N4): Simulated Data Feed and Playback.</summary>
        private static readonly HashSet<string> SimulationProviders = new HashSet<string>(StringComparer.OrdinalIgnoreCase)
        {
            "Simulator", "Playback",
        };

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

        /// <param name="isSimulationMode">Connection.Options.Mode == Mode.Simulation; null if it could not be read.</param>
        /// <param name="provider">Connection.Options.Provider name; null if it could not be read.</param>
        public static AccountKind Kind(bool? isSimulationMode, string? provider)
        {
            if (isSimulationMode == null || string.IsNullOrEmpty(provider)) return AccountKind.Unknown;
            if (isSimulationMode.Value && string.Equals(provider, "Playback", StringComparison.OrdinalIgnoreCase)) return AccountKind.Playback;
            if (isSimulationMode.Value && SimulationProviders.Contains(provider!)) return AccountKind.Simulation;
            // Mode.Live, or simulation mode on a broker provider (e.g. prop evaluation accounts reporting Live): never simulation.
            return isSimulationMode.Value ? AccountKind.Unknown : AccountKind.Live;
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
