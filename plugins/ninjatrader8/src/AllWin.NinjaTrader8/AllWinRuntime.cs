// All Win — NT8 host runtime (minimal version 0.1.0-min): wires the engine, the journal, the WebSocket client and the
// redacted log. Development build only: connects to the local fake Core on ws://127.0.0.1:8765 with the test token that
// start-fakecore.cmd creates. Paths per ADR-0005 N12 (%LOCALAPPDATA%\AllWin). C# 7.3 (compiled by NinjaTrader).
#region Using declarations
using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Store;
using AllWin.Adapter.Core.Transport;
using AwBuild = AllWin.Adapter.Core.Protocol.AdapterBuild;
#endregion

namespace NinjaTrader.NinjaScript.AddOns
{
    public sealed class AllWinRuntime
    {
        public const string Version = "0.1.0-min";
        public const string Endpoint = "ws://127.0.0.1:8765/adapter/v1";

        public static readonly AllWinRuntime Instance = new AllWinRuntime();

        private readonly object gate = new object();
        private AllWinLog log;
        private AllWinPlatform platform;
        private IDurableStore store;
        private EngineLoop loop;
        private WebSocketConnector connector;
        private string status = "Parado / Stopped";

        public static string Root { get { return Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "AllWin"); } }
        public static string LogDir { get { return Path.Combine(Root, "logs"); } }
        public static string JournalDir { get { return Path.Combine(Root, "journal"); } }
        public static string TokenFile { get { return Path.Combine(Root, "fakecore-dev.token"); } }

        public AllWinLog Log { get { return log; } }
        public string Status { get { lock (gate) return status; } }

        public void Start()
        {
            lock (gate)
            {
                if (loop != null) return;
                log = new AllWinLog(LogDir);
                log.Write(LogLevel.Info, "host.start", "version=" + Version + " nt=" + AllWinDiagnostics.NtVersion() + " clr=" + Environment.Version
                    + " endpoint=" + Endpoint + " build=development");
                try
                {
                    Directory.CreateDirectory(JournalDir);
                    store = new FileJournalStore(JournalDir);
                    platform = new AllWinPlatform(log);
                    var options = new EngineOptions { AdapterVersion = Version, Build = AwBuild.Development, PlatformVersion = AllWinDiagnostics.NtVersion() };
                    var engine = new AdapterEngine(options, platform, store, null, log);
                    engine.Changed += () => UpdateStatus(engine);
                    loop = new EngineLoop(engine, log);
                    platform.Attach(loop.PostPlatformEvent);
                    connector = new WebSocketConnector(new Uri(Endpoint), ReadToken, loop, AwBuild.Development, log);
                    connector.Start();
                    status = "Iniciando / Starting";
                }
                catch (Exception e)
                {
                    log.Write(LogLevel.Error, "host.start_failed", e.GetType().Name + ": " + e.Message);
                    status = "Erro ao iniciar / start failed: " + e.Message;
                    StopLocked();
                }
            }
        }

        public void Stop()
        {
            lock (gate) StopLocked();
        }

        private void StopLocked()
        {
            if (connector != null) { try { connector.Dispose(); } catch (Exception) { } connector = null; }
            if (platform != null) { try { platform.Dispose(); } catch (Exception) { } platform = null; }
            if (loop != null) { try { loop.Dispose(); } catch (Exception) { } loop = null; }
            if (store != null) { try { store.Dispose(); } catch (Exception) { } store = null; }
            if (log != null) log.Write(LogLevel.Info, "host.stop", "version=" + Version);
            status = "Parado / Stopped";
        }

        /// <summary>The test token of the local fake Core (never shown, never logged; key id only).</summary>
        private string ReadToken()
        {
            try
            {
                if (!File.Exists(TokenFile))
                {
                    log.Write(LogLevel.Warning, "host.token_missing", "start start-fakecore.cmd first (it creates the test token)");
                    return null;
                }
                return File.ReadAllText(TokenFile).Trim();
            }
            catch (Exception e)
            {
                log.Write(LogLevel.Error, "host.token_unreadable", e.GetType().Name);
                return null;
            }
        }

        /// <summary>Runs on the engine thread after any visible change.</summary>
        private void UpdateStatus(AdapterEngine engine)
        {
            var sb = new StringBuilder();
            sb.Append(SessionText(engine.Session));
            foreach (AccountView v in engine.GetAccountViews())
            {
                sb.Append(Environment.NewLine).Append(v.PlatformAccount).Append(": ").Append(v.Kind)
                    .Append(v.PlatformConnected ? ", conectada / connected" : ", desconectada / disconnected")
                    .Append(v.ExecutionAllowed ? ", execução permitida / execution allowed" : ", execução recusada / execution refused");
            }
            lock (gate) status = sb.ToString();
        }

        private static string SessionText(SessionState s)
        {
            switch (s)
            {
                case SessionState.Ready: return "Conectado / Connected";
                case SessionState.AwaitingWelcome: return "Conectando / Connecting";
                case SessionState.Stopped: return "Parado (veja o log) / Stopped (see the log)";
                default: return "Desconectado, tentando de novo / Disconnected, retrying";
            }
        }
    }

    /// <summary>Redacted structured log: %LOCALAPPDATA%\AllWin\logs\allwin-YYYYMMDD.log, key=value on one line.</summary>
    public sealed class AllWinLog : IAdapterLog
    {
        private static readonly Regex Token = new Regex("aw([dk])_([A-Za-z0-9]+)_[A-Za-z0-9_-]+", RegexOptions.Compiled);
        private readonly object gate = new object();
        private readonly string dir;
        private readonly List<string> recent = new List<string>();

        public AllWinLog(string dir)
        {
            this.dir = dir;
        }

        public void Write(LogLevel level, string eventName, string message)
        {
            string line = DateTime.UtcNow.ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'", CultureInfo.InvariantCulture) + " " + level + " " + eventName + " "
                + Token.Replace(message ?? "", "aw$1_$2_[REDACTED]");
            lock (gate)
            {
                recent.Add(line);
                if (recent.Count > 500) recent.RemoveAt(0);
                try
                {
                    Directory.CreateDirectory(dir);
                    File.AppendAllText(Path.Combine(dir, "allwin-" + DateTime.UtcNow.ToString("yyyyMMdd", CultureInfo.InvariantCulture) + ".log"), line + Environment.NewLine, Encoding.UTF8);
                }
                catch (Exception) { }
            }
        }

        public string Recent(int count)
        {
            lock (gate) return string.Join(Environment.NewLine, recent.Skip(Math.Max(0, recent.Count - count)).ToArray());
        }
    }
}
