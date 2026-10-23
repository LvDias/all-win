using System;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;
using Xunit;
using static AllWin.Adapter.Core.Tests.Engine.EngineHarness;

namespace AllWin.Adapter.Core.Tests.Engine
{
    /// <summary>ADR-0004 §4 "Bounded re-delivery" (Code Review 4): retention 7 d &gt; staleness 6 d &gt; Core horizon 72 h.</summary>
    public class StalenessTests
    {
        private const string C1 = "0192b3c4-0000-7000-8000-000000000501";

        [Fact]
        public void Redelivery_after_dedupe_records_were_pruned_is_stale_and_not_executed_again()
        {
            using var h = new EngineHarness();
            h.Platform.SetPrice("MNQ", 21000m);
            h.Connect();
            // An exit (no expires_at) is the case only dedupe protects.
            h.Deliver(Place("0192b3c4-0000-7000-8000-000000000500", Side.Buy, 1));
            var exit = Place(C1, Side.Sell, 1, PositionEffect.Reduce, positionRef: "0192b3c4-0000-7000-8000-000000000500");
            var msgId = Ids.New();
            var sentAt = h.Clock.UtcNow;
            var frame = ProtocolCodec.Encode(exit, msgId, sentAt, CoreVersion);
            h.DeliverRaw(frame); // executed; our ack is "lost"
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));

            h.Tick(TimeSpan.FromDays(8)); // pruning runs: msg_id and command_id records are gone
            h.Disconnect();
            h.Connect();
            h.DeliverRaw(frame); // byte-identical retransmission

            Assert.Single(h.Platform.CallLog, c => c.Contains(C1));
            var report = h.Reports(C1).Last();
            Assert.True(report.Status == ReportStatus.Rejected, string.Join(",", h.Reports(C1).Select(r => r.Status + "/" + r.ReasonCode)) + "\n" + string.Join("\n", h.Log.Where(l => !l.StartsWith("Debug"))));
            Assert.Equal(ReasonCodes.CommandStale, report.ReasonCode);
            Assert.Contains(msgId, h.Acks()); // acked again
        }

        [Fact]
        public void Known_command_id_is_answered_duplicate_even_when_stale()
        {
            using var h = new EngineHarness();
            h.Platform.SetPrice("MNQ", 21000m);
            h.Connect();
            var exit = Place(C1, Side.Sell, 1, PositionEffect.Reduce);
            h.Deliver(exit, sentAt: h.Clock.UtcNow);
            h.Clock.Advance(TimeSpan.FromDays(6.5)); // inside retention, past staleness
            h.Deliver(exit, sentAt: h.Clock.UtcNow - TimeSpan.FromDays(6.5));
            // Architect answer E: a known command_id is a duplicate (non-transition), stale is only for unknown ones.
            Assert.Equal(ReportStatus.Duplicate, h.Reports(C1).Last().Status);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.Contains(C1)); // flat at first: nothing_to_reduce, never executed
        }

        [Fact]
        public void New_command_older_than_six_days_is_recorded_and_rejected()
        {
            using var h = new EngineHarness();
            h.Connect();
            h.Deliver(Flatten(C1), sentAt: h.Clock.UtcNow - TimeSpan.FromDays(6) - TimeSpan.FromSeconds(1));
            Assert.Equal(ReasonCodes.CommandStale, h.Reports(C1).Single().ReasonCode);
            h.Deliver(Flatten(C1), sentAt: h.Clock.UtcNow);
            Assert.Equal(ReportStatus.Duplicate, h.Reports(C1).Last().Status); // recorded as handled
            Assert.Empty(h.Platform.CallLog);
        }

        [Fact]
        public void Staleness_uses_the_skew_corrected_clock()
        {
            using var h = new EngineHarness();
            h.Connect(coreClockAhead: TimeSpan.FromMinutes(2));
            // Locally only 5d23h59m old, but 6 d + 1 min on the Core clock.
            h.Deliver(Flatten(C1), sentAt: h.Clock.UtcNow - TimeSpan.FromDays(6) + TimeSpan.FromMinutes(1));
            Assert.Equal(ReasonCodes.CommandStale, h.Reports(C1).Single().ReasonCode);
        }
    }
}
