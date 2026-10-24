using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;
using AllWin.Adapter.Core.Store;
using AllWin.Adapter.Sim;
using Newtonsoft.Json.Linq;

namespace AllWin.Adapter.Core.Tests.Engine
{
    public sealed class ManualClock : IClock
    {
        public DateTime UtcNow { get; set; } = new DateTime(2026, 10, 22, 14, 31, 0, DateTimeKind.Utc);
        public void Advance(TimeSpan by) => UtcNow += by;
    }

    public sealed class RecordingSink : IFrameSink
    {
        public List<Envelope> Frames { get; } = new List<Envelope>();

        /// <summary>The frames exactly as sent on the wire, for tests that check field names.</summary>
        public List<string> RawFrames { get; } = new List<string>();
        public List<int> Closes { get; } = new List<int>();

        public void Send(string frame)
        {
            var decoded = ProtocolCodec.Decode(frame);
            if (!decoded.IsOk) throw new InvalidOperationException("Engine sent an invalid frame: " + decoded.Detail);
            // Second opinion from an independent JSON Schema implementation on every frame the engine sends.
            if (!Contracts.ReferenceSchema.IsValidAdapterFrame(frame)) throw new InvalidOperationException("Reference validator rejects an engine frame: " + frame);
            Frames.Add(decoded.Envelope!);
            RawFrames.Add(frame);
        }

        public void Close(int code, string reason) => Closes.Add(code);
    }

    /// <summary>
    /// Drives one engine with a simulated platform, a manual clock and a real file store, playing the
    /// Core's side by hand. Every frame the engine sends is decoded and schema-validated.
    /// </summary>
    public sealed class EngineHarness : IDisposable
    {
        public const string AccountId = "0192b3c4-0000-7000-8000-000000000001";
        public const string Sim101 = "Sim101";
        public static readonly Instrument Mnq = new Instrument("MNQ", "2026-12");

        private readonly string _dir = Path.Combine(Path.GetTempPath(), "allwin-engine-" + Guid.NewGuid().ToString("N"));
        private readonly Queue<PlatformEvent> _events = new Queue<PlatformEvent>();
        private FileJournalStore _file;
        private FaultInjectingStore _store;

        public EngineHarness(Action<SimulatedPlatform>? setup = null, EngineOptions? options = null, LiveConsentState? consent = null)
        {
            Options = options ?? new EngineOptions();
            Consent = consent ?? LiveConsentState.Off();
            Platform = new SimulatedPlatform(Clock) { EventSink = e => _events.Enqueue(e) };
            Platform.AddAccount(Sim101);
            Platform.SetTickSize("MNQ", 0.25m);
            setup?.Invoke(Platform);
            _file = new FileJournalStore(_dir);
            _store = new FaultInjectingStore(_file);
            Engine = NewEngine();
            Engine.Start();
        }

        /// <summary>The next durable commit throws IOException (disk full, permission lost…).</summary>
        public void FailNextCommit() => _store.FailNext = true;

        public ManualClock Clock { get; } = new ManualClock();
        public SimulatedPlatform Platform { get; }
        public EngineOptions Options { get; }

        /// <summary>The local live-account opt-in the engine reads (ADR-0005 N11).</summary>
        public LiveConsentState Consent { get; }
        public AdapterEngine Engine { get; private set; }
        public RecordingSink Sink { get; private set; } = new RecordingSink();
        public List<string> Log { get; } = new List<string>();

        public void Dispose()
        {
            _file.Dispose();
            if (Directory.Exists(_dir)) Directory.Delete(_dir, true);
        }

        private AdapterEngine NewEngine() => new AdapterEngine(Options, Platform, _store, Clock, new ListLog(Log), Consent);

        // ------------------------------------------------------------------ Core side

        public void Connect(WelcomePayload? welcome = null, TimeSpan? coreClockAhead = null)
        {
            Sink = new RecordingSink();
            Engine.OnConnected(Sink);
            Settle();
            var w = welcome ?? DefaultWelcome();
            Deliver(w, sentAt: Clock.UtcNow + (coreClockAhead ?? TimeSpan.Zero));
        }

