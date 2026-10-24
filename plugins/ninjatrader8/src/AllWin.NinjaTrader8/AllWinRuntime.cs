// All Win — NT8 host runtime: wires the engine, the journal, the WebSocket client and the redacted log.
// Development build only (AllWinBuild.Development). Two modes, both dev-only capabilities (spike rev. 7.2):
//   - local fake Core: ws://127.0.0.1:8765 with the test token that start-fakecore.cmd creates (default);
//   - real Core: when %LOCALAPPDATA%\AllWin\dev-core.json exists (wss:// endpoint + certificate SHA-256 pin), with the
//     device token in %LOCALAPPDATA%\AllWin\device.token. An invalid dev-core.json means no connection (never a fallback).
// Paths per ADR-0005 N12 (%LOCALAPPDATA%\AllWin). C# 7.3 (compiled by NinjaTrader).
#region Using declarations
using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Localization;
using AllWin.Adapter.Core.Store;
using AllWin.Adapter.Core.Transport;
using AwBuild = AllWin.Adapter.Core.Protocol.AdapterBuild;
using AwLocale = AllWin.Adapter.Core.Protocol.Locale;
#endregion

namespace NinjaTrader.NinjaScript.AddOns
{
    /// <summary>
    /// Build switch (spike rev. 7.2). Everything that exists only for development checks it: the fake Core autoconfig, the
    /// real-Core dev mode (dev-core.json + pin), the raw event capture and the probe buttons. The packager writes it and
    /// refuses a release package that still contains a dev capability.
    /// </summary>
    public static class AllWinBuild
    {
        public const bool Development = true;
    }

    public sealed class AllWinRuntime
    {
        public const string Version = "0.1.1-dev";
        public const string FakeCoreEndpoint = "ws://127.0.0.1:8765/adapter/v1";

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
        public static string JournalRoot { get { return Path.Combine(Root, "journal"); } }
        public static string InstanceIdFile { get { return Path.Combine(Root, "instance-id"); } }

        /// <summary>One journal per Core origin (Pêndulo M1 finding; JournalLayout): journal\fakecore or journal\core-&lt;12 hex&gt;.</summary>
        public static string JournalDirFor(Uri endpoint, bool fake)
        {
            return JournalLayout.DirFor(JournalRoot, endpoint, fake);
        }

        /// <summary>Shared with pair-allwin.ps1, so the pairing and the hello carry the same instance id.</summary>
        private static string ReadOrCreateInstanceId()
        {
            Directory.CreateDirectory(Root);
            Guid id;
            if (File.Exists(InstanceIdFile) && Guid.TryParse(File.ReadAllText(InstanceIdFile).Trim(), out id)) return id.ToString();
            string created = Guid.NewGuid().ToString();
            File.WriteAllText(InstanceIdFile, created);
            return created;
        }
        public static string FakeCoreTokenFile { get { return Path.Combine(Root, "fakecore-dev.token"); } }
        public static string DevCoreFile { get { return Path.Combine(Root, "dev-core.json"); } }
        public static string DeviceTokenFile { get { return Path.Combine(Root, "device.token"); } }

        private string tokenFile;
        private string modeText = "";

        /// <summary>Endpoint in use (no secret in it), for the window.</summary>
        public string ModeText { get { lock (gate) return modeText; } }

        public AllWinLog Log { get { return log; } }
        public string Status
        {
            get
            {
                lock (gate)
                {
                    // A pin mismatch is never "just retrying": the window says why (Pêndulo M1 item 5).
                    if (connector != null && connector.LastProblem == TransportProblem.PinMismatch)
                        return status + Environment.NewLine + AdapterTexts.Get("transport.pin_mismatch", AwLocale.PtBr)
                            + " / " + AdapterTexts.Get("transport.pin_mismatch", AwLocale.En);
                    return status;
                }
            }
        }

