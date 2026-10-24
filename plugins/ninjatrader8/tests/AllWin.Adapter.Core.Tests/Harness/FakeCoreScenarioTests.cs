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
            if ((bool?)scenario["nt8_only"] ?? false) return; // behaviour of the NT8 simulator itself (e.g. its rejections)
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

        [Fact]
        public async Task Half_open_connection_is_detected_by_heartbeat_and_the_adapter_reconnects()
        {
            // Code Review 6 M2: the peer stops answering while TCP stays up. Heartbeat 1 s → timeout after 3 s,
            // then the close handshake gets a short grace and the socket is aborted.
            var scenario = ProtocolJson.ParseObject("{\"name\":\"half-open\",\"accounts\":[{\"alias\":\"A\",\"platform_account\":\"Sim101\"}],\"steps\":[]}");
            var token = "awd_TestKeyId001_" + new string('b', 43);
            await using var core = new FakeCoreServer(ScenarioRunner.AccountsOf(scenario), token) { HeartbeatIntervalMs = 1000 };
            await core.StartAsync();
            using var sim = new SimAdapterHost(scenario, core.Endpoint!, token, _ => { });
            sim.Start();
            await WaitAsync(() => core.SessionReady, 10000);
            core.FreezeCurrentSession();
            var watch = System.Diagnostics.Stopwatch.StartNew();
            await WaitAsync(() => core.SessionCount >= 2 && core.SessionReady, 20000);
            Assert.True(watch.Elapsed < System.TimeSpan.FromSeconds(15), "reconnected after " + watch.Elapsed);
        }

        private static async Task WaitAsync(System.Func<bool> condition, int timeoutMs)
        {
            var watch = System.Diagnostics.Stopwatch.StartNew();
            while (!condition())
            {
                Assert.True(watch.ElapsedMilliseconds < timeoutMs, "timed out");
                await Task.Delay(50);
            }
        }
    }
}