        public static WelcomePayload DefaultWelcome(SyncState state = SyncState.InSync, params string[] reasons) => new WelcomePayload
        {
            SessionId = Ids.New(),
            ConnectionId = "0192b3c4-0000-7000-8000-000000000003",
            ProtocolVersion = CoreVersion,
            HeartbeatIntervalMs = 10000,
            AckTimeoutMs = 5000,
            SimulationOnly = true,
            Locale = Locale.PtBr,
            Accounts =
            {
                new WelcomeAccount { AccountId = AccountId, PlatformAccount = Sim101, Roles = { AccountRole.Destination }, SyncState = state, SyncReasons = reasons.ToList() },
            },
        };

        /// <summary>Protocol version of the default welcome and of frames sent by <see cref="Deliver"/>.</summary>
        public const string CoreVersion = "1.2";

        /// <summary>A position_ref that names no position (for reduces that never get to the identity check).</summary>
        public const string NoPosition = "0192b3c4-0000-7000-8000-0000000000ff";

        /// <summary>Sends a Core→adapter message; returns its msg_id.</summary>
        public string Deliver(object payload, string? msgId = null, DateTime? sentAt = null, string version = CoreVersion)
        {
            var id = msgId ?? Ids.New();
            Engine.OnFrame(ProtocolCodec.Encode(payload, id, sentAt ?? Clock.UtcNow, version));
            Settle();
            return id;
        }

        public void DeliverRaw(string text)
        {
            Engine.OnFrame(text);
            Settle();
        }

        /// <summary>Acks every reliable message sent so far (the Core stored them).</summary>
        public void AckAll()
        {
            var ids = Sink.Frames.Where(f => MessageTypes.IsReliable(f.Type)).Select(f => f.MsgId).Distinct().ToList();
            if (ids.Count == 0) return;
            Deliver(new AckPayload { MsgIds = ids });
        }

        public void Disconnect(int? code = 1006)
        {
            Engine.OnDisconnected(code);
            Settle();
        }

        /// <summary>Simulates a plugin/NT8 restart: new engine over the same store and platform.</summary>
        public void Restart(bool renewHandles = true)
        {
            _file.Dispose();
            if (renewHandles) Platform.RenewHandles();
            _events.Clear(); // events of the dead process are lost
            if (_wipeOnRestart)
            {
                _wipeOnRestart = false;
                foreach (var file in Directory.GetFiles(_dir)) File.Delete(file);
            }
            if (_corruptOnRestart)
            {
                _corruptOnRestart = false;
                var journal = Path.Combine(_dir, FileJournalStore.JournalFile);
                var bytes = File.ReadAllBytes(journal);
                bytes[bytes.Length - 5] ^= 0x20;
                File.WriteAllBytes(journal, bytes);
            }
            _file = new FileJournalStore(_dir);
            _store = new FaultInjectingStore(_file);
            Engine = NewEngine();
            Engine.Start();
            Sink = new RecordingSink();
            Settle();
        }

        /// <summary>A restart whose journal was lost (deleted, or a new install): the platform keeps its state.</summary>
        public void RestartWithLostJournal()
        {
            _wipeOnRestart = true;
            Restart();
        }

        private bool _wipeOnRestart;

        public void Tick(TimeSpan? advance = null)
        {
            if (advance.HasValue) Clock.Advance(advance.Value);
            Engine.OnTimer();
            Settle();
        }

        /// <summary>Delivers queued platform events until quiet (events may cause more events).</summary>
        public void Settle()
        {
            var guard = 0;
            while (_events.Count > 0)
            {
                if (++guard > 10000) throw new InvalidOperationException("platform event storm");
                Engine.OnPlatformEvent(_events.Dequeue());
            }
        }

        // ------------------------------------------------------------------ inspection

        public IEnumerable<Envelope> Sent(string type) => Sink.Frames.Where(f => f.Type == type);

        public List<ExecutionReportPayload> Reports(string? commandId = null) => Sent(MessageTypes.ExecutionReport)
            .GroupBy(f => f.MsgId).Select(g => g.First())
            .Select(f => f.PayloadAs<ExecutionReportPayload>())
            .Where(r => commandId == null || r.CommandId == commandId).ToList();

        public List<PositionSnapshotPayload> Snapshots() => Sent(MessageTypes.PositionSnapshot)
            .GroupBy(f => f.MsgId).Select(g => g.First()).Select(f => f.PayloadAs<PositionSnapshotPayload>()).ToList();

        public List<string> Acks() => Sent(MessageTypes.Ack).SelectMany(f => f.PayloadAs<AckPayload>().MsgIds).ToList();

