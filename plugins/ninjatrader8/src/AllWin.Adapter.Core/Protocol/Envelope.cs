using System;
using Newtonsoft.Json.Linq;

namespace AllWin.Adapter.Core.Protocol
{
    /// <summary>One protocol frame: envelope fields plus the raw payload (typed on demand).</summary>
    public sealed class Envelope
    {
        public Envelope(string protocolVersion, string msgId, string type, DateTime sentAt, JObject payload)
        {
            ProtocolVersion = protocolVersion;
            MsgId = msgId;
            Type = type;
            SentAt = sentAt;
            Payload = payload;
        }

        public string ProtocolVersion { get; }
        public string MsgId { get; }
        public string Type { get; }
        public DateTime SentAt { get; }
        public JObject Payload { get; }

        public T PayloadAs<T>() => ProtocolCodec.ToModel<T>(Payload);
    }
}
