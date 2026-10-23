using System;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;
using Xunit;
using static AllWin.Adapter.Core.Tests.Engine.EngineHarness;

namespace AllWin.Adapter.Core.Tests.Engine
{
    public class SessionTests
    {
        [Fact]
        public void Hello_is_first_and_lists_accounts_with_kind_and_oco_location()
        {
            using var h = new EngineHarness(p => p.AddAccount("Playback101", AccountKind.Playback, connection: "Playback Connection"));
            h.Engine.OnConnected(h.Sink);
            var first = h.Sink.Frames.First();
            Assert.Equal(MessageTypes.Hello, first.Type);
            var hello = first.PayloadAs<HelloPayload>();
            Assert.Equal(new[] { "1.0", "1.1" }, hello.SupportedProtocolVersions);
            Assert.Equal("1.0", first.ProtocolVersion); // nothing negotiated yet
            Assert.Equal(AdapterBuild.Development, hello.Adapter.Build);
            Assert.Contains(hello.Accounts, a => a.PlatformAccount == Sim101 && a.Kind == AccountKind.Simulation && a.OcoLocation == OcoLocation.Local);
            Assert.Contains(hello.Accounts, a => a.PlatformAccount == "Playback101" && a.Kind == AccountKind.Playback);
            Assert.Equal(SessionState.AwaitingWelcome, h.Engine.Session);
        }

        [Fact]
        public void Instance_id_survives_restart()
        {
            using var h = new EngineHarness();
            var id = h.Engine.InstanceId;
            h.Restart();
            Assert.Equal(id, h.Engine.InstanceId);
        }

        [Fact]
        public void Welcome_makes_the_session_ready_and_sends_the_mandatory_reconnect_snapshot()
        {
            using var h = new EngineHarness();
            h.Connect();
            Assert.Equal(SessionState.Ready, h.Engine.Session);
            var snapshot = Assert.Single(h.Snapshots());
            Assert.Equal(SnapshotTrigger.Reconnect, snapshot.Trigger);
            Assert.Equal(AccountId, Assert.Single(snapshot.Accounts).AccountId);
        }

        [Fact]
        public void Outbox_is_retransmitted_in_original_order_before_the_snapshot_on_reconnect()
        {
            using var h = new EngineHarness();
            h.Connect();
            h.Platform.SetPrice("MNQ", 21000m);
            h.Deliver(Place("0192b3c4-0000-7000-8000-000000000101", Side.Buy, 1));
            var before = h.Sink.Frames.Where(f => MessageTypes.IsReliable(f.Type)).Select(f => f.MsgId).ToList();
            Assert.True(before.Count >= 3);

            h.Disconnect();
            h.Connect();
            var after = h.Sink.Frames.Where(f => MessageTypes.IsReliable(f.Type)).ToList();
            Assert.Equal(before, after.Take(before.Count).Select(f => f.MsgId)); // same msg_ids, same order
            Assert.Equal(MessageTypes.PositionSnapshot, after[before.Count].Type);
        }

        [Fact]
        public void Acked_messages_are_not_retransmitted_and_unacked_are_after_the_ack_timeout()
        {
            using var h = new EngineHarness();
            h.Connect();
            var snapshotMsg = h.Sent(MessageTypes.PositionSnapshot).Single().MsgId;
            h.Tick(TimeSpan.FromSeconds(6));
            Assert.Equal(2, h.Sent(MessageTypes.PositionSnapshot).Count(f => f.MsgId == snapshotMsg));
            h.AckAll();
            Assert.Equal(0, h.Engine.PendingOutbound);
            h.Tick(TimeSpan.FromSeconds(6));
            Assert.Equal(2, h.Sent(MessageTypes.PositionSnapshot).Count(f => f.MsgId == snapshotMsg));
        }

        [Fact]
        public void Heartbeats_are_sent_and_three_missed_intervals_close_with_4005()
        {
            using var h = new EngineHarness();
            h.Connect();
            h.AckAll();
            h.Tick(TimeSpan.FromSeconds(10));
            Assert.Single(h.Sent(MessageTypes.Heartbeat));
            h.Tick(TimeSpan.FromSeconds(10));
            h.Tick(TimeSpan.FromSeconds(11));
            Assert.Contains(4005, h.Sink.Closes);
            Assert.Equal(SessionState.Offline, h.Engine.Session);
            Assert.True(h.Engine.ShouldReconnect);
        }

        [Theory]
        [InlineData(4001, StopReason.PairAgain)]
        [InlineData(4002, StopReason.PairAgain)]
        [InlineData(4003, StopReason.UpdatePlugin)]
        [InlineData(4004, StopReason.ConnectedElsewhere)]
        public void Terminal_close_codes_stop_reconnecting(int code, StopReason reason)
        {
            using var h = new EngineHarness();
            h.Connect();
            h.Disconnect(code);
            Assert.Equal(SessionState.Stopped, h.Engine.Session);
            Assert.Equal(reason, h.Engine.StopReason);
            Assert.False(h.Engine.ShouldReconnect);
        }

        [Theory]
        [InlineData(4000)]
        [InlineData(4005)]
        [InlineData(4006)]
        [InlineData(null)]
        public void Other_closes_reconnect(int? code)
        {
            using var h = new EngineHarness();
            h.Connect();
            h.Disconnect(code);
            Assert.Equal(SessionState.Offline, h.Engine.Session);
            Assert.True(h.Engine.ShouldReconnect);
        }

