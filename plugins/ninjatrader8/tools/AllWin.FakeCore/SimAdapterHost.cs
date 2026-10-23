using System;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;
using AllWin.Adapter.Core.Store;
using AllWin.Adapter.Core.Transport;
using AllWin.Adapter.Sim;
using Newtonsoft.Json.Linq;

namespace AllWin.FakeCore
{
    /// <summary>
    /// The real adapter stack (engine, durable store, engine loop, WebSocket connector) against the
    /// simulated platform, in-process. It stands in for the NT8 AddOn host on Linux.
    /// </summary>
    public sealed class SimAdapterHost : IDisposable
    {
        private readonly Uri _endpoint;
        private readonly string _token;
        private readonly Action<string> _trace;
        private readonly string _storeDir = Path.Combine(Path.GetTempPath(), "allwin-fakecore-" + Guid.NewGuid().ToString("N"));
        private FileJournalStore? _store;
        private EngineLoop? _loop;
        private WebSocketConnector? _connector;

        public SimAdapterHost(JObject scenario, Uri endpoint, string token, Action<string> trace)
        {
            _endpoint = endpoint;
            _token = token;
            _trace = trace;
            Platform = new SimulatedPlatform();
            foreach (var account in ScenarioRunner.AccountsOf(scenario)) Platform.AddAccount(account.PlatformAccount);
            var sim = scenario["sim"] as JObject ?? new JObject();
            foreach (var tick in (sim["tick_size"] as JObject ?? new JObject { ["MNQ"] = "0.25" }).Properties())
                Platform.SetTickSize(tick.Name, DecimalText.Parse((string)tick.Value!));
            foreach (var price in (sim["price"] as JObject ?? new JObject()).Properties())
                Platform.SetPrice(price.Name, DecimalText.Parse((string)price.Value!));
        }

        public SimulatedPlatform Platform { get; }

        public void Start()
        {
            var log = new TraceLog(_trace);
            _store = new FileJournalStore(_storeDir);
            var engine = new AdapterEngine(new EngineOptions { Build = AdapterBuild.Development, PlatformVersion = "sim" }, Platform, _store, SystemClock.Instance, log);
            _loop = new EngineLoop(engine, log, TimeSpan.FromMilliseconds(100));
            Platform.EventSink = _loop.PostPlatformEvent;
            _connector = new WebSocketConnector(_endpoint, () => _token, _loop, AdapterBuild.Development, log, new Backoff(() => 0.0));
            _connector.Start();
        }

        /// <summary>Kills the adapter process: platform events from now on are lost, like a closed NT8.</summary>
        public void Stop()
        {
            Platform.EventSink = _ => { };
            _connector?.Dispose();
            _loop?.Dispose();
            _store?.Dispose();
            _connector = null;
            _loop = null;
            _store = null;
        }

        public async Task ApplyAsync(JObject step, Func<string, string> account)
        {
            foreach (var property in step.Properties().Where(p => p.Name != "do"))
            {
                var v = property.Value;
                switch (property.Name)
                {
                    case "set_price":
                        foreach (var p in ((JObject)v).Properties()) Platform.SetPrice(p.Name, DecimalText.Parse((string)p.Value!));
                        break;
                    case "oco": Platform.OcoEnforced = (bool)v; break;
                    case "reject_next": Platform.RejectNextOrder = (bool)v; break;
                    case "market_fill_slices": Platform.MarketFillSlices = (int)v; break;
                    case "connected":
                        foreach (var p in ((JObject)v).Properties()) Platform.SetConnected(account(p.Name), (bool)p.Value);
                        break;
                    case "reset": Platform.ResetAccount(account((string)v!)); break;
                    case "external_fill":
                        Platform.ExternalFill(account((string)v["account"]!), new Instrument("MNQ", "2026-12"),
                            (string)v["side"]! == "buy" ? Side.Buy : Side.Sell, (int)v["quantity"]!, DecimalText.Parse((string)v["price"]!));
                        break;
                    case "stop_adapter":
                        if ((bool)v) Stop();
                        break;
                    case "start_adapter":
                        if ((bool)v)
                        {
                            Platform.RenewHandles(); // a new NT8 process has new order objects
                            Start();
                        }
                        break;
                    default:
                        throw new ScenarioFailure("unknown sim action " + property.Name);
                }
            }
            await Task.Delay(50).ConfigureAwait(false); // let the engine loop drain the platform events
        }

        public void Dispose()
        {
            Stop();
            try { if (Directory.Exists(_storeDir)) Directory.Delete(_storeDir, true); }
            catch (IOException) { }
        }

        private sealed class TraceLog : IAdapterLog
        {
            private readonly Action<string> _trace;
            public TraceLog(Action<string> trace) => _trace = trace;

            public void Write(LogLevel level, string eventName, string message)
            {
                if (level >= LogLevel.Info) _trace("adapter " + level.ToString().ToLower(CultureInfo.InvariantCulture) + " " + eventName + " " + message);
            }
        }
    }
}
