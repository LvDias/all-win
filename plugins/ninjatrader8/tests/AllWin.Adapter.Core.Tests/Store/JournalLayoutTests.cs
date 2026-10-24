using System;
using System.IO;
using AllWin.Adapter.Core.Store;
using Xunit;

namespace AllWin.Adapter.Core.Tests.Store
{
    /// <summary>Pêndulo M1 finding: the journal (limits and their versions) is per Core origin.</summary>
    public class JournalLayoutTests : IDisposable
    {
        private static readonly Uri Fake = new Uri("ws://127.0.0.1:8765/adapter/v1");
        private static readonly Uri CoreA = new Uri("wss://100.64.0.10:8443/adapter/v1");
        private static readonly Uri CoreB = new Uri("wss://100.64.0.11:8443/adapter/v1");
        private readonly string _root = Path.Combine(Path.GetTempPath(), "allwin-layout-" + Guid.NewGuid().ToString("N"));

        public void Dispose()
        {
            if (Directory.Exists(_root)) Directory.Delete(_root, true);
        }

        [Fact]
        public void Fake_core_and_each_real_endpoint_get_their_own_directory()
        {
            Assert.Equal("fakecore", JournalLayout.DirName(Fake, fakeCore: true));
            var a = JournalLayout.DirName(CoreA, fakeCore: false);
            var b = JournalLayout.DirName(CoreB, fakeCore: false);
            Assert.Matches("^core-[0-9a-f]{12}$", a);
            Assert.NotEqual(a, b);
            Assert.NotEqual(a, JournalLayout.DirName(new Uri("wss://100.64.0.10:9443/adapter/v1"), false));
            Assert.NotEqual("fakecore", JournalLayout.DirName(Fake, fakeCore: false));
        }

        [Theory]
        [InlineData("WSS://100.64.0.10:8443/adapter/v1")]
        [InlineData("wss://100.64.0.10:8443/adapter/v1/")]
        [InlineData("wss://100.64.0.10:8443/Adapter/V1")]
        public void The_same_endpoint_spelled_differently_keeps_its_directory(string spelling)
        {
            Assert.Equal(JournalLayout.DirName(CoreA, false), JournalLayout.DirName(new Uri(spelling), false));
        }

        [Fact]
        public void Default_port_and_explicit_443_are_the_same_origin()
        {
            Assert.Equal(JournalLayout.DirName(new Uri("wss://core.example/adapter/v1"), false),
                JournalLayout.DirName(new Uri("wss://core.example:443/adapter/v1"), false));
        }

        [Fact]
        public void Limits_written_against_the_fake_core_are_not_seen_by_the_real_core()
        {
            var fakeDir = JournalLayout.DirFor(_root, Fake, fakeCore: true);
            var realDir = JournalLayout.DirFor(_root, CoreA, fakeCore: false);
            Directory.CreateDirectory(fakeDir);
            using (var fake = new FileJournalStore(fakeDir))
                fake.Commit(new[] { new StoreOp("lim", "acc-1", "{\"version\":17614512000000}") });

            Directory.CreateDirectory(realDir);
            using (var real = new FileJournalStore(realDir))
                Assert.Null(real.Get("lim", "acc-1"));
            using (var fakeAgain = new FileJournalStore(fakeDir))
                Assert.NotNull(fakeAgain.Get("lim", "acc-1"));
        }
    }
}
