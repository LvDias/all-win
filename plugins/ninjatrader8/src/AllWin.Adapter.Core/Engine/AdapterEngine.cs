using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;
using AllWin.Adapter.Core.Store;

namespace AllWin.Adapter.Core.Engine
{
    public enum SessionState { Offline, AwaitingWelcome, Ready, Stopped }

    public enum StopReason { None, PairAgain, UpdatePlugin, ConnectedElsewhere, StoreFailure, InternalError }

    /// <summary>Outbound side of the WebSocket. Implementations serialize sends (one at a time).</summary>
    public interface IFrameSink
    {
        void Send(string frame);
        void Close(int code, string reason);
    }

    /// <summary>
    /// Client state machine of adapter protocol v1 (ADR-0004). Single-threaded: every input (frames,
    /// platform events, timer ticks) must be delivered from one thread, e.g. through
    /// <see cref="EngineLoop"/>. State changes are written to the durable store before any frame is
    /// sent and before any platform call they lead to (write-ahead), so a crash can lose a message
    /// but never repeat an execution.
    /// </summary>
    public sealed partial class AdapterEngine
    {
        public static readonly IReadOnlyList<string> SupportedProtocolVersions = ProtocolCodec.SupportedVersions;

        private readonly EngineOptions _options;
        private readonly ITradingPlatform _platform;
        private readonly IDurableStore _store;
        private readonly IClock _clock;
        private readonly IAdapterLog _log;
        private readonly ILiveAccountConsent _consent;

        private readonly List<StoreOp> _pending = new List<StoreOp>();
        private readonly List<Action> _afterCommit = new List<Action>();

        private readonly SortedDictionary<long, OutboxEntry> _outbox = new SortedDictionary<long, OutboxEntry>();
        private readonly Dictionary<string, long> _outboxByMsgId = new Dictionary<string, long>(StringComparer.Ordinal);
        private readonly Dictionary<string, DateTime> _inbound = new Dictionary<string, DateTime>(StringComparer.Ordinal);
        private readonly Dictionary<string, CommandRecord> _commands = new Dictionary<string, CommandRecord>(StringComparer.Ordinal);
        private readonly Dictionary<string, List<string>> _queues = new Dictionary<string, List<string>>(StringComparer.Ordinal);
        private readonly HashSet<string> _pumping = new HashSet<string>(StringComparer.Ordinal);
        private readonly Dictionary<string, long> _connectionEpochs = new Dictionary<string, long>(StringComparer.Ordinal);
        private readonly HashSet<string> _deferredFlattens = new HashSet<string>(StringComparer.Ordinal);
        private readonly Dictionary<string, FillRecord> _fills = new Dictionary<string, FillRecord>(StringComparer.Ordinal);
        private readonly Dictionary<string, AccountLimits> _limits = new Dictionary<string, AccountLimits>(StringComparer.Ordinal);
        private readonly Dictionary<string, string> _owners = new Dictionary<string, string>(StringComparer.Ordinal);
        private readonly Dictionary<string, TrackedOrder> _orders = new Dictionary<string, TrackedOrder>(StringComparer.Ordinal);
        private readonly Dictionary<string, HashSet<string>> _stickyLocalReasons = new Dictionary<string, HashSet<string>>(StringComparer.Ordinal);
        private readonly Dictionary<string, AccountCoreState> _coreState = new Dictionary<string, AccountCoreState>(StringComparer.Ordinal);

        private long _nextOutboxSeq = 1;
        private long _nextCommandSeq = 1;
        private string _instanceId = "";
        private WelcomePayload? _welcome;

        /// <summary>Envelope version for frames created in this session (answer F); "1.0" before welcome.</summary>
        private string _sessionVersion = ProtocolCodec.Version;
        private IFrameSink? _sink;
        private TimeSpan _skew;
        private int _heartbeatMs;
        private int _ackTimeoutMs;
        private DateTime _lastFrameReceivedAt;
        private DateTime _lastFrameSentAt;
        private DateTime _lastPeriodicSnapshotAt;
        private DateTime _lastPruneAt;
        private bool _started;
        private bool _faulted;

