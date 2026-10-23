using System;
using System.Globalization;
using Newtonsoft.Json;

namespace AllWin.Adapter.Core.Protocol
{
    /// <summary>Exact decimals travel as JSON strings; JSON numbers are refused (CLAUDE.md §2).</summary>
    public sealed class DecimalStringConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType) => objectType == typeof(decimal) || objectType == typeof(decimal?);

        public override object? ReadJson(JsonReader reader, Type objectType, object? existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null && objectType == typeof(decimal?)) return null;
            if (reader.TokenType != JsonToken.String) throw new JsonSerializationException("Decimal values must be JSON strings.");
            return DecimalText.Parse((string)reader.Value!);
        }

        public override void WriteJson(JsonWriter writer, object? value, JsonSerializer serializer)
        {
            if (value == null) writer.WriteNull();
            else writer.WriteValue(DecimalText.Format((decimal)value));
        }
    }

    /// <summary>UTC timestamps with a Z suffix (RFC 3339), millisecond precision on output.</summary>
    public sealed class UtcTimestampConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType) => objectType == typeof(DateTime) || objectType == typeof(DateTime?);

        public override object? ReadJson(JsonReader reader, Type objectType, object? existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null && objectType == typeof(DateTime?)) return null;
            if (reader.TokenType != JsonToken.String) throw new JsonSerializationException("Timestamps must be JSON strings.");
            return UtcTimestamp.Parse((string)reader.Value!);
        }

        public override void WriteJson(JsonWriter writer, object? value, JsonSerializer serializer)
        {
            if (value == null) writer.WriteNull();
            else writer.WriteValue(UtcTimestamp.Format((DateTime)value));
        }
    }

    public static class DecimalText
    {
        private const NumberStyles Styles = NumberStyles.AllowLeadingSign | NumberStyles.AllowDecimalPoint;

        public static decimal Parse(string text) => decimal.Parse(text, Styles, CultureInfo.InvariantCulture);

        public static bool TryParse(string text, out decimal value) => decimal.TryParse(text, Styles, CultureInfo.InvariantCulture, out value);

        /// <summary>Invariant, no exponent, scale preserved ("21034.50" stays "21034.50").</summary>
        public static string Format(decimal value) => value.ToString(CultureInfo.InvariantCulture);
    }

    public static class UtcTimestamp
    {
        public static DateTime Parse(string text)
        {
            if (text.Length == 0 || text[text.Length - 1] != 'Z') throw new FormatException("Timestamp must end with Z.");
            var body = text.Substring(0, text.Length - 1);
            var dot = body.IndexOf('.');
            // DateTime keeps 7 fraction digits; the contract allows 9.
            if (dot >= 0 && body.Length - dot - 1 > 7) body = body.Substring(0, dot + 8);
            var parsed = DateTime.ParseExact(body, "yyyy-MM-dd'T'HH:mm:ss.FFFFFFF", CultureInfo.InvariantCulture,
                DateTimeStyles.AssumeUniversal | DateTimeStyles.AdjustToUniversal);
            return DateTime.SpecifyKind(parsed, DateTimeKind.Utc);
        }

        public static string Format(DateTime value)
        {
            if (value.Kind == DateTimeKind.Local) value = value.ToUniversalTime();
            return value.ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'", CultureInfo.InvariantCulture);
        }
    }
}
