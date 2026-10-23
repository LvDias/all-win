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
        [Theory]
        [InlineData(true, "Simulator", AccountKind.Simulation)]
        [InlineData(true, "Playback", AccountKind.Playback)]
        [InlineData(false, "Rithmic", AccountKind.Live)]
        [InlineData(true, "Rithmic", AccountKind.Unknown)]   // simulation mode on a broker provider: refused
        [InlineData(null, "Simulator", AccountKind.Unknown)] // unreadable: fail closed
        [InlineData(true, null, AccountKind.Unknown)]
        public void Simulation_guard_never_trusts_names_and_fails_closed(bool? simulationMode, string? provider, AccountKind expected)
        {
            Assert.Equal(expected, AccountClassifier.Kind(simulationMode, provider));
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
