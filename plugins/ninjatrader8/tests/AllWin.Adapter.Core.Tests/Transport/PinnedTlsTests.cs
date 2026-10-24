using System;
using System.Linq;
using System.Text;
using AllWin.Adapter.Core.Transport;
using Xunit;

namespace AllWin.Adapter.Core.Tests.Transport
{
    /// <summary>Development "real Core" mode: certificate pin, dev-core.json and the RFC 6455 handshake (Maestro 2026-10-24).</summary>
    public class PinnedTlsTests
    {
        private static readonly byte[] Der = Encoding.ASCII.GetBytes("not a real certificate, only bytes to hash");

        [Fact]
        public void Pin_matches_only_the_exact_sha256_in_any_hex_format()
        {
            var hex = CertificatePin.Sha256Hex(Der);
            Assert.True(CertificatePin.Matches(Der, hex));
            Assert.True(CertificatePin.Matches(Der, hex.ToUpperInvariant()));
            var colons = string.Join(":", Enumerable.Range(0, 32).Select(i => hex.Substring(i * 2, 2)));
            Assert.True(CertificatePin.Matches(Der, colons));
            var other = (hex[0] == 'a' ? "b" : "a") + hex.Substring(1);
            Assert.False(CertificatePin.Matches(Der, other));
        }

        [Theory]
        [InlineData(null)]
        [InlineData("")]
        [InlineData("abc")]
        [InlineData("zz2d2c3ed5d8e3f9f53b5a8cd70c2fdc0e7b5e0f6a1a0f1c9e4c6d2b8a7f3e1d")]
        public void A_malformed_pin_never_matches(string? pin)
        {
            Assert.False(CertificatePin.Matches(Der, pin!));
            Assert.Null(CertificatePin.Normalize(pin));
        }

        [Fact]
        public void An_empty_certificate_never_matches()
        {
            Assert.False(CertificatePin.Matches(new byte[0], CertificatePin.Sha256Hex(new byte[0])));
            Assert.False(CertificatePin.Matches(null, CertificatePin.Sha256Hex(Der)));
        }

        [Fact]
        public void Dev_core_config_accepts_only_wss_and_a_sha256_pin()
        {
            var pin = CertificatePin.Sha256Hex(Der);
            var ok = DevCoreConfig.Parse("{\"endpoint\":\"wss://100.64.0.10:8443/adapter/v1\",\"cert_sha256\":\"" + pin + "\"}", out var error);
            Assert.Null(error);
            Assert.Equal("wss://100.64.0.10:8443/adapter/v1", ok!.Endpoint.ToString());
            Assert.Equal(pin, ok.CertSha256);
        }

        [Theory]
        [InlineData("{\"endpoint\":\"ws://100.64.0.10:8443/adapter/v1\",\"cert_sha256\":\"PIN\"}")]          // not TLS
        [InlineData("{\"endpoint\":\"wss://user:pw@host:8443/adapter/v1\",\"cert_sha256\":\"PIN\"}")]       // credentials in the URL
        [InlineData("{\"endpoint\":\"wss://host:8443/adapter/v1?t=x\",\"cert_sha256\":\"PIN\"}")]           // query (a token could hide there)
        [InlineData("{\"endpoint\":\"wss://host:8443/adapter/v1\"}")]                                     // no pin
        [InlineData("{\"endpoint\":\"wss://host:8443/adapter/v1\",\"cert_sha256\":\"1234\"}")]             // short pin
        [InlineData("{\"endpoint\":\"wss://host:8443/adapter/v1\",\"cert_sha256\":\"PIN\",\"insecure\":true}")] // unknown field
        [InlineData("not json")]
        public void Dev_core_config_refuses_anything_else(string json)
        {
            var config = DevCoreConfig.Parse(json.Replace("PIN", CertificatePin.Sha256Hex(Der)), out var error);
            Assert.Null(config);
            Assert.NotNull(error);
        }

        [Fact]
        public void Handshake_follows_rfc_6455()
        {
            const string key = "dGhlIHNhbXBsZSBub25jZQ==";
            Assert.Equal("s3pPLMBiTxaQ9kYGzzhZRbK+xOo=", WebSocketHandshake.ExpectedAccept(key));
            var request = WebSocketHandshake.BuildRequest(new Uri("wss://host:8443/adapter/v1"), key, "awd_KEYID1234567_secret");
            Assert.StartsWith("GET /adapter/v1 HTTP/1.1\r\nHost: host:8443\r\n", request);
            Assert.Contains("Authorization: Bearer awd_KEYID1234567_secret\r\n", request);
            Assert.EndsWith("\r\n\r\n", request);
            Assert.Null(WebSocketHandshake.Validate("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=", key));
        }

        [Theory]
        [InlineData("HTTP/1.1 401 Unauthorized\r\n", "HTTP 401")]
        [InlineData("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: wrong=", "Sec-WebSocket-Accept")]
        [InlineData("HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=", "Upgrade")]
        public void Handshake_refuses_a_bad_response(string head, string reason)
        {
            Assert.Contains(reason, WebSocketHandshake.Validate(head, "dGhlIHNhbXBsZSBub25jZQ==")!);
        }

        [Theory]
        [InlineData("HTTP/1.1 429 Too Many Requests\r\nRetry-After: 120", 120)]
        [InlineData("HTTP/1.1 503 Service Unavailable\r\nretry-after: 7", 7)]
        [InlineData("HTTP/1.1 429 Too Many Requests\r\nRetry-After: 999999", 3600)]
        public void Retry_after_is_read_in_seconds_and_capped(string head, int seconds)
        {
            Assert.Equal(TimeSpan.FromSeconds(seconds), WebSocketHandshake.RetryAfter(head));
            Assert.Contains("upgrade refused", WebSocketHandshake.Validate(head, "dGhlIHNhbXBsZSBub25jZQ==")!);
        }

        [Theory]
        [InlineData("HTTP/1.1 429 Too Many Requests\r\nRetry-After: Wed, 21 Oct 2026 07:28:00 GMT")]
        [InlineData("HTTP/1.1 503 Service Unavailable")]
        public void Retry_after_absent_or_a_date_is_ignored(string head)
        {
            Assert.Null(WebSocketHandshake.RetryAfter(head));
        }

        [Fact]
        public void Next_delay_is_the_larger_of_retry_after_and_backoff_with_jitter_on_retry_after()
        {
            var backoff = TimeSpan.FromSeconds(4);
            Assert.Equal(backoff, WebSocketConnector.NextDelay(backoff, null));
            Assert.Equal(backoff, WebSocketConnector.NextDelay(backoff, TimeSpan.FromSeconds(2)));
            Assert.Equal(TimeSpan.FromSeconds(60), WebSocketConnector.NextDelay(backoff, TimeSpan.FromSeconds(60), 0.0));
            Assert.Equal(TimeSpan.FromSeconds(72), WebSocketConnector.NextDelay(backoff, TimeSpan.FromSeconds(60), 1.0));
        }
    }
}
