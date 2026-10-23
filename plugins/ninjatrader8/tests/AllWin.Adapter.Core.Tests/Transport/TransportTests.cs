using System;
using System.Threading;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Engine;
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
        [InlineData("ws://allwin.example/adapter/v1")]
        [InlineData("http://127.0.0.1/adapter/v1")]
        public void Connector_refuses_plain_text_endpoints_except_loopback(string uri)
        {
            using var loop = new EngineLoop(new AdapterEngine(new EngineOptions(), new SimulatedPlatform(), new InMemoryStore()));
            Assert.Throws<ArgumentException>(() => new WebSocketConnector(new Uri(uri), () => null, loop));
            using var ok = new WebSocketConnector(new Uri("ws://127.0.0.1:1/adapter/v1"), () => null, loop);
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
