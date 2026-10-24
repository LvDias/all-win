using System;
using System.Collections.Concurrent;
using System.IO;
using System.Net.WebSockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Transport
{
    /// <summary>
    /// Opens the WebSocket to the Core: the token goes only in the Authorization header. The default uses
    /// <see cref="ClientWebSocket"/>; a host can pass its own (e.g. TLS pinned to one certificate on .NET Framework,
    /// where ClientWebSocket has no per-connection certificate callback).
    /// </summary>
    public delegate Task<WebSocket> WebSocketFactory(Uri uri, string token, CancellationToken ct);

    /// <summary>
    /// Keeps one outbound WebSocket to the Core open while the engine wants it (ADR-0004 §2, §4).
    /// Receives on its own task, sends from a serialized queue (ClientWebSocket allows one send at a
    /// time), and hands every frame to the <see cref="EngineLoop"/>. The device token travels only in
    /// the Authorization header and is never logged (only its key_id).
    /// </summary>
    public sealed class WebSocketConnector : IDisposable
    {
        private readonly Uri _uri;
        private readonly Func<string?> _token;
        private readonly EngineLoop _loop;
        private readonly IAdapterLog _log;
        private readonly Backoff _backoff;
        private readonly WebSocketFactory _connect;
        private readonly CancellationTokenSource _stop = new CancellationTokenSource();
        private Task? _run;
        private int _problem;

        /// <summary>
        /// Why the last attempt was refused, for the window (Pêndulo M1 item 5); None after a connection opens. Read from any thread.
        /// </summary>
        public TransportProblem LastProblem => (TransportProblem)Volatile.Read(ref _problem);

        /// <param name="build">
        /// Release builds accept only wss://. Development builds also accept ws:// on loopback, for the fake
        /// Core harness (QA Security I2).
        /// </param>
        public WebSocketConnector(Uri uri, Func<string?> deviceToken, EngineLoop loop, AdapterBuild build, IAdapterLog? log = null, Backoff? backoff = null,
            WebSocketFactory? connect = null)
        {
            var allowed = uri.Scheme == "wss" || (build == AdapterBuild.Development && uri.Scheme == "ws" && uri.IsLoopback);
            if (!allowed)
                throw new ArgumentException("The Core endpoint must use wss:// (ws:// only on loopback, in development builds).", nameof(uri));
            _uri = uri;
            _token = deviceToken;
            _loop = loop;
            _log = log ?? NullLog.Instance;
            _backoff = backoff ?? new Backoff();
            _connect = connect ?? ConnectDefaultAsync;
        }

        private static async Task<WebSocket> ConnectDefaultAsync(Uri uri, string token, CancellationToken ct)
        {
            var socket = new ClientWebSocket();
            try
            {
                socket.Options.SetRequestHeader("Authorization", "Bearer " + token);
                socket.Options.KeepAliveInterval = TimeSpan.FromSeconds(20);
                await socket.ConnectAsync(uri, ct).ConfigureAwait(false);
                return socket;
            }
            catch
            {
                socket.Dispose();
                throw;
            }
        }

        public void Start() => _run = Task.Run(() => RunAsync(_stop.Token));

        public void Dispose()
        {
            _stop.Cancel();
            try { _run?.Wait(TimeSpan.FromSeconds(5)); }
            catch (AggregateException) { }
            _stop.Dispose();
        }

        private async Task RunAsync(CancellationToken ct)
        {
            var attempt = 0;
            while (!ct.IsCancellationRequested)
            {
                if (!await _loop.InvokeAsync(e => e.ShouldReconnect).ConfigureAwait(false)) break;
                var token = _token();
                if (string.IsNullOrEmpty(token))
                {
                    _log.Write(LogLevel.Warning, "transport.not_paired", "no device token");
                    await Delay(TimeSpan.FromSeconds(5), ct).ConfigureAwait(false);
                    continue;
                }

                var reachedReady = false;
                TimeSpan? retryAfter = null;
                WebSocket? socket = null;
                try
                {
                    int? closeCode = null;
                    try
                    {
                        using (var connectTimeout = CancellationTokenSource.CreateLinkedTokenSource(ct))
                        {
                            connectTimeout.CancelAfter(TimeSpan.FromSeconds(15));
                            socket = await _connect(_uri, token!, connectTimeout.Token).ConfigureAwait(false);
                        }
                        Volatile.Write(ref _problem, (int)TransportProblem.None);
                        _log.Write(LogLevel.Info, "transport.connected", _uri.Host + " key=" + TokenMask.Mask(token));
                        using (var sink = new SocketSink(socket, _log))
                        using (var receive = CancellationTokenSource.CreateLinkedTokenSource(ct, sink.Aborted))
                        {
                            _loop.Post(e => e.OnConnected(sink));
                            closeCode = await ReceiveAsync(socket, receive.Token).ConfigureAwait(false);
                        }
                    }
                    catch (OperationCanceledException) when (ct.IsCancellationRequested) { }
                    catch (OperationCanceledException) { _log.Write(LogLevel.Warning, "transport.aborted", "engine close"); }
                    catch (Exception e) when (e is WebSocketException || e is IOException || e is OperationCanceledException || e is TransportRefusedException)
                    {
                        _log.Write(LogLevel.Warning, "transport.error", e.GetType().Name + (e is TransportRefusedException ? ": " + e.Message : ""));
                        retryAfter = (e as TransportRefusedException)?.RetryAfter;
                        Volatile.Write(ref _problem, (int)((e as TransportRefusedException)?.Problem ?? TransportProblem.None));
                    }
                    reachedReady = await _loop.InvokeAsync(e => e.Session == SessionState.Ready).ConfigureAwait(false);
                    var code = closeCode;
                    _loop.Post(e => e.OnDisconnected(code));
                }
                finally { socket?.Dispose(); }
                if (reachedReady) attempt = 0;
                await Delay(NextDelay(_backoff.Delay(attempt++), retryAfter), ct).ConfigureAwait(false);
            }
        }

        private async Task<int?> ReceiveAsync(WebSocket socket, CancellationToken ct)
        {
            var buffer = new byte[16 * 1024];
            var message = new MemoryStream();
            while (socket.State == WebSocketState.Open && !ct.IsCancellationRequested)
            {
                var result = await socket.ReceiveAsync(new ArraySegment<byte>(buffer), ct).ConfigureAwait(false);
                if (result.MessageType == WebSocketMessageType.Close)
                {
                    try { await socket.CloseOutputAsync(WebSocketCloseStatus.NormalClosure, "", CancellationToken.None).ConfigureAwait(false); }
                    catch (WebSocketException) { }
                    return (int?)result.CloseStatus;
                }
                message.Write(buffer, 0, result.Count);
                if (message.Length > ProtocolJson.MaxFrameBytes)
                {
                    await socket.CloseAsync(WebSocketCloseStatus.MessageTooBig, "frame too large", CancellationToken.None).ConfigureAwait(false);
                    return null;
                }
                if (!result.EndOfMessage) continue;
                if (result.MessageType == WebSocketMessageType.Text)
                {
                    var text = Encoding.UTF8.GetString(message.GetBuffer(), 0, (int)message.Length);
                    _loop.Post(e => e.OnFrame(text));
                }
                message.SetLength(0);
            }
            return (int?)socket.CloseStatus;
        }

        /// <summary>C-1: max(Retry-After, backoff); a Retry-After that wins gets up to +20 % jitter so clients do not return together.</summary>
        internal static TimeSpan NextDelay(TimeSpan backoff, TimeSpan? retryAfter, double random01 = -1)
        {
            if (retryAfter == null || retryAfter.Value <= backoff) return backoff;
            var r = random01 >= 0 ? random01 : JitterSource.NextDouble();
            return TimeSpan.FromMilliseconds(retryAfter.Value.TotalMilliseconds * (1.0 + 0.2 * r));
        }

        private static readonly Random JitterSource = new Random();

        private static async Task Delay(TimeSpan delay, CancellationToken ct)
        {
            try { await Task.Delay(delay, ct).ConfigureAwait(false); }
            catch (OperationCanceledException) { }
        }

        /// <summary>Serialized sender: the engine enqueues and returns; one task writes to the socket.</summary>
        private sealed class SocketSink : IFrameSink, IDisposable
        {
            /// <summary>Grace for the close handshake before the socket is aborted (half-open TCP never answers).</summary>
            private static readonly TimeSpan CloseGrace = TimeSpan.FromSeconds(2);

            private readonly WebSocket _socket;
            private readonly IAdapterLog _log;
            private readonly BlockingCollection<Func<Task>> _sends = new BlockingCollection<Func<Task>>();
            private readonly CancellationTokenSource _abort = new CancellationTokenSource();
            private readonly Task _pump;

            public SocketSink(WebSocket socket, IAdapterLog log)
            {
                _socket = socket;
                _log = log;
                _pump = Task.Run(PumpAsync);
            }

            public void Send(string frame)
            {
                var bytes = Encoding.UTF8.GetBytes(frame);
                TryAdd(() => _socket.SendAsync(new ArraySegment<byte>(bytes), WebSocketMessageType.Text, true, CancellationToken.None));
            }

            /// <summary>Cancelled when the engine closed the session and the peer did not answer in time.</summary>
            public CancellationToken Aborted => _abort.Token;

            /// <summary>
            /// The engine closes (e.g. 4005 heartbeat timeout, Code Review 6 M2). On a half-open connection
            /// neither the close handshake nor the pending receive would complete before the OS TCP timeout,
            /// so after a short grace the socket is aborted and the receive cancelled; the connector reconnects.
            /// </summary>
            public void Close(int code, string reason)
            {
                _ = Task.Run(async () =>
                {
                    using (var grace = new CancellationTokenSource(CloseGrace))
                    {
                        try { await _socket.CloseOutputAsync((WebSocketCloseStatus)code, reason, grace.Token).ConfigureAwait(false); }
                        catch (Exception e) when (e is WebSocketException || e is OperationCanceledException || e is ObjectDisposedException || e is InvalidOperationException) { }
                    }
                    try { await Task.Delay(CloseGrace).ConfigureAwait(false); } catch (ObjectDisposedException) { }
                    if (_socket.State != WebSocketState.Closed)
                    {
                        _socket.Abort();
                        try { _abort.Cancel(); } catch (ObjectDisposedException) { }
                    }
                });
            }

            private void TryAdd(Func<Task> work)
            {
                try { _sends.Add(work); }
                catch (InvalidOperationException) { } // connection already gone; the outbox keeps reliable messages
            }

            private async Task PumpAsync()
            {
                foreach (var send in _sends.GetConsumingEnumerable())
                {
                    if (_socket.State != WebSocketState.Open) continue;
                    try { await send().ConfigureAwait(false); }
                    catch (Exception e) when (e is WebSocketException || e is ObjectDisposedException || e is InvalidOperationException)
                    {
                        _log.Write(LogLevel.Warning, "transport.send_failed", e.GetType().Name);
                    }
                }
            }

            public void Dispose()
            {
                _sends.CompleteAdding();
                try { _pump.Wait(TimeSpan.FromSeconds(2)); }
                catch (AggregateException) { }
                _sends.Dispose();
                _abort.Dispose();
            }
        }
    }

    /// <summary>A refusal the window explains on its own line; the text key is "transport." + snake_case name.</summary>
    public enum TransportProblem
    {
        None,
        PinMismatch,
    }

    /// <summary>A connection the transport refused on purpose (e.g. certificate pin mismatch); the reason is safe to log.</summary>
    public sealed class TransportRefusedException : Exception
    {
        public TransportRefusedException(string reason, TimeSpan? retryAfter = null, TransportProblem problem = TransportProblem.None) : base(reason)
        {
            RetryAfter = retryAfter;
            Problem = problem;
        }

        public TransportProblem Problem { get; }

        /// <summary>The server's Retry-After (429/503): the next attempt waits at least this long.</summary>
        public TimeSpan? RetryAfter { get; }
    }
}