        /// <summary>True while executions that happened during a restart are being caught up (answer A).</summary>
        private bool _catchUp;

        /// <param name="liveConsent">
        /// The local "Allow live accounts" opt-in (ADR-0005 N11); null means off with nothing confirmed. Release builds only.
        /// </param>
        public AdapterEngine(EngineOptions options, ITradingPlatform platform, IDurableStore store, IClock? clock = null, IAdapterLog? log = null,
            ILiveAccountConsent? liveConsent = null)
        {
            _consent = liveConsent ?? LiveConsentState.Off();
            _options = options;
            _platform = platform;
            _store = store;
            _clock = clock ?? SystemClock.Instance;
            _log = log ?? NullLog.Instance;
            _heartbeatMs = options.DefaultHeartbeatIntervalMs;
            _ackTimeoutMs = options.DefaultAckTimeoutMs;
        }

        /// <summary>Raised (on the engine thread) after anything the plugin window shows has changed.</summary>
        public event Action? Changed;

        /// <summary>Raised for instruction results addressed to master accounts (S19).</summary>
        public event Action<InstructionResultPayload>? InstructionResultReceived;

        public SessionState Session { get; private set; } = SessionState.Offline;
        public StopReason StopReason { get; private set; } = StopReason.None;
        public string InstanceId => _instanceId;
        public Locale? CoreLocale => _welcome?.Locale;
        public int PendingOutbound => _outbox.Count;
        public int HeartbeatIntervalMs => _heartbeatMs;

        /// <summary>
        /// PC and Core clocks differ by more than MaxBracketDelay (measured at welcome and on every Core
        /// heartbeat): the plugin shows a clock warning (ADR-0004 §5).
        /// </summary>
        public bool ClockSkewWarning { get; private set; }

        /// <summary>The store dropped an unconfirmed last write on open (Architect L3): shown as a notice.</summary>
        public bool StoreRecoveredTornWrite { get; private set; }

        /// <summary>True while the transport supervisor should keep reconnecting (ADR-0004 §4).</summary>
        public bool ShouldReconnect => _started && !_faulted && Session != SessionState.Stopped;

        /// <summary>Skew-corrected estimate of the Core clock (ADR-0004 §5).</summary>
        public DateTime CoreNow => _clock.UtcNow + _skew;

        // ------------------------------------------------------------------ lifecycle

        /// <summary>Loads the durable store and reconciles with the platform. Call once, before connecting.</summary>
        public void Start()
        {
            Run("start", () =>
            {
                LoadState();
                _started = true;
                Recover();
                EnforceLiveTable();
                Prune();
            });
        }

        // ------------------------------------------------------------------ transport inputs

        public void OnConnected(IFrameSink sink)
        {
            Run("connected", () =>
            {
                if (!_started || Session == SessionState.Stopped) { sink.Close(1000, "stopped"); return; }
                _sink = sink;
                Session = SessionState.AwaitingWelcome;
                _sessionVersion = ProtocolCodec.Version;
                var now = _clock.UtcNow;
                _lastFrameReceivedAt = now;
                SendNow(BuildHello());
                RaiseChanged();
            });
        }

        public void OnFrame(string text)
        {
            Run("frame", () =>
            {
                if (_sink == null) return;
                _lastFrameReceivedAt = _clock.UtcNow;
                var decoded = ProtocolCodec.Decode(text);
                if (!decoded.IsOk)
                {
                    _log.Write(LogLevel.Warning, "protocol.invalid_frame", decoded.ErrorCode + " ref=" + decoded.RefMsgId + " " + decoded.Detail);
                    SendNow(new ErrorPayload { Code = decoded.ErrorCode!, RefMsgId = decoded.RefMsgId, Detail = decoded.Detail });
                    return;
                }
                Dispatch(decoded.Envelope!);
            });
        }