        [Fact]
        public void Invalid_frame_gets_an_error_and_no_ack()
        {
            using var h = new EngineHarness();
            h.Connect();
            var msgId = Ids.New();
            var frame = ProtocolCodec.ToJson(Place("0192b3c4-0000-7000-8000-000000000102", Side.Buy, 1));
            frame["place"]!["quantity"] = 0;
            h.DeliverRaw("{\"protocol_version\":\"1.0\",\"msg_id\":\"" + msgId + "\",\"type\":\"command\",\"sent_at\":\"2026-10-22T14:31:00Z\",\"payload\":" + frame.ToString(Newtonsoft.Json.Formatting.None) + "}");
            var error = Assert.Single(h.Sent(MessageTypes.Error)).PayloadAs<ErrorPayload>();
            Assert.Equal(ReasonCodes.ProtocolInvalidMessage, error.Code);
            Assert.Equal(msgId, error.RefMsgId);
            Assert.DoesNotContain(msgId, h.Acks());
            Assert.Empty(h.Platform.CallLog);
        }

        [Fact]
        public void Transport_duplicate_is_acked_again_and_not_processed_twice()
        {
            using var h = new EngineHarness();
            h.Connect();
            h.Platform.SetPrice("MNQ", 21000m);
            var command = Place("0192b3c4-0000-7000-8000-000000000103", Side.Buy, 1);
            var msgId = h.Deliver(command);
            h.Deliver(command, msgId);
            Assert.Equal(2, h.Acks().Count(a => a == msgId));
            Assert.Single(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal));
            Assert.DoesNotContain(h.Reports(), r => r.Status == ReportStatus.Duplicate);
        }

        [Fact]
        public void Business_duplicate_is_answered_duplicate_even_after_restart()
        {
            using var h = new EngineHarness();
            h.Connect();
            h.Platform.SetPrice("MNQ", 21000m);
            var command = Place("0192b3c4-0000-7000-8000-000000000104", Side.Buy, 1);
            h.Deliver(command);
            h.Restart();
            h.Connect();
            var second = h.Deliver(command); // new msg_id, same command_id
            Assert.True(h.Acks().Contains(second), string.Join("\n", h.Log));
            var duplicate = Assert.Single(h.Reports(command.CommandId), r => r.Status == ReportStatus.Duplicate);
            Assert.Equal(ReasonCodes.CommandDuplicate, duplicate.ReasonCode);
            Assert.Equal(ChildRole.Entry, duplicate.ChildRole);
            Assert.NotNull(duplicate.PlatformOrderId);
            Assert.Single(h.Platform.CallLog, c => c.StartsWith("submit aw:" + command.CommandId + ":e", StringComparison.Ordinal));
        }

        [Fact]
        public void Command_before_the_snapshot_is_queued_then_processed_after_it()
        {
            using var h = new EngineHarness();
            h.Connect();
            h.Disconnect();
            h.Platform.SetPrice("MNQ", 21000m);
            h.Engine.OnConnected(h.Sink);
            var command = Place("0192b3c4-0000-7000-8000-000000000105", Side.Buy, 1);
            h.Deliver(command); // protocol violation by the Core: queued, never rejected
            Assert.Empty(h.Platform.CallLog);
            Assert.Contains(h.Log, l => l.Contains("protocol.message_before_snapshot"));
            h.Deliver(DefaultWelcome());
            var frames = h.Sink.Frames.Select(f => f.Type).ToList();
            Assert.Single(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal));
            var snapshotIndex = h.Sink.Frames.FindLastIndex(f => f.Type == MessageTypes.PositionSnapshot);
            var workingIndex = h.Sink.Frames.FindIndex(f => f.Type == MessageTypes.ExecutionReport && f.PayloadAs<ExecutionReportPayload>().Status == ReportStatus.Working);
            Assert.True(snapshotIndex < workingIndex);
        }

        [Fact]
        public void Newer_minor_message_type_is_answered_unsupported_value()
        {
            using var h = new EngineHarness();
            h.Connect();
            h.DeliverRaw("{\"protocol_version\":\"1.2\",\"msg_id\":\"" + Ids.New() + "\",\"type\":\"account_limits_v2\",\"sent_at\":\"2026-10-22T14:31:00Z\",\"payload\":{}}");
            Assert.Equal(ReasonCodes.ProtocolUnsupportedValue, Assert.Single(h.Sent(MessageTypes.Error)).PayloadAs<ErrorPayload>().Code);
        }

        [Fact]
        public void Limits_are_stored_by_monotonic_version_and_acked()
        {
            using var h = new EngineHarness();
            h.Connect();
            var limits = new LimitsPayload
            {
                Accounts =
                {
                    new AccountLimits
                    {
                        AccountId = AccountId, PlatformAccount = Sim101, Version = 3, EnforcementEnabled = true, MaxPosition = 5,
                        TradingDay = new TradingDaySpec { ResetLocalTime = "17:00", TimeZone = "America/Chicago" }, WarningThresholdPct = 80,
                        BlockedUntil = h.Clock.UtcNow.AddHours(2),
                    },
                },
            };
            var msgId = h.Deliver(limits);
            Assert.Contains(msgId, h.Acks());
            limits.Accounts[0].Version = 2;
            limits.Accounts[0].BlockedUntil = null; // older version: ignored, so the block stays
            h.Deliver(limits);
            h.Restart();
            h.Connect();
            h.Deliver(Place("0192b3c4-0000-7000-8000-000000000106", Side.Buy, 1, expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            Assert.Equal(ReasonCodes.RiskAccountBlocked, h.Reports().Last().ReasonCode);
        }
    }
}
