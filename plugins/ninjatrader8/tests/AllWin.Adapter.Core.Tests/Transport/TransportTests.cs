using System;
using System.Threading;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;
using AllWin.Adapter.Core.Store;
using AllWin.Adapter.Core.Transport;
using AllWin.Adapter.Sim;
using Xunit;

namespace AllWin.Adapter.Core.Tests.Transport
{
    public class TransportTests
    {
        [Theory]
        [InlineData(0, 1.0)]
        [InlineData(1, 2.0)]
        [InlineData(2, 4.0)]
        [InlineData(4, 16.0)]
        [InlineData(5, 30.0)]
        [InlineData(50, 30.0)]
        public void Backoff_doubles_from_one_second_and_caps_at_thirty(int attempt, double seconds)
        {
            Assert.Equal(seconds, new Backoff(() => 0.5).Delay(attempt).TotalSeconds, 6);
        }

        [Fact]
        public void Backoff_jitter_stays_within_twenty_percent()
        {
            Assert.Equal(16.0 * 0.8, new Backoff(() => 0.0).Delay(4).TotalSeconds, 6);
            Assert.Equal(16.0 * 1.2, new Backoff(() => 1.0).Delay(4).TotalSeconds, 6);
        }

        [Theory]
        [InlineData("ws://allwin.example/adapter/v1", AdapterBuild.Development)]
        [InlineData("http://127.0.0.1/adapter/v1", AdapterBuild.Development)]
        [InlineData("ws://127.0.0.1:1/adapter/v1", AdapterBuild.Release)] // QA Security I2
        public void Connector_refuses_plain_text_endpoints_except_loopback_in_development(string uri, AdapterBuild build)
        {
            using var loop = new EngineLoop(new AdapterEngine(new EngineOptions(), new SimulatedPlatform(), new InMemoryStore()));
            Assert.Throws<ArgumentException>(() => new WebSocketConnector(new Uri(uri), () => null, loop, build));
            using var dev = new WebSocketConnector(new Uri("ws://127.0.0.1:1/adapter/v1"), () => null, loop, AdapterBuild.Development);
            using var release = new WebSocketConnector(new Uri("wss://allwin.example/adapter/v1"), () => null, loop, AdapterBuild.Release);
        }

        [Fact]
        public async Task Connector_reports_a_pin_mismatch_until_another_outcome_replaces_it()
        {
            using var loop = new EngineLoop(new AdapterEngine(new EngineOptions(), new SimulatedPlatform(), new InMemoryStore()));
            var attempts = 0;
            WebSocketFactory refuse = (uri, token, ct) =>
            {
                var n = Interlocked.Increment(ref attempts);
                throw n == 1
                    ? new TransportRefusedException("server certificate does not match the configured pin", null, TransportProblem.PinMismatch)
                    : new TransportRefusedException("upgrade refused: HTTP 503");
            };
            using var connector = new WebSocketConnector(new Uri("wss://allwin.example/adapter/v1"), () => "awd_AbCdEfGhIjKl_" + new string('x', 43), loop,
                AdapterBuild.Development, null, new Backoff(() => 0.0), refuse);
            Assert.Equal(TransportProblem.None, connector.LastProblem);
            connector.Start();
            Assert.True(await Until(() => connector.LastProblem == TransportProblem.PinMismatch || Volatile.Read(ref attempts) > 1));
            Assert.Equal(TransportProblem.PinMismatch, connector.LastProblem);
            Assert.True(await Until(() => Volatile.Read(ref attempts) >= 2 && connector.LastProblem == TransportProblem.None));
        }

        private static async Task<bool> Until(Func<bool> condition)
        {
            for (var i = 0; i < 100; i++)
            {
                if (condition()) return true;
                await Task.Delay(50);
            }
            return condition();
        }

        [Theory]
        [InlineData("awd_AbCdEfGhIjKl_0123456789abcdefghijABCDEFGHIJ-_0123456789a", true)]
        [InlineData("awd_AbCdEfGhIjK_0123456789abcdefghijABCDEFGHIJ-_0123456789a", false)]       // key id of 11
        [InlineData("awd_AbCdEfGhIjKl_0123456789abcdefghijABCDEFGHIJ-_0123456789", false)]  // secret of 42
        [InlineData("awk_AbCdEfGhIjKl_0123456789abcdefghijABCDEFGHIJ-_0123456789a", false)]      // ingress key prefix
        [InlineData("awd_AbCdEfGhIjKl_0123456789abcdefghijABCDEFGHIJ+/0123456789a", false)]      // base64, not base64url
        [InlineData("awd_AbCdEfGhIjKl_0123456789abcdefghijABCDEFGHIJ-_0123456789a\nx", false)]  // a second line
        [InlineData("awd_AbCdEfGhIjKl_0123456789abcdefghijABCDEFGHIJ-_0123456789a\n", false)]   // trailing newline ($ would accept it)
        [InlineData("", false)]
        [InlineData(null, false)]
        public void Device_token_format_matches_the_pairing_contract(string? token, bool expected)
        {
            Assert.Equal(expected, DeviceTokenFormat.IsWellFormed(token));
        }

        [Fact]
        public void Token_mask_keeps_only_the_key_id()
        {
            var token = "awd_AbCdEfGhIjKl_" + new string('x', 43);
            Assert.Equal("awd_AbCdEfGhIjKl_***", TokenMask.Mask(token));
            Assert.Equal("***", TokenMask.Mask("garbage"));
            Assert.DoesNotContain("xxxx", TokenMask.Mask(token));
        }

        [Fact]
        public async Task Engine_loop_runs_work_on_one_thread_in_order()
        {
            using var loop = new EngineLoop(new AdapterEngine(new EngineOptions(), new SimulatedPlatform(), new InMemoryStore()));
            var threads = new System.Collections.Concurrent.ConcurrentBag<int>();
            var order = new System.Collections.Concurrent.ConcurrentQueue<int>();
            Parallel.For(0, 4, _ => loop.Post(e => threads.Add(Environment.CurrentManagedThreadId)));
            for (var i = 0; i < 100; i++)
            {
                var n = i;
                loop.Post(e => order.Enqueue(n));
            }
            var started = await loop.InvokeAsync(e => e.InstanceId);
            Assert.False(string.IsNullOrEmpty(started)); // Start ran before the query
            Assert.Single(System.Linq.Enumerable.Distinct(threads));
            Assert.Equal(System.Linq.Enumerable.Range(0, 100), order);
        }
    }
}
