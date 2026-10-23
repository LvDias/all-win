using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Text;
using AllWin.Adapter.Core.Protocol.Schema;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json.Serialization;

namespace AllWin.Adapter.Core.Protocol
{
    /// <summary>
    /// Frame (de)serialization. Every inbound and outbound frame is validated against the canonical
    /// schema: invalid input is refused with a stable code, and an invalid outbound frame is a bug
    /// that throws instead of reaching the Core.
    /// </summary>
    public static class ProtocolCodec
    {
        /// <summary>Envelope version used before a version is negotiated (and for hello).</summary>
        public const string Version = "1.0";
        public const int Major = 1;

        /// <summary>Highest minor this adapter implements.</summary>
        public const int Minor = 2;

        /// <summary>Versions advertised in hello, lowest first (Architect answer F).</summary>
        public static readonly string[] SupportedVersions = { "1.0", "1.1", "1.2" };

        private static readonly JsonSerializer Serializer = JsonSerializer.Create(new JsonSerializerSettings
        {
            ContractResolver = new DefaultContractResolver { NamingStrategy = new SnakeCaseNamingStrategy() },
            NullValueHandling = NullValueHandling.Ignore,
            MissingMemberHandling = MissingMemberHandling.Error,
            DateParseHandling = DateParseHandling.None,
            FloatParseHandling = FloatParseHandling.Decimal,
            Culture = CultureInfo.InvariantCulture,
            Converters =
            {
                new DecimalStringConverter(),
                new UtcTimestampConverter(),
                new StringEnumConverter(new SnakeCaseNamingStrategy(), allowIntegerValues: false),
            },
        });

        private static readonly Dictionary<string, Type> PayloadTypes = new Dictionary<string, Type>(StringComparer.Ordinal)
        {
            [MessageTypes.Hello] = typeof(HelloPayload),
            [MessageTypes.Welcome] = typeof(WelcomePayload),
            [MessageTypes.Error] = typeof(ErrorPayload),
            [MessageTypes.Ack] = typeof(AckPayload),
            [MessageTypes.Heartbeat] = typeof(HeartbeatPayload),
            [MessageTypes.Command] = typeof(CommandPayload),
            [MessageTypes.Limits] = typeof(LimitsPayload),
            [MessageTypes.InstructionResult] = typeof(InstructionResultPayload),
            [MessageTypes.ExecutionReport] = typeof(ExecutionReportPayload),
            [MessageTypes.PositionSnapshot] = typeof(PositionSnapshotPayload),
            [MessageTypes.Instruction] = typeof(InstructionPayload),
            [MessageTypes.RiskEvent] = typeof(RiskEventPayload),
            [MessageTypes.RiskState] = typeof(RiskStatePayload),
            [MessageTypes.AccountState] = typeof(AccountStatePayload),
        };

        public static string TypeOf(object payload)
        {
            foreach (var pair in PayloadTypes)
                if (pair.Value == payload.GetType()) return pair.Key;
            throw new ArgumentException("Not a protocol payload: " + payload.GetType().Name);
        }

        public static T ToModel<T>(JObject payload) => payload.ToObject<T>(Serializer)!;

        public static JObject ToJson(object payload) => JObject.FromObject(payload, Serializer);

        /// <summary>Builds and validates an outbound frame. Throws on a contract violation (a bug).</summary>
        public static string Encode(object payload, string msgId, DateTime sentAt, string protocolVersion = Version)
        {
            var frame = new JObject
            {
                ["protocol_version"] = protocolVersion,
                ["msg_id"] = msgId,
                ["type"] = TypeOf(payload),
                ["sent_at"] = UtcTimestamp.Format(sentAt),
                ["payload"] = ToJson(payload),
            };
            return EncodeFrame(frame);
        }

        public static string EncodeFrame(JObject frame)
        {
            var errors = ProtocolSchemas.ValidateFrame(frame);
            if (errors.Count > 0)
                throw new InvalidOperationException("Outbound frame violates the contract: " + string.Join("; ", errors.Take(5)));
            var text = ProtocolJson.Serialize(frame);
            if (Encoding.UTF8.GetByteCount(text) > ProtocolJson.MaxFrameBytes)
                throw new InvalidOperationException("Outbound frame exceeds 256 KiB.");
            return text;
        }

        public static DecodeResult Decode(string text)
        {
            if (Encoding.UTF8.GetByteCount(text) > ProtocolJson.MaxFrameBytes)
                return DecodeResult.Fail(ReasonCodes.ProtocolInvalidMessage, null, "frame exceeds 256 KiB");

            JObject frame;
            try { frame = ProtocolJson.ParseObject(text); }
            catch (JsonException) { return DecodeResult.Fail(ReasonCodes.ProtocolInvalidMessage, null, "not a JSON object"); }

            var msgId = frame["msg_id"]?.Type == JTokenType.String ? (string?)frame["msg_id"] : null;
            var errors = ProtocolSchemas.ValidateFrame(frame);
            if (errors.Count > 0)
            {
                // Only paths and keywords go into detail, never values (they may carry account data).
                var detail = string.Join("; ", errors.Take(3).Select(e => e.Path + " " + e.Keyword));
                var code = IsNewerMinor(frame) ? ReasonCodes.ProtocolUnsupportedValue : ReasonCodes.ProtocolInvalidMessage;
                return DecodeResult.Fail(code, LooksLikeUuid(msgId) ? msgId : null, detail);
            }

            var envelope = new Envelope(
                (string)frame["protocol_version"]!,
                (string)frame["msg_id"]!,
                (string)frame["type"]!,
                UtcTimestamp.Parse((string)frame["sent_at"]!),
                (JObject)frame["payload"]!);
            return DecodeResult.Ok(envelope);
        }

        private static bool IsNewerMinor(JObject frame)
        {
            var version = frame["protocol_version"]?.Type == JTokenType.String ? (string?)frame["protocol_version"] : null;
            if (version == null) return false;
            var parts = version.Split('.');
            return parts.Length == 2 && parts[0] == "1" && int.TryParse(parts[1], NumberStyles.None, CultureInfo.InvariantCulture, out var minor) && minor > Minor;
        }

        private static bool LooksLikeUuid(string? value) =>
            value != null && value.Length == 36 && Guid.TryParseExact(value, "D", out _) && value == value.ToLowerInvariant();
    }

    public sealed class DecodeResult
    {
        private DecodeResult(Envelope? envelope, string? errorCode, string? refMsgId, string? detail)
        {
            Envelope = envelope;
            ErrorCode = errorCode;
            RefMsgId = refMsgId;
            Detail = detail;
        }

        public Envelope? Envelope { get; }
        public string? ErrorCode { get; }
        public string? RefMsgId { get; }
        public string? Detail { get; }
        public bool IsOk => Envelope != null;

        public static DecodeResult Ok(Envelope envelope) => new DecodeResult(envelope, null, null, null);
        public static DecodeResult Fail(string code, string? refMsgId, string detail) => new DecodeResult(null, code, refMsgId, detail);
    }
}
