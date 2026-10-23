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
        private readonly CancellationTokenSource _stop = new CancellationTokenSource();
        private Task? _run;

        public WebSocketConnector(Uri uri, Func<string?> deviceToken, EngineLoop loop, IAdapterLog? log = null, Backoff? backoff = null)
        {
            if (uri.Scheme != "wss" && !(uri.Scheme == "ws" && uri.IsLoopback))
                throw new ArgumentException("The Core endpoint must use wss:// (ws:// only on loopback for the test harness).", nameof(uri));
            _uri = uri;
            _token = deviceToken;
            _loop = loop;
            _log = log ?? NullLog.Instance;
            _backoff = backoff ?? new Backoff();
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
                using (var socket = new ClientWebSocket())
                {
                    socket.Options.SetRequestHeader("Authorization", "Bearer " + token);
                    socket.Options.KeepAliveInterval = TimeSpan.FromSeconds(20);
                    int? closeCode = null;
                    try
                    {
                        using (var connectTimeout = CancellationTokenSource.CreateLinkedTokenSource(ct))
                        {
                            connectTimeout.CancelAfter(TimeSpan.FromSeconds(15));
                            await socket.ConnectAsync(_uri, connectTimeout.Token).ConfigureAwait(false);
                        }
                        _log.Write(LogLevel.Info, "transport.connected", _uri.Host + " key=" + TokenMask.Mask(token));
                        using (var sink = new SocketSink(socket, _log))
                        {
                            _loop.Post(e => e.OnConnected(sink));
                            closeCode = await ReceiveAsync(socket, ct).ConfigureAwait(false);
                        }
                    }
                    catch (OperationCanceledException) when (ct.IsCancellationRequested) { }
                    catch (Exception e) when (e is WebSocketException || e is IOException || e is OperationCanceledException)
                    {
                        _log.Write(LogLevel.Warning, "transport.error", e.GetType().Name);
                    }
                    reachedReady = await _loop.InvokeAsync(e => e.Session == SessionState.Ready).ConfigureAwait(false);
                    var code = closeCode;
                    _loop.Post(e => e.OnDisconnected(code));
                }
                if (reachedReady) attempt = 0;
                await Delay(_backoff.Delay(attempt++), ct).ConfigureAwait(false);
            }
        }

        private async Task<int?> ReceiveAsync(ClientWebSocket socket, CancellationToken ct)
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

        private static async Task Delay(TimeSpan delay, CancellationToken ct)
        {
            try { await Task.Delay(delay, ct).ConfigureAwait(false); }
            catch (OperationCanceledException) { }
        }

        /// <summary>Serialized sender: the engine enqueues and returns; one task writes to the socket.</summary>
        private sealed class SocketSink : IFrameSink, IDisposable
        {
            private readonly ClientWebSocket _socket;
            private readonly IAdapterLog _log;
            private readonly BlockingCollection<Func<Task>> _sends = new BlockingCollection<Func<Task>>();
            private readonly Task _pump;

            public SocketSink(ClientWebSocket socket, IAdapterLog log)
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

            public void Close(int code, string reason)
            {
                TryAdd(() => _socket.CloseAsync((WebSocketCloseStatus)code, reason, CancellationToken.None));
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
            }
        }
    }
}
