using System.Globalization;
using AllWin.Adapter.Core.Protocol;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Serialization;

namespace AllWin.Adapter.Core.Engine
{
    /// <summary>JSON for durable records: same exact-decimal rules as the wire, tolerant of added members.</summary>
    internal static class StoreJson
    {
        private static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            ContractResolver = new DefaultContractResolver { NamingStrategy = new SnakeCaseNamingStrategy() },
            NullValueHandling = NullValueHandling.Ignore,
            MissingMemberHandling = MissingMemberHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            FloatParseHandling = FloatParseHandling.Decimal,
            Culture = CultureInfo.InvariantCulture,
            Formatting = Formatting.None,
            Converters =
            {
                new DecimalStringConverter(),
                new UtcTimestampPreciseConverter(),
                new StringEnumConverter(new SnakeCaseNamingStrategy(), allowIntegerValues: false),
            },
        };

        public static string Write(object value) => JsonConvert.SerializeObject(value, Settings);

        public static T Read<T>(string json) => JsonConvert.DeserializeObject<T>(json, Settings)!;

        /// <summary>Full 100 ns precision in the store (the wire uses milliseconds).</summary>
        private sealed class UtcTimestampPreciseConverter : JsonConverter
        {
            public override bool CanConvert(System.Type objectType) => objectType == typeof(System.DateTime) || objectType == typeof(System.DateTime?);

            public override object? ReadJson(JsonReader reader, System.Type objectType, object? existingValue, JsonSerializer serializer) =>
                reader.TokenType == JsonToken.Null ? null : (object)UtcTimestamp.Parse((string)reader.Value!);

            public override void WriteJson(JsonWriter writer, object? value, JsonSerializer serializer)
            {
                if (value == null) { writer.WriteNull(); return; }
                var t = (System.DateTime)value;
                if (t.Kind == System.DateTimeKind.Local) t = t.ToUniversalTime();
                writer.WriteValue(t.ToString("yyyy-MM-dd'T'HH:mm:ss.fffffff'Z'", CultureInfo.InvariantCulture));
            }
        }
    }
}
