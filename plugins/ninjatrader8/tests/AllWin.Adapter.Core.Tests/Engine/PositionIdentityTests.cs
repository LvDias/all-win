using System;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;
using Newtonsoft.Json.Linq;
using Xunit;
using static AllWin.Adapter.Core.Tests.Engine.EngineHarness;

namespace AllWin.Adapter.Core.Tests.Engine
{
    /// <summary>Protocol 1.2 position-identity guard on reduce (ADR-0004 §5, U13 option A).</summary>
    public class PositionIdentityTests
    {
        private const string C1 = "0192b3c4-0000-7000-8000-000000000a01";
        private const string C2 = "0192b3c4-0000-7000-8000-000000000a02";
        private const string C3 = "0192b3c4-0000-7000-8000-000000000a03";
        private const string R1 = "0192b3c4-0000-7000-8000-000000000a11";

        private static EngineHarness Ready(WelcomePayload? welcome = null)
        {
            var h = new EngineHarness();
            h.Platform.SetPrice("MNQ", 21000m);
            h.Connect(welcome);
            return h;
        }

        private static ExecutionReportPayload Final(EngineHarness h, string commandId) =>
            h.Reports(commandId).Last(r => r.Status != ReportStatus.Received);

        private static void AssertPositionChanged(EngineHarness h, string commandId)
        {
            var report = Final(h, commandId);
            Assert.Equal(ReportStatus.Rejected, report.Status);
            Assert.Equal(ReasonCodes.CommandPositionChanged, report.ReasonCode);
            Assert.Equal(ChildRole.Exit, report.ChildRole);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.Contains(commandId));
        }

