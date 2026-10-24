// All Win — development "real Core" transport: WebSocket over our own TLS stream, pinned to ONE certificate SHA-256
// (Maestro 2026-10-24). On .NET Framework 4.8 ClientWebSocket has no per-connection certificate callback, and the global
// ServicePointManager callback would affect every TLS connection in NinjaTrader, so we do the TLS + RFC 6455 handshake here.
// No CA chain, no name check, no "accept invalid": the certificate is accepted only if its SHA-256 equals the pin.
// C# 7.3 (compiled by NinjaTrader).
#region Using declarations
using System;
using System.IO;
using System.Net.Security;
using System.Net.Sockets;
using System.Net.WebSockets;
using System.Security.Authentication;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Transport;
#endregion

namespace NinjaTrader.NinjaScript.AddOns
{
    public static class AllWinPinnedSocket
    {
        private const int BufferSize = 16 * 1024;
        private const int MaxHeadBytes = 8192;

        public static WebSocketFactory Create(string pinSha256)
        {
            return (uri, token, ct) => ConnectAsync(uri, token, pinSha256, ct);
        }

        private static async Task<WebSocket> ConnectAsync(Uri uri, string token, string pin, CancellationToken ct)
        {
            var tcp = new TcpClient();
            SslStream ssl = null;
            // TLS and the upgrade are not cancellable on net48: closing the socket is the only way to stop a server that
            // accepts TCP and then stays silent (Crivo M1). The registration covers TCP + TLS + upgrade head, not the session.
            // The callback may run on the connector's timeout timer: it must never throw.
            CancellationTokenRegistration abort = ct.Register(() => { try { tcp.Close(); } catch (Exception) { } });
            try
            {
                await tcp.ConnectAsync(uri.Host, uri.Port).ConfigureAwait(false);
                ct.ThrowIfCancellationRequested();
                bool certificateSeen = false;
                bool pinMatched = false;
                ssl = new SslStream(tcp.GetStream(), false, (sender, certificate, chain, errors) =>
                {
                    certificateSeen = true;
                    pinMatched = certificate != null && CertificatePin.Matches(certificate.GetRawCertData(), pin);
                    return pinMatched;
                });
                try
                {
                    await ssl.AuthenticateAsClientAsync(uri.Host, null, SslProtocols.Tls12, false).ConfigureAwait(false);
                }
                catch (AuthenticationException)
                {
                    // Only a certificate we actually saw and rejected is a pin mismatch; a timeout abort or an early close is not.
                    ct.ThrowIfCancellationRequested();
                    throw pinMatched || !certificateSeen
                        ? new TransportRefusedException("TLS handshake failed")
                        : new TransportRefusedException("server certificate does not match the configured pin", null, TransportProblem.PinMismatch);
                }

                string key = WebSocketHandshake.NewKey();
                byte[] request = Encoding.ASCII.GetBytes(WebSocketHandshake.BuildRequest(uri, key, token));
                await ssl.WriteAsync(request, 0, request.Length, ct).ConfigureAwait(false);
                string head = await ReadHeadAsync(ssl, ct).ConfigureAwait(false);
                string error = WebSocketHandshake.Validate(head, key);
                // A refused upgrade (401/429/503/5xx) is transient for the transport; Retry-After is honoured (C-1).
                if (error != null) throw new TransportRefusedException(error, WebSocketHandshake.RetryAfter(head));

                // Dispose waits for a running callback, so after it the socket is either open for good or already closed.
                abort.Dispose();
                ct.ThrowIfCancellationRequested();
                WebSocket inner = WebSocket.CreateClientWebSocket(ssl, null, BufferSize, BufferSize, TimeSpan.FromSeconds(20), false,
                    WebSocket.CreateClientBuffer(BufferSize, BufferSize));
                return new OwningWebSocket(inner, ssl, tcp);
            }
            catch (TransportRefusedException)
            {
                Close(ssl, tcp);
                if (ct.IsCancellationRequested) throw new OperationCanceledException(ct);
                throw;
            }
            catch (OperationCanceledException) { Close(ssl, tcp); throw; }
            catch (Exception e)
            {
                Close(ssl, tcp);
                if (ct.IsCancellationRequested) throw new OperationCanceledException(ct);
                // The connector retries on IOException; the type name is safe to log (no token, no URL secret).
                throw new IOException("connect failed: " + e.GetType().Name, e);
            }
            finally
            {
                abort.Dispose();
            }
        }

        /// <summary>Reads the HTTP response head byte by byte, so no WebSocket frame that follows it is consumed.</summary>
        private static async Task<string> ReadHeadAsync(Stream stream, CancellationToken ct)
        {
            var head = new StringBuilder();
            var one = new byte[1];
            while (head.Length < MaxHeadBytes)
            {
                int read = await stream.ReadAsync(one, 0, 1, ct).ConfigureAwait(false);
                if (read == 0) throw new TransportRefusedException("connection closed during the upgrade");
                head.Append((char)one[0]);
                int n = head.Length;
                if (n >= 4 && head[n - 4] == '\r' && head[n - 3] == '\n' && head[n - 2] == '\r' && head[n - 1] == '\n')
                    return head.ToString(0, n - 4);
            }
            throw new TransportRefusedException("upgrade response head too large");
        }

        private static void Close(SslStream ssl, TcpClient tcp)
        {
            try { if (ssl != null) ssl.Dispose(); } catch (Exception) { }
            try { tcp.Close(); } catch (Exception) { }
        }

        /// <summary>The stream-based WebSocket does not own its stream: this wrapper closes TLS and TCP with it.</summary>
        private sealed class OwningWebSocket : WebSocket
        {
            private readonly WebSocket inner;
            private readonly SslStream ssl;
            private readonly TcpClient tcp;

            public OwningWebSocket(WebSocket inner, SslStream ssl, TcpClient tcp)
            {
                this.inner = inner;
                this.ssl = ssl;
                this.tcp = tcp;
            }

            public override WebSocketCloseStatus? CloseStatus { get { return inner.CloseStatus; } }
            public override string CloseStatusDescription { get { return inner.CloseStatusDescription; } }
            public override string SubProtocol { get { return inner.SubProtocol; } }
            public override WebSocketState State { get { return inner.State; } }

            public override void Abort()
            {
                inner.Abort();
                Close(ssl, tcp);
            }

            public override Task CloseAsync(WebSocketCloseStatus closeStatus, string statusDescription, CancellationToken cancellationToken)
            {
                return inner.CloseAsync(closeStatus, statusDescription, cancellationToken);
            }

            public override Task CloseOutputAsync(WebSocketCloseStatus closeStatus, string statusDescription, CancellationToken cancellationToken)
            {
                return inner.CloseOutputAsync(closeStatus, statusDescription, cancellationToken);
            }

            public override Task<WebSocketReceiveResult> ReceiveAsync(ArraySegment<byte> buffer, CancellationToken cancellationToken)
            {
                return inner.ReceiveAsync(buffer, cancellationToken);
            }

            public override Task SendAsync(ArraySegment<byte> buffer, WebSocketMessageType messageType, bool endOfMessage, CancellationToken cancellationToken)
            {
                return inner.SendAsync(buffer, messageType, endOfMessage, cancellationToken);
            }

            public override void Dispose()
            {
                try { inner.Dispose(); } catch (Exception) { }
                Close(ssl, tcp);
            }
        }
    }
}