        /// <summary>The WebSocket closed; <paramref name="closeCode"/> is the server's close code if any.</summary>
        public void OnDisconnected(int? closeCode)
        {
            Run("disconnected", () =>
            {
                if (_sink == null && Session != SessionState.AwaitingWelcome && Session != SessionState.Ready) return;
                _sink = null;
                switch (closeCode)
                {
                    case 4001:
                    case 4002: Stop(StopReason.PairAgain); break;
                    case 4003: Stop(StopReason.UpdatePlugin); break;
                    case 4004: Stop(StopReason.ConnectedElsewhere); break;
                    default: Session = SessionState.Offline; break;
                }
                foreach (var entry in _outbox.Values) entry.LastSentAt = null;
                _log.Write(LogLevel.Info, "session.closed", "code=" + (closeCode?.ToString(CultureInfo.InvariantCulture) ?? "none") + " state=" + Session);
                RaiseChanged();
            });
        }

        // ------------------------------------------------------------------ timer

        /// <summary>Call about every 250 ms: heartbeats, ack retransmission, periodic snapshot, timeouts, pruning.</summary>
        public void OnTimer()
        {
            Run("timer", () =>
            {
                if (!_started) return;
                var now = _clock.UtcNow;
                if (_sink != null && (Session == SessionState.AwaitingWelcome || Session == SessionState.Ready))
                {
                    if ((now - _lastFrameReceivedAt).TotalMilliseconds > 3.0 * _heartbeatMs)
                    {
                        _log.Write(LogLevel.Warning, "session.heartbeat_timeout", "closing");
                        var sink = _sink;
                        OnDisconnectedCore();
                        sink.Close(4005, "heartbeat timeout");
                    }
                    else if ((now - _lastFrameSentAt).TotalMilliseconds >= _heartbeatMs) SendNow(new HeartbeatPayload());
                }

                if (Session == SessionState.Ready)
                {
                    foreach (var entry in _outbox.Values)
                        if (entry.LastSentAt.HasValue && (now - entry.LastSentAt.Value).TotalMilliseconds >= _ackTimeoutMs)
                            Transmit(entry);
                    if (now - _lastPeriodicSnapshotAt >= _options.PeriodicSnapshotInterval)
                    {
                        _lastPeriodicSnapshotAt = now;
                        EnqueueSnapshot(SnapshotTrigger.Periodic, null);
                    }
                }

                CheckFlattenTimeouts(now);
                if (now - _lastPruneAt >= _options.PruneInterval) Prune();
            });
        }

        private void OnDisconnectedCore()
        {
            _sink = null;
            Session = SessionState.Offline;
            foreach (var entry in _outbox.Values) entry.LastSentAt = null;
            RaiseChanged();
        }

        // ------------------------------------------------------------------ inbound dispatch

        private void Dispatch(Envelope envelope)
        {
            switch (envelope.Type)
            {
                case MessageTypes.Welcome:
                    HandleWelcome(envelope);
                    break;
                case MessageTypes.Error:
                    var error = envelope.PayloadAs<ErrorPayload>();
                    _log.Write(LogLevel.Warning, "core.error", error.Code + " ref=" + error.RefMsgId);
                    if (Session == SessionState.AwaitingWelcome && error.Code == ReasonCodes.ProtocolUnsupportedVersion)
                        Stop(StopReason.UpdatePlugin);
                    break;
                case MessageTypes.Ack:
                    HandleAck(envelope.PayloadAs<AckPayload>());
                    break;
                case MessageTypes.Heartbeat:
                    UpdateClockWarning(envelope.SentAt - _clock.UtcNow);
                    break;
                case MessageTypes.AccountState:
                    HandleAccountState(envelope);
                    break;
                case MessageTypes.Command:
                case MessageTypes.Limits:
                case MessageTypes.InstructionResult:
                    HandleReliable(envelope);
                    break;
                default:
                    // A→C message types are never valid from the Core.
                    _log.Write(LogLevel.Warning, "protocol.unexpected_type", envelope.Type);
                    SendNow(new ErrorPayload { Code = ReasonCodes.ProtocolInvalidMessage, RefMsgId = envelope.MsgId, Detail = "unexpected type " + envelope.Type });
                    break;
            }
        }

