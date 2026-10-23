using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Protocol;
using AllWin.FakeCore;
using Xunit;

namespace AllWin.Adapter.Core.Tests.Harness
{
    /// <summary>
    /// End to end over a real loopback WebSocket: fake Core ↔ WebSocketConnector ↔ EngineLoop ↔ engine ↔
    /// simulated platform. The same scenario files run against NinjaTrader 8 on the Windows machine.
    /// </summary>
    public class FakeCoreScenarioTests
    {
        public static IEnumerable<object[]> Scenarios() =>
            Directory.GetFiles(Path.Combine(System.AppContext.BaseDirectory, "scenarios"), "*.json").OrderBy(f => f).Select(f => new object[] { Path.GetFileName(f) });

        [Theory]
        [MemberData(nameof(Scenarios))]
        public async Task Scenario_passes_with_the_simulated_platform(string file)
        {
            var scenario = ProtocolJson.ParseObject(File.ReadAllText(Path.Combine(System.AppContext.BaseDirectory, "scenarios", file)));
            var trace = new List<string>();
            var token = "awd_TestKeyId001_" + new string('a', 43);
            await using var core = new FakeCoreServer(ScenarioRunner.AccountsOf(scenario), token, l => { lock (trace) trace.Add(l); });
            await core.StartAsync();
            using var sim = new SimAdapterHost(scenario, core.Endpoint!, token, l => { lock (trace) trace.Add(l); });
            sim.Start();
            var result = await new ScenarioRunner(scenario, core, sim).RunAsync();
            Assert.True(result.Passed, result.Failure + "\n" + string.Join("\n", result.Steps) + "\n--- trace ---\n" + string.Join("\n", trace.TakeLast(80)));
            lock (trace) Assert.DoesNotContain(trace, l => l.Contains(token.Substring(17)));
        }
    }
}