        // ------------------------------------------------------------------ command builders

        public static CommandPayload Place(string commandId, Side side, int quantity, PositionEffect effect = PositionEffect.Open,
            ProtectionSpec? protection = null, DateTime? expiresAt = null, string? dependsOn = null, OrderSpec? order = null, string? positionRef = null)
        {
            return new CommandPayload
            {
                CommandId = commandId,
                InstructionId = "0192b3c4-0000-7000-8000-000000000006",
                AccountId = AccountId,
                PlatformAccount = Sim101,
                Instrument = Mnq.Clone(),
                Kind = CommandKind.Place,
                ExpiresAt = effect == PositionEffect.Open ? expiresAt ?? new DateTime(2026, 10, 22, 14, 31, 5, DateTimeKind.Utc) : expiresAt,
                DependsOn = dependsOn,
                Refs = effect == PositionEffect.Reduce ? new CommandRefs { PositionRef = positionRef ?? NoPosition } : null,
                Place = new PlaceSpec { Side = side, Quantity = quantity, Order = order ?? new OrderSpec { Type = OrderType.Market }, PositionEffect = effect, Protection = protection },
            };
        }

        public static ProtectionSpec Individual(int stop = 40, int target = 80) => new ProtectionSpec { Mode = ProtectionMode.Individual, StopTicks = stop, TargetTicks = target };
        public static ProtectionSpec Shared(int stop = 40, int target = 80) => new ProtectionSpec { Mode = ProtectionMode.Shared, StopTicks = stop, TargetTicks = target };

        public static CommandPayload Flatten(string commandId, FlattenReason reason = FlattenReason.Close) => new CommandPayload
        {
            CommandId = commandId,
            InstructionId = "0192b3c4-0000-7000-8000-000000000006",
            AccountId = AccountId,
            PlatformAccount = Sim101,
            Instrument = Mnq.Clone(),
            Kind = CommandKind.Flatten,
            Flatten = new FlattenSpec { Reason = reason },
        };

        public static CommandPayload Modify(string commandId, string target, ChildRole role, decimal? delta = null, int? quantity = null) => new CommandPayload
        {
            CommandId = commandId,
            InstructionId = "0192b3c4-0000-7000-8000-000000000006",
            AccountId = AccountId,
            PlatformAccount = Sim101,
            Instrument = Mnq.Clone(),
            Kind = CommandKind.Modify,
            Modify = new ModifySpec { Targets = { new OrderTarget { CommandId = target, ChildRole = role } }, PriceDelta = delta, Quantity = quantity },
        };

        public static CommandPayload Cancel(string commandId, string target, ChildRole role) => new CommandPayload
        {
            CommandId = commandId,
            InstructionId = "0192b3c4-0000-7000-8000-000000000006",
            AccountId = AccountId,
            PlatformAccount = Sim101,
            Instrument = Mnq.Clone(),
            Kind = CommandKind.Cancel,
            Cancel = new CancelSpec { Targets = { new OrderTarget { CommandId = target, ChildRole = role } } },
        };

        /// <summary>Damages the last journal line, as a power loss during an unconfirmed write can (store closed first).</summary>
        public void CorruptLastJournalLineOnNextRestart() => _corruptOnRestart = true;

        private bool _corruptOnRestart;

        /// <summary>Drops platform events that are queued but not yet delivered (the process died first).</summary>
        public void LoseEvents() => _events.Clear();

        private sealed class FaultInjectingStore : IDurableStore
        {
            private readonly IDurableStore _inner;
            public FaultInjectingStore(IDurableStore inner) => _inner = inner;
            public bool FailNext { get; set; }
            public bool RecoveredFromTornWrite => _inner.RecoveredFromTornWrite;
            public IReadOnlyDictionary<string, string> Collection(string name) => _inner.Collection(name);
            public string? Get(string collection, string key) => _inner.Get(collection, key);

            public void Commit(IReadOnlyList<StoreOp> ops)
            {
                if (FailNext)
                {
                    FailNext = false;
                    throw new IOException("simulated disk failure");
                }
                _inner.Commit(ops);
            }

            public void Dispose() { }
        }

        private sealed class ListLog : IAdapterLog
        {
            private readonly List<string> _lines;
            public ListLog(List<string> lines) => _lines = lines;
            public void Write(LogLevel level, string eventName, string message) => _lines.Add(level + " " + eventName + " " + message);
        }
    }
}