        private void HandleWelcome(Envelope envelope)
        {
            if (Session != SessionState.AwaitingWelcome)
            {
                _log.Write(LogLevel.Warning, "protocol.unexpected_welcome", Session.ToString());
                return;
            }
            var welcome = envelope.PayloadAs<WelcomePayload>();
            if (!SupportedProtocolVersions.Contains(welcome.ProtocolVersion))
            {
                _log.Write(LogLevel.Error, "protocol.version_mismatch", welcome.ProtocolVersion);
                Stop(StopReason.UpdatePlugin);
                _sink?.Close(1002, "unsupported protocol version");
                _sink = null;
                return;
            }

            var now = _clock.UtcNow;
            _skew = envelope.SentAt - now;
            UpdateClockWarning(_skew);
            _sessionVersion = welcome.ProtocolVersion;
            _heartbeatMs = welcome.HeartbeatIntervalMs;
            _ackTimeoutMs = welcome.AckTimeoutMs;
            _welcome = welcome;
            Put(Collections.Meta, "welcome", welcome);
            foreach (var account in welcome.Accounts)
                ApplyCoreAccountState(account.PlatformAccount, account.SyncState, account.SyncReasons ?? new List<string>());
            // The Core now knows what happened while we were away; its reasons replace our sticky hints.
            _stickyLocalReasons.Clear();
            SaveStickyReasons();
            RefreshTrackedOrders();
            EnforceLiveTable(); // simulation_only may have changed (ADR-0005 §N11: the same table applies)
            Flush();

            Session = SessionState.Ready;
            _log.Write(LogLevel.Info, "session.ready", "session=" + welcome.SessionId + " skew_ms=" + ((long)_skew.TotalMilliseconds).ToString(CultureInfo.InvariantCulture)
                + " pending=" + _outbox.Count.ToString(CultureInfo.InvariantCulture));

            // Architect answer 6: outbox in original order, then the mandatory snapshot, then commands.
            foreach (var entry in _outbox.Values.ToList()) Transmit(entry);
            _lastPeriodicSnapshotAt = now;
            EnqueueSnapshot(SnapshotTrigger.Reconnect, null);
            Flush();
            PumpAll();
            RaiseChanged();
        }

        private void UpdateClockWarning(TimeSpan skew)
        {
            var warn = skew.Duration() > _options.MaxBracketDelay;
            if (warn == ClockSkewWarning) return;
            ClockSkewWarning = warn;
            _log.Write(warn ? LogLevel.Warning : LogLevel.Info, "clock.skew", ((long)skew.TotalMilliseconds).ToString(CultureInfo.InvariantCulture) + " ms");
            RaiseChanged();
        }

        /// <summary>Protocol 1.1 latest-wins sync state; only valid on a session that negotiated 1.1.</summary>
        private void HandleAccountState(Envelope envelope)
        {
            if (Session != SessionState.Ready || _sessionVersion == "1.0")
            {
                // A type the negotiated version does not have (e.g. after a Core rollback to 1.0): refused, not applied.
                _log.Write(LogLevel.Warning, "protocol.unexpected_account_state", _sessionVersion);
                SendNow(new ErrorPayload { Code = ReasonCodes.ProtocolUnsupportedValue, RefMsgId = envelope.MsgId, Detail = "account_state needs protocol 1.1" });
                return;
            }
            var state = envelope.PayloadAs<AccountStatePayload>();
            var mapping = MappingOf(state.PlatformAccount);
            if (mapping == null || mapping.AccountId != state.AccountId)
            {
                _log.Write(LogLevel.Warning, "account_state.unmapped", state.AccountId);
                return;
            }
            ApplyCoreAccountState(state.PlatformAccount, state.SyncState, state.SyncReasons, state.ChangedAt);
        }

