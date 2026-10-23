using System;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using AllWin.Adapter.Core.Protocol;
using Newtonsoft.Json.Linq;
using Xunit;

namespace AllWin.Adapter.Core.Tests.Contracts
{
    /// <summary>Valid fixtures decode into typed models and re-encode losslessly; invalid ones are refused.</summary>
    public class CodecConformanceTests
    {
        private static readonly Regex Timestamp = new Regex(@"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?Z$");

        [Theory]
        [MemberData(nameof(SchemaConformanceTests.AdapterFixtures), MemberType = typeof(SchemaConformanceTests))]
        public void Fixture_decodes_according_to_its_folder(string kind, string file)
        {
            var text = File.ReadAllText(Path.Combine(RepoPaths.AdapterExamples, kind, file));
            var result = ProtocolCodec.Decode(text);
            if (kind == "invalid")
            {
                Assert.False(result.IsOk);
                Assert.StartsWith("protocol.", result.ErrorCode);
                return;
            }

            Assert.True(result.IsOk, result.Detail);
            var envelope = result.Envelope!;
            var model = ToTyped(envelope);
            var reencoded = ProtocolJson.ParseObject(ProtocolCodec.Encode(model, envelope.MsgId, envelope.SentAt, envelope.ProtocolVersion));
            AssertSemanticallyEqual(ProtocolJson.Parse(text), reencoded, "");
        }

        [Fact]
        public void Newer_minor_with_unknown_type_is_unsupported_value()
        {
            var frame = ProtocolJson.ParseObject(File.ReadAllText(Path.Combine(RepoPaths.AdapterExamples, "valid", "heartbeat.json")));
            frame["protocol_version"] = "1.2";
            frame["type"] = "account_limits_v2";
            var result = ProtocolCodec.Decode(ProtocolJson.Serialize(frame));
            Assert.Equal(ReasonCodes.ProtocolUnsupportedValue, result.ErrorCode);
            Assert.Equal((string?)frame["msg_id"], result.RefMsgId);
        }

        [Fact]
        public void Price_as_number_is_refused_even_if_schema_were_bypassed()
        {
            var payload = JObject.Parse("{\"type\":\"limit\",\"limit_price\":21000.25}");
            Assert.ThrowsAny<Exception>(() => ProtocolCodec.ToModel<OrderSpec>(payload));
        }

        [Fact]
        public void Oversized_frame_is_refused()
        {
            var result = ProtocolCodec.Decode("{\"x\":\"" + new string('a', ProtocolJson.MaxFrameBytes) + "\"}");
            Assert.Equal(ReasonCodes.ProtocolInvalidMessage, result.ErrorCode);
        }

        [Fact]
        public void Encoding_an_invalid_payload_throws()
        {
            var report = new ExecutionReportPayload
            {
                EventId = Guid.NewGuid().ToString(), AccountId = Guid.NewGuid().ToString(), PlatformAccount = "Sim101",
                Instrument = new Instrument("MNQ", "2026-12"), Origin = OrderOrigin.AllWin, Status = ReportStatus.Filled,
                CommandId = Guid.NewGuid().ToString(), OccurredAt = DateTime.UtcNow,
            };
            Assert.Throws<InvalidOperationException>(() => ProtocolCodec.Encode(report, Guid.NewGuid().ToString(), DateTime.UtcNow));
        }

        [Fact]
        public void Timestamps_round_trip_and_truncate_beyond_100ns()
        {
            var t = UtcTimestamp.Parse("2026-10-22T14:31:06.123456789Z");
            Assert.Equal(DateTimeKind.Utc, t.Kind);
            Assert.Equal(new DateTime(2026, 10, 22, 14, 31, 6, DateTimeKind.Utc).AddTicks(1234567), t);
            Assert.Equal("2026-10-22T14:31:06.123Z", UtcTimestamp.Format(t));
        }

        internal static object ToTyped(Envelope e) => e.Type switch
        {
            MessageTypes.Hello => e.PayloadAs<HelloPayload>(),
            MessageTypes.Welcome => e.PayloadAs<WelcomePayload>(),
            MessageTypes.Error => e.PayloadAs<ErrorPayload>(),
            MessageTypes.Ack => e.PayloadAs<AckPayload>(),
            MessageTypes.Heartbeat => e.PayloadAs<HeartbeatPayload>(),
            MessageTypes.Command => e.PayloadAs<CommandPayload>(),
            MessageTypes.Limits => e.PayloadAs<LimitsPayload>(),
            MessageTypes.InstructionResult => e.PayloadAs<InstructionResultPayload>(),
            MessageTypes.ExecutionReport => e.PayloadAs<ExecutionReportPayload>(),
            MessageTypes.PositionSnapshot => e.PayloadAs<PositionSnapshotPayload>(),
            MessageTypes.Instruction => e.PayloadAs<InstructionPayload>(),
            MessageTypes.RiskEvent => e.PayloadAs<RiskEventPayload>(),
            MessageTypes.RiskState => e.PayloadAs<RiskStatePayload>(),
            MessageTypes.AccountState => e.PayloadAs<AccountStatePayload>(),
            _ => throw new InvalidOperationException(e.Type),
        };

        private static void AssertSemanticallyEqual(JToken expected, JToken actual, string path)
        {
            if (expected is JObject eo)
            {
                var ao = Assert.IsType<JObject>(actual);
                Assert.True(eo.Properties().Select(p => p.Name).OrderBy(n => n).SequenceEqual(ao.Properties().Select(p => p.Name).OrderBy(n => n)),
                    $"{path}: keys {string.Join(",", eo.Properties().Select(p => p.Name))} vs {string.Join(",", ao.Properties().Select(p => p.Name))}");
                foreach (var p in eo.Properties()) AssertSemanticallyEqual(p.Value, ao[p.Name]!, path + "/" + p.Name);
                return;
            }
            if (expected is JArray ea)
            {
                var aa = Assert.IsType<JArray>(actual);
                Assert.Equal(ea.Count, aa.Count);
                for (var i = 0; i < ea.Count; i++) AssertSemanticallyEqual(ea[i], aa[i], path + "/" + i);
                return;
            }
            if (expected.Type == JTokenType.String && Timestamp.IsMatch((string)expected!))
            {
                Assert.Equal(UtcTimestamp.Parse((string)expected!), UtcTimestamp.Parse((string)actual!));
                return;
            }
            Assert.True(JToken.DeepEquals(expected, actual), $"{path}: {expected} vs {actual}");
        }
    }
}
