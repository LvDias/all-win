using System.Linq;
using System.Reflection;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Localization;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;
using Xunit;

namespace AllWin.Adapter.Core.Tests
{
    public class HostRulesTests
    {
        // ADR-0005 N4 (2026-10-24), from the real NinjaTrader.Core.dll metadata: Mode { Live = 0, Simulation = 1 } defaults to
        // Live and SimulatorOptions does not set it; Provider.Simulator = 15, Provider.Playback = 13; brokers are named,
        // ProviderN or CustomN. Columns: account provider, options provider, mode, exact SimulatorOptions, exact
        // PlaybackOptions, is Connection.PlaybackConnection, IsDemo, read failed → kind.
        [Theory]
        // The real case that failed on the user's NT8 (Sim101 + Simulated Data Feed, Mode Live): simulation.
        [InlineData("Simulator", "Simulator", "Live", true, false, false, false, false, AccountKind.Simulation)]
        [InlineData("Simulator", "Simulator", "Simulation", true, false, false, false, false, AccountKind.Simulation)] // Mode irrelevant
        [InlineData("Simulator", "Simulator", "Live", true, false, false, true, false, AccountKind.Simulation)]        // IsDemo irrelevant
        // A subclass of SimulatorOptions (not the exact type) or any missing simulator mark: never simulation.
        [InlineData("Simulator", "Simulator", "Live", false, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Simulator", "Simulator", "Simulation", false, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Simulator", "Playback", "Live", true, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Playback", "Simulator", "Live", true, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Custom3", "Simulator", "Live", true, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Simulator", "Simulator", "Live", false, true, false, false, false, AccountKind.Unknown)]           // playback options under simulator providers
        // Playback: all four marks.
        [InlineData("Playback", "Playback", "Live", false, true, true, false, false, AccountKind.Playback)]
        [InlineData("Playback", "Playback", "Simulation", false, true, true, false, false, AccountKind.Playback)]
        [InlineData("Playback", "Playback", "Live", false, true, false, false, false, AccountKind.Unknown)]             // not the PlaybackConnection
        [InlineData("Playback", "Playback", "Live", false, false, true, false, false, AccountKind.Unknown)]             // PlaybackOptions subclass
        [InlineData("Playback", "Custom7", "Live", false, true, true, false, false, AccountKind.Unknown)]
        [InlineData("Custom7", "Playback", "Live", false, true, true, false, false, AccountKind.Unknown)]
        [InlineData("Simulator", "Simulator", "Live", false, false, true, false, false, AccountKind.Unknown)]           // a simulator account on the PlaybackConnection
        // Mode Simulation on a broker, prop or demo connection: never simulation, never Live.
        [InlineData("Custom3", "Custom3", "Simulation", false, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Provider12", "Provider12", "Simulation", false, false, false, false, false, AccountKind.Unknown)]
        [InlineData("InteractiveBrokers", "InteractiveBrokers", "Simulation", false, false, false, true, false, AccountKind.Unknown)]
        // Broker connections: Live (refused in development; N11 in release), demo or not.
        [InlineData("Custom3", "Custom3", "Live", false, false, false, false, false, AccountKind.Live)]
        [InlineData("Provider12", "Provider12", "Live", false, false, false, true, false, AccountKind.Live)]
        [InlineData("InteractiveBrokers", "InteractiveBrokers", "Live", false, false, false, false, false, AccountKind.Live)]
        // N4 open case (fail closed): Sim101 under a broker data feed. The first row is the REAL case seen on the user's NT8
        // (min1 round, 2026-10-24): Sim101 attached to a Tradovate connection, Options provider Provider31, Mode Live → refused.
        [InlineData("Simulator", "Provider31", "Live", false, false, false, false, false, AccountKind.Live)]
        [InlineData("Simulator", "Custom3", "Live", false, false, false, false, false, AccountKind.Live)]
        // Unreadable, a failed read, an unexpected Mode or a different spelling: Unknown.
        [InlineData(null, "Simulator", "Live", true, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Simulator", null, "Live", true, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Simulator", "Simulator", null, true, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Simulator", "Simulator", "Live", true, false, false, false, true, AccountKind.Unknown)]
        [InlineData("Playback", "Playback", "Live", false, true, true, false, true, AccountKind.Unknown)]
        [InlineData("Custom3", "Custom3", "Live", false, false, false, false, true, AccountKind.Unknown)]
        [InlineData("Simulator", "Simulator", "Hybrid", true, false, false, false, false, AccountKind.Unknown)]
        [InlineData("simulator", "simulator", "Live", true, false, false, false, false, AccountKind.Unknown)]
        [InlineData("", "", "Live", false, false, false, false, false, AccountKind.Unknown)]
        [InlineData("Custom3", "Custom3", "", false, false, false, false, false, AccountKind.Unknown)]
        public void Account_kind_needs_every_local_simulator_or_playback_mark(string? accountProvider, string? optionsProvider, string? mode,
            bool exactlySimulator, bool exactlyPlayback, bool playbackConnection, bool isDemo, bool readFailed, AccountKind expected)
        {
            var marks = new AccountMarks
            {
                AccountProvider = accountProvider, OptionsProvider = optionsProvider, Mode = mode, OptionsExactlySimulator = exactlySimulator,
                OptionsExactlyPlayback = exactlyPlayback, IsPlaybackConnection = playbackConnection, IsDemo = isDemo, ReadFailed = readFailed,
            };
            Assert.Equal(expected, AccountClassifier.Kind(marks));
        }

        [Fact]
        public void No_marks_at_all_is_unknown()
        {
            Assert.Equal(AccountKind.Unknown, AccountClassifier.Kind(null));
            Assert.Equal(AccountKind.Unknown, AccountClassifier.Kind(new AccountMarks()));
        }

        [Theory]
        [InlineData("Rithmic", OcoLocation.Local)]
        [InlineData("NinjaTrader", OcoLocation.Local)]
        [InlineData("Playback", OcoLocation.Local)]
        [InlineData("Cqg", OcoLocation.Server)]
        [InlineData("Tradovate", OcoLocation.Unknown)]
        [InlineData(null, OcoLocation.Unknown)]
        public void Oco_location_follows_the_provider(string? provider, OcoLocation expected)
        {
            Assert.Equal(expected, AccountClassifier.Oco(provider));
        }

        [Theory]
        [InlineData("MNQ 12-26", "MNQ", "2026-12")]
        [InlineData("ES 03-27", "ES", "2027-03")]
        public void Nt_instrument_names_round_trip(string full, string symbol, string month)
        {
            Assert.True(NtInstrumentName.TryParse(full, out var instrument));
            Assert.Equal(new Instrument(symbol, month), instrument);
            Assert.Equal(full, NtInstrumentName.Format(instrument));
        }

        [Theory]
        [InlineData("MNQ")]
        [InlineData("MNQ 13-26")]
        [InlineData("mnq 12-26")]
        [InlineData("EURUSD")]
        [InlineData(null)]
        public void Non_futures_names_are_refused(string? full)
        {
            Assert.False(NtInstrumentName.TryParse(full, out _));
        }

        [Fact]
        public void Every_text_exists_in_both_languages()
        {
            Assert.All(AdapterTexts.Keys, k => Assert.True(AdapterTexts.HasBoth(k), k));
        }

        [Fact]
        public void Every_adapter_reason_code_has_a_text()
        {
            var codes = typeof(ReasonCodes).GetFields(BindingFlags.Public | BindingFlags.Static).Select(f => (string)f.GetValue(null)!)
                .Where(c => !c.StartsWith("protocol.", System.StringComparison.Ordinal));
            Assert.All(codes, c => Assert.True(AdapterTexts.HasBoth(c), c));
        }

        [Fact]
        public void Unknown_sync_reason_falls_back_to_a_generic_text()
        {
            Assert.Equal("Fora de sincronia.", AdapterTexts.Get("sync.something_new", Locale.PtBr));
        }

        [Fact]
        public void Stop_reasons_and_session_states_have_texts()
        {
            foreach (var reason in System.Enum.GetValues(typeof(StopReason)).Cast<StopReason>().Where(r => r != StopReason.None))
                Assert.True(AdapterTexts.HasBoth("stop." + ToSnake(reason.ToString())), reason.ToString());
            foreach (var state in System.Enum.GetValues(typeof(SessionState)).Cast<SessionState>())
                Assert.True(AdapterTexts.HasBoth("session." + ToSnake(state.ToString())), state.ToString());
        }

        private static string ToSnake(string name) => string.Concat(name.Select((c, i) => i > 0 && char.IsUpper(c) ? "_" + char.ToLowerInvariant(c) : char.ToLowerInvariant(c).ToString()));
    }
}