        private void HandleAck(AckPayload ack)
        {
            foreach (var msgId in ack.MsgIds)
            {
                if (!_outboxByMsgId.TryGetValue(msgId, out var seq)) continue;
                _outboxByMsgId.Remove(msgId);
                _outbox.Remove(seq);
                _pending.Add(new StoreOp(Collections.Outbox, SeqKey(seq), null));
            }
            RaiseChanged();
        }

        private void HandleReliable(Envelope envelope)
        {
            if (_inbound.ContainsKey(envelope.MsgId))
            {
                SendAck(envelope.MsgId); // transport duplicate: ack again, nothing else
                return;
            }
            if (Session != SessionState.Ready)
                _log.Write(LogLevel.Warning, "protocol.message_before_snapshot", envelope.Type + " " + envelope.MsgId);

            string? pumpAccount = null;
            switch (envelope.Type)
            {
                case MessageTypes.Command:
                    pumpAccount = IntakeCommand(envelope.PayloadAs<CommandPayload>(), envelope.SentAt, envelope.ProtocolVersion);
                    break;
                case MessageTypes.Limits:
                    ApplyLimits(envelope.PayloadAs<LimitsPayload>());
                    break;
                case MessageTypes.InstructionResult:
                    var result = envelope.PayloadAs<InstructionResultPayload>();
                    _log.Write(LogLevel.Info, "instruction.result", result.MessageId + " " + result.Status);
                    InstructionResultReceived?.Invoke(result);
                    break;
            }
            var now = _clock.UtcNow;
            _inbound[envelope.MsgId] = now;
            _pending.Add(new StoreOp(Collections.Inbound, envelope.MsgId, UtcTimestamp.Format(now)));
            Flush();
            SendAck(envelope.MsgId); // ack = durably recorded (ADR-0004 §4)
            if (pumpAccount != null) Pump(pumpAccount);
        }

        private void ApplyLimits(LimitsPayload payload)
        {
            foreach (var limits in payload.Accounts)
            {
                if (_limits.TryGetValue(limits.AccountId, out var current) && current.Version >= limits.Version) continue;
                _limits[limits.AccountId] = limits;
                Put(Collections.Limits, limits.AccountId, limits);
            }
        }

        /// <summary>
        /// Applies the Core's view of one account. Today it comes from <c>welcome</c>; protocol 1.1 adds
        /// <c>account_state</c>, which will call this too (Architect answer 3).
        /// </summary>
        /// <param name="changedAt">
        /// account_state.changed_at; null for welcome (a new session resets the view). An update older than the
        /// one held is ignored; an equal one wins because it was received later (Architect, 1.1 rule).
        /// </param>
        public void ApplyCoreAccountState(string platformAccount, SyncState state, IReadOnlyList<string> reasons, DateTime? changedAt = null)
        {
            if (changedAt.HasValue && _coreState.TryGetValue(platformAccount, out var held) && held.ChangedAt.HasValue && changedAt.Value < held.ChangedAt.Value)
                return;
            _coreState[platformAccount] = new AccountCoreState(state, reasons.ToArray(), changedAt);
            RaiseChanged();
        }

        // ------------------------------------------------------------------ outbound

        private HelloPayload BuildHello()
        {
            var hello = new HelloPayload
            {
                InstanceId = _instanceId,
                Adapter = new AdapterInfo
                {
                    Type = _options.AdapterType,
                    Version = _options.AdapterVersion,
                    Build = _options.Build,
                    PlatformVersion = _options.PlatformVersion,
                },
                SupportedProtocolVersions = SupportedProtocolVersions.ToList(),
                PendingOutbound = _outbox.Count,
            };
            foreach (var account in SafeAccounts())
            {
                hello.Accounts.Add(new HelloAccount
                {
                    PlatformAccount = account.Name,
                    ConnectionName = account.ConnectionName,
                    Kind = account.Kind,
                    OcoLocation = account.OcoLocation,
                });
            }
            return hello;
        }