        public void Start()
        {
            lock (gate)
            {
                if (loop != null) return;
                log = new AllWinLog(LogDir);
                log.Write(LogLevel.Info, "host.start", "version=" + Version + " nt=" + AllWinDiagnostics.NtVersion() + " clr=" + Environment.Version
                    + " build=" + (AllWinBuild.Development ? "development" : "release"));
#pragma warning disable 0162 // AllWinBuild.Development is a compile-time switch: one branch is always dead.
                if (!AllWinBuild.Development)
                {
                    // No release transport exists yet: pairing, DPAPI token and the production endpoint come with S3.
                    status = "Build de release sem transporte configurado / release build has no transport yet";
                    log.Write(LogLevel.Error, "host.no_release_transport", "release builds are not supported by this version");
                    return;
                }
#pragma warning restore 0162
                Uri endpoint;
                string journalDir;
                WebSocketFactory connect = null;
                if (File.Exists(DevCoreFile))
                {
                    string error;
                    DevCoreConfig config = null;
                    try { config = DevCoreConfig.Parse(File.ReadAllText(DevCoreFile), out error); }
                    catch (Exception e) { error = "dev-core.json unreadable: " + e.GetType().Name; }
                    if (config == null)
                    {
                        // Fail closed: a present but invalid real-Core config never falls back to anything.
                        log.Write(LogLevel.Error, "host.dev_core_invalid", error);
                        status = "Configuração inválida, sem conexão / invalid configuration, not connecting: " + error;
                        modeText = "dev-core.json";
                        return;
                    }
                    endpoint = config.Endpoint;
                    journalDir = JournalDirFor(endpoint, false);
                    connect = AllWinPinnedSocket.Create(config.CertSha256);
                    tokenFile = DeviceTokenFile;
                    modeText = "Núcleo real / real Core: " + endpoint + " | pin sha256 " + config.CertSha256.Substring(0, 8) + "…";
                }
                else
                {
                    endpoint = new Uri(FakeCoreEndpoint);
                    journalDir = JournalDirFor(endpoint, true);
                    tokenFile = FakeCoreTokenFile;
                    modeText = "Núcleo falso local / local fake Core: " + endpoint;
                }
                log.Write(LogLevel.Info, "host.mode", modeText);
                try
                {
                    Directory.CreateDirectory(journalDir);
                    store = new FileJournalStore(journalDir);
                    log.Write(LogLevel.Info, "host.journal", journalDir);
                    platform = new AllWinPlatform(log);
                    var options = new EngineOptions { AdapterVersion = Version, Build = AwBuild.Development, PlatformVersion = AllWinDiagnostics.NtVersion(),
                        InstanceId = ReadOrCreateInstanceId() };
                    var engine = new AdapterEngine(options, platform, store, null, log);
                    engine.Changed += () => UpdateStatus(engine);
                    loop = new EngineLoop(engine, log);
                    platform.Attach(loop.PostPlatformEvent);
                    connector = new WebSocketConnector(endpoint, ReadToken, loop, AwBuild.Development, log, null, connect);
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
                string file = tokenFile;
                if (file == null || !File.Exists(file))
                {
                    log.Write(LogLevel.Warning, "host.token_missing", file == FakeCoreTokenFile
                        ? "start start-fakecore.cmd first (it creates the test token)" : "device.token missing in %LOCALAPPDATA%\\AllWin");
                    return null;
                }
                string token = File.ReadAllText(file).Trim();
                if (!DeviceTokenFormat.IsWellFormed(token))
                {
                    // Guarita I2: never send a file that is not a device token; the content is never logged.
                    log.Write(LogLevel.Error, "host.token_malformed", "the token file does not hold a device token; pair again");
                    return null;
                }
                return token;
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

        /// <summary>The one redaction for every All Win log line (Crivo L1): token-shaped strings keep their key id only.</summary>
        public static string Redact(string text)
        {
            return Token.Replace(text ?? "", "aw$1_$2_[REDACTED]");
        }
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
                + Redact(message);
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
