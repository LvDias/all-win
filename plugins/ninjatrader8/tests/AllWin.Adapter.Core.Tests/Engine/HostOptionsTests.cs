using AllWin.Adapter.Core.Engine;
using Xunit;

namespace AllWin.Adapter.Core.Tests.Engine
{
    /// <summary>Host-provided options (0.1.1): the instance id shared with the pairing tool.</summary>
    public class HostOptionsTests
    {
        private const string Id = "0192b3c4-0000-7000-8000-00000000f001";

        [Fact]
        public void A_new_journal_takes_the_host_instance_id_and_keeps_it()
        {
            using var h = new EngineHarness(options: new EngineOptions { InstanceId = Id });
            Assert.Equal(Id, h.Engine.InstanceId);
            h.Restart();
            Assert.Equal(Id, h.Engine.InstanceId);
        }

        [Fact]
        public void Without_a_host_instance_id_one_is_generated()
        {
            using var h = new EngineHarness();
            Assert.False(string.IsNullOrEmpty(h.Engine.InstanceId));
            Assert.NotEqual(Id, h.Engine.InstanceId);
        }
    }
}