        /// <summary>Stores a reliable message in the outbox; it is sent after the next commit if the session is ready.</summary>
        private void Enqueue(object payload)
        {
            var now = _clock.UtcNow;
            var msgId = Ids.New();
            var entry = new OutboxEntry
            {
                Seq = _nextOutboxSeq++,
                MsgId = msgId,
                Type = ProtocolCodec.TypeOf(payload),
                Frame = ProtocolCodec.Encode(payload, msgId, now, _sessionVersion), // kept byte-identical for retransmission
                CreatedAt = now,
            };
            _outbox[entry.Seq] = entry;
            _outboxByMsgId[msgId] = entry.Seq;
            Put(Collections.Outbox, SeqKey(entry.Seq), entry);
            _afterCommit.Add(() =>
            {
                if (Session == SessionState.Ready && _outbox.ContainsKey(entry.Seq) && !entry.LastSentAt.HasValue) Transmit(entry);
            });
        }

        private void Transmit(OutboxEntry entry)
        {
            if (_sink == null || Session != SessionState.Ready) return;
            entry.LastSentAt = _clock.UtcNow;
            SendRaw(entry.Frame);
        }

        private void SendNow(object payload)
        {
            if (_sink == null) return;
            // hello always goes out as 1.0 (nothing is negotiated yet); later frames use the session's version.
            var version = payload is HelloPayload ? ProtocolCodec.Version : _sessionVersion;
            SendRaw(ProtocolCodec.Encode(payload, Ids.New(), _clock.UtcNow, version));
        }

        private void SendAck(string msgId) => SendNow(new AckPayload { MsgIds = { msgId } });

        private void SendRaw(string frame)
        {
            var sink = _sink;
            if (sink == null) return;
            _lastFrameSentAt = _clock.UtcNow;
            sink.Send(frame);
        }

        /// <summary>Latest-wins live state for display (not acked, not stored).</summary>
        public void PublishRiskState(RiskStatePayload state)
        {
            Run("risk_state", () => { if (Session == SessionState.Ready) SendNow(state); });
        }

        // ------------------------------------------------------------------ persistence plumbing

        private void Run(string what, Action action)
        {
            if (_faulted) return;
            try
            {
                action();
                Flush();
            }
            catch (Exception e) when (e is System.IO.IOException || e is StoreCorruptException || e is UnauthorizedAccessException)
            {
                // Fail closed: without a durable store we cannot guarantee exactly-once execution.
                _faulted = true;
                _pending.Clear();
                _afterCommit.Clear();
                _log.Write(LogLevel.Error, "store.failure", what + ": " + e.GetType().Name);
                Stop(StopReason.StoreFailure);
                try { _sink?.Close(1011, "adapter store failure"); } catch (Exception) { }
                _sink = null;
                RaiseChanged();
            }
            catch (Exception e)
            {
                // An unexpected exception is a bug: the in-memory state may be half-applied. Nothing more is
                // committed or sent and the adapter stops trading (fail closed); a restart recovers from the
                // last durable state. Platform-call failures never get here: they are mapped to reports.
                _faulted = true;
                _pending.Clear();
                _afterCommit.Clear();
                _log.Write(LogLevel.Error, "engine.fault", what + ": " + e.GetType().Name + ": " + e.Message + "\n" + e.StackTrace);
                Stop(StopReason.InternalError);
                try { _sink?.Close(1011, "adapter internal error"); } catch (Exception) { }
                _sink = null;
                RaiseChanged();
            }
        }

        private void Flush()
        {
            if (_pending.Count > 0)
            {
                var ops = _pending.ToArray();
                _pending.Clear();
                _store.Commit(ops);
            }
            while (_afterCommit.Count > 0)
            {
                var actions = _afterCommit.ToArray();
                _afterCommit.Clear();
                foreach (var a in actions) a();
            }
        }

