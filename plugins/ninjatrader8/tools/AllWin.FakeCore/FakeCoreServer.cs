using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Net.WebSockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Hosting.Server.Features;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace AllWin.FakeCore
{
    public sealed class FakeAccount
    {
        public string Alias { get; set; } = "";
        public string AccountId { get; set; } = Ids.New();
        public string PlatformAccount { get; set; } = "";
        public List<AccountRole> Roles { get; set; } = new List<AccountRole> { AccountRole.Destination };
        public SyncState SyncState { get; set; } = SyncState.InSync;
        public List<string> SyncReasons { get; set; } = new List<string>();
    }

    public sealed class ReceivedMessage
    {
        public ReceivedMessage(int session, Envelope envelope, bool duplicate)
        {
            Session = session;
            Envelope = envelope;
            Duplicate = duplicate;
        }

        public int Session { get; }
        public Envelope Envelope { get; }

        /// <summary>Same msg_id seen before: a transport retransmission (expected after reconnects).</summary>
        public bool Duplicate { get; }
    }

    /// <summary>
    /// Plays the Core side of adapter protocol v1 on loopback: handshake, acks, heartbeats, an outbox for
    /// its own reliable messages (byte-identical retransmission), and a record of everything received.
    /// Every frame in both directions is validated against the contract schema.
    /// </summary>
    public sealed class FakeCoreServer : IAsyncDisposable
    {
        public const string Path = "/adapter/v1";

        private readonly object _gate = new object();
        private readonly List<ReceivedMessage> _received = new List<ReceivedMessage>();
        private readonly HashSet<string> _seenMsgIds = new HashSet<string>(StringComparer.Ordinal);
        private readonly List<(string MsgId, string Frame, DateTime SentAt)> _outbox = new List<(string, string, DateTime)>();
        private readonly List<string> _violations = new List<string>();
        private readonly string _token;
        private readonly Action<string> _trace;
        private WebApplication? _app;
        private Session? _session;
        private int _sessionCount;

        public FakeCoreServer(IReadOnlyList<FakeAccount> accounts, string token, Action<string>? trace = null)
        {
            Accounts = accounts;
            _token = token;
            _trace = trace ?? (_ => { });
        }

        public IReadOnlyList<FakeAccount> Accounts { get; }
        public Uri? Endpoint { get; private set; }
        public int HeartbeatIntervalMs { get; set; } = 5000;
        public int AckTimeoutMs { get; set; } = 3000;
        public bool SimulationOnly { get; set; } = true;
        public Locale Locale { get; set; } = Locale.PtBr;

        /// <summary>When true, adapter messages are recorded but not acked (forces retransmission).</summary>
        public bool DropAcks { get; set; }

        public int SessionCount { get { lock (_gate) return _sessionCount; } }

        /// <summary>The adapter sent its mandatory reconnect snapshot in the current session.</summary>
        public bool SessionReady { get { lock (_gate) return _session?.SnapshotReceived == true; } }

        public IReadOnlyList<ReceivedMessage> Received { get { lock (_gate) return _received.ToList(); } }
        public IReadOnlyList<string> Violations { get { lock (_gate) return _violations.ToList(); } }
        public int PendingOutbound { get { lock (_gate) return _outbox.Count; } }

        public async Task StartAsync(int port = 0)
        {
            var builder = WebApplication.CreateSlimBuilder();
            builder.Logging.ClearProviders();
            // Loopback only: nothing is exposed and no firewall port is involved.
            builder.WebHost.ConfigureKestrel(k => k.Listen(IPAddress.Loopback, port));
            var app = builder.Build();
            app.UseWebSockets();
            app.Map(Path, HandleAsync);
            await app.StartAsync().ConfigureAwait(false);
            var address = app.Services.GetRequiredService<Microsoft.AspNetCore.Hosting.Server.IServer>().Features.Get<IServerAddressesFeature>()!.Addresses.First();
            Endpoint = new Uri(address.Replace("http://", "ws://") + Path);
            _app = app;
        }

        public async ValueTask DisposeAsync()
        {
            if (_app != null) await _app.DisposeAsync().ConfigureAwait(false);
        }

        // ------------------------------------------------------------------ Core → adapter

        /// <summary>Sends a reliable message and keeps it until acked; retransmissions are byte-identical.</summary>
        public Task<string> SendReliableAsync(object payload, string? msgId = null, DateTime? sentAt = null)
        {
            var id = msgId ?? Ids.New();
            var frame = ProtocolCodec.Encode(payload, id, sentAt ?? DateTime.UtcNow);
            lock (_gate) _outbox.Add((id, frame, DateTime.UtcNow));
            return SendRawAsync(frame).ContinueWith(_ => id, TaskScheduler.Default);
        }

        public Task SendRawAsync(string frame)
        {
            Session? session;
            lock (_gate) session = _session;
            return session?.SendAsync(frame) ?? Task.CompletedTask;
        }

        /// <summary>Re-sends an earlier frame verbatim (same msg_id, same sent_at).</summary>
        public Task ResendAsync(string msgId)
        {
            string? frame;
            lock (_gate) frame = _outbox.Concat(_sentLog).FirstOrDefault(o => o.MsgId == msgId).Frame;
            return frame == null ? throw new InvalidOperationException("unknown msg_id") : SendRawAsync(frame);
        }

        private readonly List<(string MsgId, string Frame, DateTime SentAt)> _sentLog = new List<(string, string, DateTime)>();

        public async Task CloseAsync(int code)
        {
            Session? session;
            lock (_gate) session = _session;
            if (session != null) await session.CloseAsync(code).ConfigureAwait(false);
        }

        // ------------------------------------------------------------------ session handling

        private async Task HandleAsync(HttpContext context)
        {
            if (!context.WebSockets.IsWebSocketRequest)
            {
                context.Response.StatusCode = 400;
                return;
            }
            var auth = context.Request.Headers.Authorization.ToString();
            using var socket = await context.WebSockets.AcceptWebSocketAsync().ConfigureAwait(false);
            if (auth != "Bearer " + _token)
            {
                // Never echo or log the presented token.
                _trace("auth rejected");
                await socket.CloseAsync((WebSocketCloseStatus)4001, "invalid token", CancellationToken.None).ConfigureAwait(false);
                return;
            }

            Session session;
            lock (_gate)
            {
                _session?.Supersede();
                session = new Session(this, socket, ++_sessionCount);
                _session = session;
            }
            _trace("session " + session.Number + " open");
            using var heartbeat = new Timer(_ => _ = session.SendAsync(ProtocolCodec.Encode(new HeartbeatPayload(), Ids.New(), DateTime.UtcNow)), null, HeartbeatIntervalMs, HeartbeatIntervalMs);
            using var retransmit = new Timer(_ => RetransmitDue(session), null, AckTimeoutMs, AckTimeoutMs);
            try
            {
                await session.ReceiveLoopAsync().ConfigureAwait(false);
            }
            catch (Exception e) when (e is WebSocketException || e is OperationCanceledException)
            {
                _trace("session " + session.Number + " error " + e.GetType().Name);
            }
            lock (_gate)
            {
                if (_session == session) _session = null;
            }
            _trace("session " + session.Number + " closed");
        }

        private void RetransmitDue(Session session)
        {
            List<string> due;
            lock (_gate)
            {
                if (_session != session || !session.SnapshotReceived) return;
                var cutoff = DateTime.UtcNow.AddMilliseconds(-AckTimeoutMs);
                due = _outbox.Where(o => o.SentAt < cutoff).Select(o => o.Frame).ToList();
            }
            foreach (var frame in due) _ = session.SendAsync(frame);
        }

        private void OnFrame(Session session, string text)
        {
            var decoded = ProtocolCodec.Decode(text);
            if (!decoded.IsOk)
            {
                lock (_gate) _violations.Add("invalid frame from adapter: " + decoded.ErrorCode + " " + decoded.Detail);
                _ = session.SendAsync(ProtocolCodec.Encode(new ErrorPayload { Code = decoded.ErrorCode!, RefMsgId = decoded.RefMsgId }, Ids.New(), DateTime.UtcNow));
                return;
            }
            var envelope = decoded.Envelope!;
            bool duplicate;
            lock (_gate)
            {
                duplicate = !_seenMsgIds.Add(envelope.MsgId);
                _received.Add(new ReceivedMessage(session.Number, envelope, duplicate));
            }
            _trace("<- " + envelope.Type + (duplicate ? " (dup)" : "") + " " + Summary(envelope));

            switch (envelope.Type)
            {
                case MessageTypes.Hello:
                    if (session.HelloReceived) { lock (_gate) _violations.Add("second hello in a session"); return; }
                    session.HelloReceived = true;
                    _ = SendHandshakeAsync(session);
                    return;
                case MessageTypes.Ack:
                    var acked = envelope.PayloadAs<AckPayload>().MsgIds;
                    lock (_gate) _outbox.RemoveAll(o => acked.Contains(o.MsgId));
                    return;
                case MessageTypes.Heartbeat:
                case MessageTypes.Error:
                case MessageTypes.RiskState:
                    return;
            }
            if (!MessageTypes.IsReliable(envelope.Type))
            {
                lock (_gate) _violations.Add("unexpected type from adapter: " + envelope.Type);
                return;
            }
            if (!session.HelloReceived) lock (_gate) _violations.Add("reliable message before hello");
            if (envelope.Type == MessageTypes.PositionSnapshot && envelope.PayloadAs<PositionSnapshotPayload>().Trigger == SnapshotTrigger.Reconnect)
            {
                lock (_gate) session.SnapshotReceived = true;
                // Our own unacked messages from the previous session, in original order and byte-identical.
                List<string> pending;
                lock (_gate) pending = _outbox.Select(o => o.Frame).ToList();
                foreach (var frame in pending) _ = session.SendAsync(frame);
            }
            if (!DropAcks) _ = session.SendAsync(ProtocolCodec.Encode(new AckPayload { MsgIds = { envelope.MsgId } }, Ids.New(), DateTime.UtcNow));
        }

        private async Task SendHandshakeAsync(Session session)
        {
            var welcome = new WelcomePayload
            {
                SessionId = Ids.New(),
                ConnectionId = "0192b3c4-0000-7000-8000-00000000c0de",
                ProtocolVersion = ProtocolCodec.Version,
                HeartbeatIntervalMs = HeartbeatIntervalMs,
                AckTimeoutMs = AckTimeoutMs,
                SimulationOnly = SimulationOnly,
                Locale = Locale,
            };
            foreach (var a in Accounts)
                welcome.Accounts.Add(new WelcomeAccount { AccountId = a.AccountId, PlatformAccount = a.PlatformAccount, Roles = a.Roles.ToList(), SyncState = a.SyncState, SyncReasons = a.SyncReasons.ToList() });
            await session.SendAsync(ProtocolCodec.Encode(welcome, Ids.New(), DateTime.UtcNow)).ConfigureAwait(false);
            var limits = new LimitsPayload();
            foreach (var a in Accounts)
            {
                limits.Accounts.Add(new AccountLimits
                {
                    AccountId = a.AccountId, PlatformAccount = a.PlatformAccount, Version = 1, EnforcementEnabled = false, MaxPosition = 10,
                    TradingDay = new TradingDaySpec { ResetLocalTime = "17:00", TimeZone = "America/Chicago" }, WarningThresholdPct = 80,
                });
            }
            await SendReliableAsync(limits).ConfigureAwait(false);
        }

        private static string Summary(Envelope e)
        {
            switch (e.Type)
            {
                case MessageTypes.ExecutionReport:
                    var r = e.PayloadAs<ExecutionReportPayload>();
                    return (r.CommandId ?? "external") + " " + r.ChildRole + " " + r.Status + (r.ReasonCode != null ? " " + r.ReasonCode : "")
                        + (r.Fill != null ? " fill " + r.Fill.Quantity + "@" + DecimalText.Format(r.Fill.Price) : "");
                case MessageTypes.PositionSnapshot:
                    var s = e.PayloadAs<PositionSnapshotPayload>();
                    return s.Trigger + " " + string.Join(";", s.Accounts.Select(a => a.PlatformAccount + " pos=" + string.Join(",", a.Positions.Select(p => p.Position.Quantity)) + " orders=" + a.WorkingOrders.Count));
                default:
                    return "";
            }
        }

        private sealed class Session
        {
            private readonly FakeCoreServer _server;
            private readonly WebSocket _socket;
            private readonly SemaphoreSlim _send = new SemaphoreSlim(1, 1);

            public Session(FakeCoreServer server, WebSocket socket, int number)
            {
                _server = server;
                _socket = socket;
                Number = number;
            }

            public int Number { get; }
            public bool HelloReceived { get; set; }
            public bool SnapshotReceived { get; set; }

            public async Task SendAsync(string frame)
            {
                await _send.WaitAsync().ConfigureAwait(false);
                try
                {
                    if (_socket.State != WebSocketState.Open) return;
                    lock (_server._gate)
                    {
                        var decoded = ProtocolCodec.Decode(frame);
                        if (decoded.IsOk) _server._sentLog.Add((decoded.Envelope!.MsgId, frame, DateTime.UtcNow));
                    }
                    await _socket.SendAsync(Encoding.UTF8.GetBytes(frame), WebSocketMessageType.Text, true, CancellationToken.None).ConfigureAwait(false);
                    var type = ProtocolCodec.Decode(frame).Envelope?.Type;
                    if (type != MessageTypes.Heartbeat && type != MessageTypes.Ack) _server._trace("-> " + type);
                }
                catch (WebSocketException) { }
                finally { _send.Release(); }
            }

            public async Task CloseAsync(int code)
            {
                await _send.WaitAsync().ConfigureAwait(false);
                try
                {
                    if (_socket.State == WebSocketState.Open)
                        await _socket.CloseOutputAsync((WebSocketCloseStatus)code, "fake core close", CancellationToken.None).ConfigureAwait(false);
                }
                catch (WebSocketException) { }
                finally { _send.Release(); }
            }

            public void Supersede() => _ = CloseAsync(4004);

            public async Task ReceiveLoopAsync()
            {
                var buffer = new byte[64 * 1024];
                var message = new List<byte>();
                while (_socket.State == WebSocketState.Open || _socket.State == WebSocketState.CloseSent)
                {
                    var result = await _socket.ReceiveAsync(buffer, CancellationToken.None).ConfigureAwait(false);
                    if (result.MessageType == WebSocketMessageType.Close)
                    {
                        if (_socket.State == WebSocketState.CloseReceived)
                            await _socket.CloseOutputAsync(WebSocketCloseStatus.NormalClosure, "", CancellationToken.None).ConfigureAwait(false);
                        return;
                    }
                    message.AddRange(new ArraySegment<byte>(buffer, 0, result.Count));
                    if (!result.EndOfMessage) continue;
                    _server.OnFrame(this, Encoding.UTF8.GetString(message.ToArray()));
                    message.Clear();
                }
            }
        }
    }
}
