using System;
using System.Collections.Generic;
using System.Globalization;
using System.Security.Cryptography;
using System.Text;
using Newtonsoft.Json.Linq;

namespace AllWin.Adapter.Core.Transport
{
    /// <summary>
    /// Development-build "real Core" settings (%LOCALAPPDATA%\AllWin\dev-core.json, Maestro 2026-10-24):
    /// <c>{"endpoint": "wss://&lt;host&gt;:8443/adapter/v1", "cert_sha256": "&lt;64 hex&gt;"}</c>. The certificate is accepted only
    /// when its SHA-256 equals the pin: no CA chain, no "accept invalid". A file that is not exactly this shape is refused.
    /// </summary>
    public sealed class DevCoreConfig
    {
        private DevCoreConfig(Uri endpoint, string pin)
        {
            Endpoint = endpoint;
            CertSha256 = pin;
        }

        public Uri Endpoint { get; }

        /// <summary>Lower-case hex, 64 characters.</summary>
        public string CertSha256 { get; }

        /// <summary>Parses the file; returns null and a reason when anything is missing, unknown or malformed.</summary>
        public static DevCoreConfig? Parse(string json, out string? error)
        {
            error = null;
            JObject o;
            try { o = JObject.Parse(json); }
            catch (Exception) { error = "dev-core.json is not valid JSON"; return null; }
            foreach (var property in o.Properties())
                if (property.Name != "endpoint" && property.Name != "cert_sha256") { error = "dev-core.json: unknown field " + property.Name; return null; }
            var endpoint = o["endpoint"]?.Type == JTokenType.String ? (string?)o["endpoint"] : null;
            var pin = o["cert_sha256"]?.Type == JTokenType.String ? (string?)o["cert_sha256"] : null;
            if (endpoint == null || !Uri.TryCreate(endpoint, UriKind.Absolute, out var uri) || uri.Scheme != "wss" || !string.IsNullOrEmpty(uri.UserInfo) || uri.Query.Length > 0)
            {
                error = "dev-core.json: endpoint must be wss://host:port/path";
                return null;
            }
            var normalized = CertificatePin.Normalize(pin);
            if (normalized == null) { error = "dev-core.json: cert_sha256 must be 64 hex characters (colons allowed)"; return null; }
            return new DevCoreConfig(uri, normalized);
        }
    }

    /// <summary>SHA-256 pin of the server certificate (DER).</summary>
    public static class CertificatePin
    {
        /// <summary>64 hex digits, case-insensitive, ':' and spaces ignored; null when not a SHA-256.</summary>
        public static string? Normalize(string? pin)
        {
            if (pin == null) return null;
            var sb = new StringBuilder(64);
            foreach (var c in pin)
            {
                if (c == ':' || c == ' ') continue;
                if (!Uri.IsHexDigit(c)) return null;
                sb.Append(char.ToLowerInvariant(c));
            }
            return sb.Length == 64 ? sb.ToString() : null;
        }

        public static string Sha256Hex(byte[] der)
        {
            using (var sha = SHA256.Create())
            {
                var hash = sha.ComputeHash(der);
                var sb = new StringBuilder(64);
                foreach (var b in hash) sb.Append(b.ToString("x2", CultureInfo.InvariantCulture));
                return sb.ToString();
            }
        }

        /// <summary>True only when the certificate's SHA-256 equals the pin (constant-time compare).</summary>
        public static bool Matches(byte[]? der, string pin)
        {
            var expected = Normalize(pin);
            if (der == null || der.Length == 0 || expected == null) return false;
            var actual = Sha256Hex(der);
            var diff = 0;
            for (var i = 0; i < 64; i++) diff |= actual[i] ^ expected[i];
            return diff == 0;
        }
    }

    /// <summary>RFC 6455 client opening handshake, for hosts that run the WebSocket over their own (pinned) TLS stream.</summary>
    public static class WebSocketHandshake
    {
        private const string Guid = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

        public static string NewKey()
        {
            var bytes = new byte[16];
            using (var rng = RandomNumberGenerator.Create()) rng.GetBytes(bytes);
            return Convert.ToBase64String(bytes);
        }

        /// <summary>The HTTP/1.1 upgrade request. The token goes only in the Authorization header.</summary>
        public static string BuildRequest(Uri uri, string key, string token)
        {
            var host = uri.IsDefaultPort ? uri.Host : uri.Host + ":" + uri.Port.ToString(CultureInfo.InvariantCulture);
            return "GET " + uri.PathAndQuery + " HTTP/1.1\r\n"
                + "Host: " + host + "\r\n"
                + "Upgrade: websocket\r\n"
                + "Connection: Upgrade\r\n"
                + "Sec-WebSocket-Key: " + key + "\r\n"
                + "Sec-WebSocket-Version: 13\r\n"
                + "Authorization: Bearer " + token + "\r\n"
                + "\r\n";
        }

        /// <summary>
        /// Retry-After of a refused upgrade (429/503, C-1): delta-seconds only, capped at 1 h; null when absent or not a number.
        /// A refused upgrade is always transient (never "pair again"); the connector waits max(Retry-After, backoff).
        /// </summary>
        public static TimeSpan? RetryAfter(string responseHead)
        {
            foreach (var line in responseHead.Replace("\r\n", "\n").Split('\n'))
            {
                var colon = line.IndexOf(':');
                if (colon <= 0 || !string.Equals(line.Substring(0, colon).Trim(), "Retry-After", StringComparison.OrdinalIgnoreCase)) continue;
                if (int.TryParse(line.Substring(colon + 1).Trim(), NumberStyles.None, CultureInfo.InvariantCulture, out var seconds))
                    return TimeSpan.FromSeconds(Math.Min(seconds, 3600));
                return null;
            }
            return null;
        }

        public static string ExpectedAccept(string key)
        {
            using (var sha = SHA1.Create()) return Convert.ToBase64String(sha.ComputeHash(Encoding.ASCII.GetBytes(key + Guid)));
        }

        /// <summary>Null when the response head (status line + headers, without the final blank line) completes the upgrade; else the reason.</summary>
        public static string? Validate(string responseHead, string key)
        {
            var lines = responseHead.Replace("\r\n", "\n").Split('\n');
            var status = lines[0].Split(' ');
            if (status.Length < 2 || !status[0].StartsWith("HTTP/1.1", StringComparison.Ordinal)) return "not an HTTP/1.1 response";
            if (status[1] != "101") return "upgrade refused: HTTP " + status[1];
            var headers = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
            for (var i = 1; i < lines.Length; i++)
            {
                var colon = lines[i].IndexOf(':');
                if (colon > 0) headers[lines[i].Substring(0, colon).Trim()] = lines[i].Substring(colon + 1).Trim();
            }
            if (!headers.TryGetValue("Upgrade", out var upgrade) || !string.Equals(upgrade, "websocket", StringComparison.OrdinalIgnoreCase)) return "missing Upgrade: websocket";
            if (!headers.TryGetValue("Connection", out var connection) || connection.IndexOf("upgrade", StringComparison.OrdinalIgnoreCase) < 0) return "missing Connection: Upgrade";
            if (!headers.TryGetValue("Sec-WebSocket-Accept", out var accept) || accept != ExpectedAccept(key)) return "bad Sec-WebSocket-Accept";
            return null;
        }
    }
}