        private void Put(string collection, string key, object value) => _pending.Add(new StoreOp(collection, key, StoreJson.Write(value)));

        private void Save(CommandRecord record) => Put(Collections.Commands, record.CommandId, record);

        private static string SeqKey(long seq) => seq.ToString("D20", CultureInfo.InvariantCulture);

        private void Stop(StopReason reason)
        {
            Session = SessionState.Stopped;
            StopReason = reason;
            _log.Write(LogLevel.Warning, "session.stopped", reason.ToString());
        }

        private void RaiseChanged()
        {
            try { Changed?.Invoke(); }
            catch (Exception e) { _log.Write(LogLevel.Error, "observer.error", e.GetType().Name); }
        }

        private void LoadState()
        {
            StoreRecoveredTornWrite = _store.RecoveredFromTornWrite;
            if (StoreRecoveredTornWrite) _log.Write(LogLevel.Warning, "store.torn_tail_dropped", "an unconfirmed last write was discarded");
            if (_store.Get(Collections.Meta, "connection_epochs") is string epochs)
                foreach (var pair in StoreJson.Read<Dictionary<string, long>>(epochs)) _connectionEpochs[pair.Key] = pair.Value;
            _instanceId = _store.Get(Collections.Meta, "instance_id") is string id ? StoreJson.Read<string>(id) : "";
            if (_instanceId.Length == 0)
            {
                _instanceId = Ids.New();
                Put(Collections.Meta, "instance_id", _instanceId);
            }
            if (_store.Get(Collections.Meta, "local_reasons") is string reasons)
                foreach (var pair in StoreJson.Read<Dictionary<string, List<string>>>(reasons))
                    _stickyLocalReasons[pair.Key] = new HashSet<string>(pair.Value, StringComparer.Ordinal);
            var welcome = _store.Get(Collections.Meta, "welcome");
            if (welcome != null) _welcome = StoreJson.Read<WelcomePayload>(welcome);

            foreach (var pair in _store.Collection(Collections.Outbox).OrderBy(p => p.Key, StringComparer.Ordinal))
            {
                var entry = StoreJson.Read<OutboxEntry>(pair.Value);
                _outbox[entry.Seq] = entry;
                _outboxByMsgId[entry.MsgId] = entry.Seq;
                _nextOutboxSeq = Math.Max(_nextOutboxSeq, entry.Seq + 1);
            }
            if (_store.Get(Collections.Meta, "next_outbox_seq") is string next)
                _nextOutboxSeq = Math.Max(_nextOutboxSeq, StoreJson.Read<long>(next));
            foreach (var pair in _store.Collection(Collections.Inbound))
                _inbound[pair.Key] = UtcTimestamp.Parse(pair.Value);
            foreach (var pair in _store.Collection(Collections.Commands))
            {
                var record = StoreJson.Read<CommandRecord>(pair.Value);
                _commands[record.CommandId] = record;
                _nextCommandSeq = Math.Max(_nextCommandSeq, record.Seq + 1);
            }
            foreach (var pair in _store.Collection(Collections.Fills)) _fills[pair.Key] = StoreJson.Read<FillRecord>(pair.Value);
            foreach (var pair in _store.Collection(Collections.Limits)) _limits[pair.Key] = StoreJson.Read<AccountLimits>(pair.Value);
            foreach (var pair in _store.Collection(Collections.Owners)) _owners[pair.Key] = StoreJson.Read<string>(pair.Value);
            foreach (var pair in _store.Collection(Collections.Identities)) _identities[pair.Key] = StoreJson.Read<PositionIdentityRecord>(pair.Value);
            if (_welcome != null)
                foreach (var account in _welcome.Accounts)
                    ApplyCoreAccountState(account.PlatformAccount, account.SyncState, account.SyncReasons ?? new List<string>());
        }