        private static void AssertReduced(EngineHarness h, string commandId, int positionAfter)
        {
            Assert.Contains(h.Reports(commandId), r => r.Status == ReportStatus.Filled && r.ChildRole == ChildRole.Exit);
            Assert.Equal(positionAfter, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Reduce_whose_position_ref_is_the_opening_command_executes()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2));
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertReduced(h, R1, 1);
        }

        [Fact]
        public void Adds_and_partial_closes_keep_the_identity()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Place(C2, Side.Buy, 2)); // an All Win add
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m); // an external add (Vigia I2: reducing it only lowers exposure)
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 21000m); // an external partial close
            h.Settle();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertReduced(h, R1, 2);
        }

        [Fact]
        public void Reduce_naming_the_second_entry_is_refused_the_identity_is_the_first_fill_from_flat()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Place(C2, Side.Buy, 1));
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C2));
            AssertPositionChanged(h, R1);
            Assert.Equal(2, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Position_closed_and_reopened_by_another_entry_is_a_different_position()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Flatten(C2));
            h.Deliver(Place(C3, Side.Buy, 1));
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1)); // late reduce of the first position
            AssertPositionChanged(h, R1);
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Externally_opened_position_never_matches()
        {
            using var h = Ready();
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 2, 21000m);
            h.Settle();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertPositionChanged(h, R1);
            Assert.Equal(2, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void A_flip_by_an_external_fill_makes_the_identity_external()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 2, 21000m);
            h.Settle();
            h.Deliver(Place(R1, Side.Buy, 1, PositionEffect.Reduce, positionRef: C1));
            AssertPositionChanged(h, R1);
            Assert.Equal(-1, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void A_flip_by_an_All_Win_entry_takes_that_entry_as_identity()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Place(C2, Side.Sell, 2));
            h.Deliver(Place(R1, Side.Buy, 1, PositionEffect.Reduce, positionRef: C2));
            AssertReduced(h, R1, 0);
        }

        [Fact]
        public void Same_side_order_is_position_changed_flat_stays_nothing_to_reduce()
        {
            using var h = Ready();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            Assert.Equal(ReasonCodes.CommandNothingToReduce, Final(h, R1).ReasonCode);
            Assert.Equal(ReportStatus.Cancelled, Final(h, R1).Status);

            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Place(C2, Side.Buy, 1, PositionEffect.Reduce, positionRef: C1));
            AssertPositionChanged(h, C2);
        }

        [Theory]
        [InlineData("1.0")]
        [InlineData("1.1")]
        public void Reduce_in_a_pre_1_2_envelope_is_position_changed_and_flat_is_still_nothing_to_reduce(string version)
        {
            var welcome = DefaultWelcome();
            welcome.ProtocolVersion = version;
            using var h = Ready(welcome);
            var flatReduce = Place(R1, Side.Sell, 1, PositionEffect.Reduce);
            flatReduce.Refs = null; // an older Core cannot send refs
            h.Deliver(flatReduce, version: version);
            Assert.Equal(ReasonCodes.CommandNothingToReduce, Final(h, R1).ReasonCode);

            h.Deliver(Place(C1, Side.Buy, 1), version: version);
            var reduce = Place(C2, Side.Sell, 1, PositionEffect.Reduce);
            reduce.Refs = null;
            h.Deliver(reduce, version: version);
            AssertPositionChanged(h, C2);
        }

        [Fact]
        public void Reduce_in_a_1_2_envelope_without_refs_is_invalid_not_acked_never_executed()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            var reduce = Place(R1, Side.Sell, 1, PositionEffect.Reduce);
            reduce.Refs = null;
            var msgId = Ids.New();
            var frame = new JObject
            {
                ["protocol_version"] = "1.2",
                ["msg_id"] = msgId,
                ["type"] = MessageTypes.Command,
                ["sent_at"] = UtcTimestamp.Format(h.Clock.UtcNow),
                ["payload"] = ProtocolCodec.ToJson(reduce),
            };
            h.DeliverRaw(frame.ToString(Newtonsoft.Json.Formatting.None));
            var error = h.Sent(MessageTypes.Error).Single().PayloadAs<ErrorPayload>();
            Assert.Equal(ReasonCodes.ProtocolInvalidMessage, error.Code);
            Assert.Equal(msgId, error.RefMsgId);
            Assert.DoesNotContain(msgId, h.Acks());
            Assert.Empty(h.Reports(R1));
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Identity_survives_a_restart()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2));
            h.AckAll();
            h.Restart();
            h.Connect();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertReduced(h, R1, 1);
        }

        [Fact]
        public void Closed_and_reopened_the_same_size_while_down_with_executions_kept_is_external()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.AckAll();
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 21000m);
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m);
            h.Restart(); // the plugin was down: its events are lost, catch-up replays the executions
            h.Connect();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertPositionChanged(h, R1);
        }

        [Fact]
        public void Closed_and_reopened_the_same_size_while_down_without_executions_is_unknown()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.AckAll();
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 21000m);
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m);
            h.Platform.ForgetExecutions(Sim101); // NT8 did not keep them across its restart
            h.Restart();
            h.Connect();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertPositionChanged(h, R1);
            Assert.Contains(h.Log, l => l.Contains("identity.unknown"));
        }

        [Fact]
        public void Lost_journal_without_the_opening_execution_is_unknown()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2));
            h.Platform.ForgetExecutions(Sim101);
            h.RestartWithLostJournal();
            h.Connect();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertPositionChanged(h, R1);
        }

        [Fact]
        public void Account_reset_makes_the_identity_unknown()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Platform.ResetAccount(Sim101);
            h.Settle();
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m); // e.g. a Playback rewind re-opens a position
            h.Settle();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertPositionChanged(h, R1);
        }

        [Theory]
        [InlineData(true)]
        [InlineData(false)]
        public void Correction_of_the_opening_fill_makes_the_identity_unknown(bool remove)
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2));
            var opening = h.Reports(C1).Single(r => r.Fill != null).Fill!.FillId;
            if (remove)
            {
                h.Deliver(Place(C2, Side.Buy, 1)); // keeps a position after the removal
                h.Platform.RemoveExecution(Sim101, opening);
            }
            else h.Platform.AmendExecution(Sim101, opening, 3, 21000m);
            h.Settle();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertPositionChanged(h, R1);
        }

        [Fact]
        public void Live_position_ahead_of_the_record_fails_closed()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Platform.EventSink = _ => { }; // the next execution is still queued when the reduce runs
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m);
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertPositionChanged(h, R1);
        }

        [Fact]
        public void Step_0_a_redelivered_reduce_after_a_lost_journal_reports_what_already_happened()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            AssertReduced(h, R1, 0);
            h.RestartWithLostJournal(); // dedupe records gone; the Core never got our reports
            h.Connect();
            h.Platform.CallLog.Clear();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal));
            var reports = h.Reports(R1);
            Assert.DoesNotContain(reports, r => r.ReasonCode == ReasonCodes.CommandNothingToReduce || r.ReasonCode == ReasonCodes.CommandPositionChanged);
            Assert.Contains(h.Log, l => l.Contains("order.adopted"));
        }

        [Fact]
        public void Step_0_two_matches_are_ambiguous_and_nothing_is_submitted()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2));
            var name = "aw:" + R1 + ":x";
            for (var i = 0; i < 2; i++)
                h.Platform.Submit(Sim101, new[] { new Platform.OrderRequest { Name = name, Instrument = Mnq, Side = Side.Sell, Type = OrderType.Limit, Quantity = 1, LimitPrice = 22000m } });
            h.Settle();
            h.Platform.CallLog.Clear();
            h.Deliver(Place(R1, Side.Sell, 1, PositionEffect.Reduce, positionRef: C1));
            Assert.Equal(ReasonCodes.OrderAmbiguousReference, Final(h, R1).ReasonCode);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal));
        }

        [Fact]
        public void Identity_record_is_written_in_the_fill_batch_and_removed_when_flat()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Flatten(C2));
            h.Deliver(Place(C3, Side.Sell, 1));
            h.Deliver(Place(R1, Side.Buy, 1, PositionEffect.Reduce, positionRef: C3));
            AssertReduced(h, R1, 0);
            Assert.DoesNotContain(h.Log, l => l.Contains("identity.unknown"));
        }
    }
}
