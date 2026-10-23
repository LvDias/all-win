using System.Globalization;
using System.IO;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace AllWin.Adapter.Core.Protocol
{
    /// <summary>
    /// JSON reading/writing rules for the protocol: timestamps stay strings, numbers never become
    /// doubles (ADR-0005 N3), duplicate keys are an error, output is compact and culture-invariant.
    /// </summary>
    public static class ProtocolJson
    {
        /// <summary>Largest frame either side may send (ADR-0004 §2).</summary>
        public const int MaxFrameBytes = 256 * 1024;

        private static readonly JsonLoadSettings LoadSettings = new JsonLoadSettings
        {
            DuplicatePropertyNameHandling = DuplicatePropertyNameHandling.Error,
            CommentHandling = CommentHandling.Ignore,
            LineInfoHandling = LineInfoHandling.Ignore,
        };

        public static JToken Parse(string json)
        {
            using (var reader = new JsonTextReader(new StringReader(json)))
            {
                reader.DateParseHandling = DateParseHandling.None;
                reader.FloatParseHandling = FloatParseHandling.Decimal;
                reader.MaxDepth = 64;
                reader.Culture = CultureInfo.InvariantCulture;
                var token = JToken.ReadFrom(reader, LoadSettings);
                if (reader.Read()) throw new JsonReaderException("Additional content after the JSON value.");
                return token;
            }
        }

        public static JObject ParseObject(string json)
        {
            var token = Parse(json);
            return token as JObject ?? throw new JsonReaderException("Expected a JSON object.");
        }

        public static string Serialize(JToken token) => token.ToString(Formatting.None);
    }
}