        /// <summary>Drops dedupe records past retention; never touches a command that is not final (answer 8).</summary>
        private void Prune()
        {
            var now = _clock.UtcNow;
            _lastPruneAt = now;
            var cutoff = now - _options.DedupeRetention;
            foreach (var pair in _inbound.Where(p => p.Value < cutoff).ToList())
            {
                _inbound.Remove(pair.Key);
                _pending.Add(new StoreOp(Collections.Inbound, pair.Key, null));
            }
            foreach (var record in _commands.Values.Where(r => r.Phase == CommandPhase.Done && (r.DoneAt ?? r.ReceivedAt) < cutoff && !StillProtecting(r)).ToList())
            {
                _commands.Remove(record.CommandId);
                _pending.Add(new StoreOp(Collections.Commands, record.CommandId, null));
            }
            // A fill record is kept while the platform still holds that execution, so a replay is never reported
            // again. Fills of an account we cannot observe right now are kept too (unknown = keep).
            var heldByPlatform = new HashSet<string>(StringComparer.Ordinal);
            var observedPrefixes = new List<string>();
            foreach (var mapping in _welcome?.Accounts ?? new List<WelcomeAccount>())
            {
                if (!IsObservable(mapping.PlatformAccount)) continue;
                observedPrefixes.Add(mapping.PlatformAccount + "|");
                foreach (var execution in SafeExecutions(mapping.PlatformAccount)) heldByPlatform.Add(mapping.PlatformAccount + "|" + execution.ExecutionId);
            }
            foreach (var pair in _fills.Where(p => p.Value.RecordedAt < cutoff && !heldByPlatform.Contains(p.Key)
                && observedPrefixes.Any(prefix => p.Key.StartsWith(prefix, StringComparison.Ordinal))).ToList())
            {
                _fills.Remove(pair.Key);
                _pending.Add(new StoreOp(Collections.Fills, pair.Key, null));
            }
            // Keep the outbox sequence monotonic even after the outbox drains.
            Put(Collections.Meta, "next_outbox_seq", _nextOutboxSeq);
        }

        /// <summary>
        /// A final place command is still needed while its bracket protects something: live legs, shared
        /// ownership, or an individual entry with open quantity and a position on the contract (Code Review 6 H2).
        /// </summary>
        private bool StillProtecting(CommandRecord record)
        {
            if (record.Command.Kind != CommandKind.Place || record.Command.Place?.Protection == null) return false;
            // Fail-safe (Code Review 6 re-check H3): while the account is absent or disconnected, e.g. NT8 started
            // before its connection, we cannot know whether the bracket still protects anything, so keep it.
            if (!IsObservable(record.PlatformAccount)) return true;
            if (_orders.Values.Any(o => o.CommandId == record.CommandId && o.Account == record.PlatformAccount && !o.Order.IsTerminal)) return true;
            if (_owners.ContainsValue(record.CommandId)) return true;
            var open = record.EntryFilled - LegFilled(record.PlatformAccount, record.CommandId, 's') - LegFilled(record.PlatformAccount, record.CommandId, 't');
            return open > 0 && PositionQuantity(record.PlatformAccount, record.Command.Instrument) != 0;
        }

        /// <summary>The account is visible on the platform and its connection is up.</summary>
        private bool IsObservable(string platformAccount) => SafeAccounts().Any(a => a.Name == platformAccount && a.Connected);

        private IReadOnlyList<PlatformAccountInfo> SafeAccounts()
        {
            try { return _platform.GetAccounts(); }
            catch (PlatformException e)
            {
                _log.Write(LogLevel.Error, "platform.accounts_failed", e.Message);
                return new PlatformAccountInfo[0];
            }
        }

        private sealed class AccountCoreState
        {
            public AccountCoreState(SyncState state, string[] reasons, DateTime? changedAt)
            {
                State = state;
                Reasons = reasons;
                ChangedAt = changedAt;
            }

            public SyncState State { get; }
            public string[] Reasons { get; }
            public DateTime? ChangedAt { get; }
        }
    }
}
