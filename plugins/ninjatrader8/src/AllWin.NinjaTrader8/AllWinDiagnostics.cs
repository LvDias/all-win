// All Win — NT8 host diagnostics: raw platform event capture for the spike and the "Export diagnostics" allowlist
// (ADR-0005 N12). Compiled by NinjaTrader on import (C# 7.3). Never reads or copies the token or consent files.
#region Using declarations
using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using NinjaTrader.Cbi;
#endregion

namespace NinjaTrader.NinjaScript.AddOns
{
    /// <summary>Raw event capture for the spike (E4 sequence, K15 executions after a restart, E14 commission) and the diagnostics export.</summary>
    public sealed class AllWinDiagnostics
    {
        public static readonly AllWinDiagnostics Instance = new AllWinDiagnostics();

        private const string ProbeAccount = "Sim101";

        private readonly object gate = new object();
        private readonly HashSet<Account> subscribed = new HashSet<Account>();
        private readonly List<string> events = new List<string>();
        private bool started;

        public static string UserDir { get { return Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.MyDocuments), "NinjaTrader 8"); } }
        /// <summary>ADR-0005 N12: %LOCALAPPDATA%\AllWin\logs (nothing of ours under Documents).</summary>
        public static string LogDir { get { return AllWinRuntime.LogDir; } }
        public static string LogFile { get { return Path.Combine(LogDir, "events-" + DateTime.UtcNow.ToString("yyyyMMdd", CultureInfo.InvariantCulture) + ".log"); } }

        public void Start()
        {
            lock (gate)
            {
                if (started) return;
                started = true;
            }
            Log("start", "All Win " + Version() + " raw event capture" + " | NT " + NtVersion() + " | CLR " + Environment.Version + " | OS " + Environment.OSVersion
                + " | Newtonsoft " + LoadedVersion("Newtonsoft.Json"));
            CheckCoreDll();
            SubscribeAll();
        }

        public void Stop()
        {
            lock (gate)
            {
                foreach (Account a in subscribed)
                {
                    a.OrderUpdate -= OnOrderUpdate;
                    a.ExecutionUpdate -= OnExecutionUpdate;
                    a.PositionUpdate -= OnPositionUpdate;
                }
                subscribed.Clear();
                started = false;
            }
            Log("stop", "probe terminated");
        }

        public static string Version() { return AllWinRuntime.Version; }

        // ------------------------------------------------------------------ logging

        public void Log(string kind, string text)
        {
            string line = DateTime.UtcNow.ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'", CultureInfo.InvariantCulture) + " " + kind + " " + text;
            lock (gate)
            {
                events.Add(line);
                if (events.Count > 5000) events.RemoveAt(0);
                try
                {
                    Directory.CreateDirectory(LogDir);
                    File.AppendAllText(LogFile, line + Environment.NewLine, Encoding.UTF8);
                }
                catch (Exception) { }
            }
        }

        public string Events()
        {
            lock (gate) return string.Join(Environment.NewLine, events.Where(l => l.Contains(" order ") || l.Contains(" execution ") || l.Contains(" position ") || l.Contains(" probe ")).ToArray());
        }

        // ------------------------------------------------------------------ accounts and events

        public void SubscribeAll()
        {
            List<Account> accounts;
            lock (Account.All) accounts = Account.All.ToList();
            lock (gate)
            {
                foreach (Account a in accounts)
                {
                    if (subscribed.Contains(a)) continue;
                    a.OrderUpdate += OnOrderUpdate;
                    a.ExecutionUpdate += OnExecutionUpdate;
                    a.PositionUpdate += OnPositionUpdate;
                    subscribed.Add(a);
                }
            }
        }

        public string ListAccounts()
        {
            SubscribeAll();
            List<Account> accounts;
            lock (Account.All) accounts = Account.All.ToList();
            var sb = new StringBuilder();
            foreach (Account a in accounts)
            {
                object connection = Prop(a, "Connection");
                object options = Prop(connection, "Options");
                string line = "account=" + a.Name
                    + " connection=" + Str(Prop(options, "Name"))
                    + " mode=" + Str(Prop(options, "Mode"))
                    + " provider=" + Str(Prop(options, "Provider"))
                    + " status=" + Str(Prop(connection, "Status"))
                    + " price_status=" + Str(Prop(connection, "PriceStatus"))
                    + " account_provider=" + Str(Prop(a, "Provider"))
                    + " cash_value=" + AccountValue(a, "CashValue")
                    + " net_liquidation=" + AccountValue(a, "NetLiquidation");
                sb.AppendLine(line);
                Log("account", line);
            }
            return sb.ToString();
        }

        public string ListExecutions()
        {
            Account a = Find(ProbeAccount);
            if (a == null) return "Sim101 not found";
            var sb = new StringBuilder();
            int count = 0;
            lock (a.Executions)
            {
                foreach (object x in a.Executions)
                {
                    count++;
                    string line = Dump(x);
                    sb.AppendLine(line);
                    Log("execution_list", line);
                }
            }
            string head = "Sim101 executions after start: " + count + " (K15)";
            Log("execution_list", head);
            lock (a.Orders)
            {
                foreach (object o in a.Orders) Log("order_list", Dump(o));
            }
            return head + Environment.NewLine + sb;
        }

        private void OnOrderUpdate(object sender, OrderEventArgs e)
        {
            Log("order", "account=" + AccountName(sender) + " " + Dump(e) + " | order: " + Dump(e.Order));
        }

        private void OnExecutionUpdate(object sender, ExecutionEventArgs e)
        {
            Log("execution", "account=" + AccountName(sender) + " " + Dump(e) + " | execution: " + Dump(e.Execution));
        }

        private void OnPositionUpdate(object sender, PositionEventArgs e)
        {
            Log("position", "account=" + AccountName(sender) + " " + Dump(e) + " | position: " + Dump(e.Position));
        }

        private static Account Find(string name)
        {
            lock (Account.All) return Account.All.FirstOrDefault(a => a.Name == name);
        }

        // ------------------------------------------------------------------ Core DLL check (E3) and diagnostics

        /// <summary>Is AllWin.Adapter.Core.dll in bin\Custom, referenced in Config.xml, and does it run with NT's Newtonsoft?</summary>
        public void CheckCoreDll()
        {
            string custom = Path.Combine(UserDir, "bin", "Custom");
            string dll = Path.Combine(custom, "AllWin.Adapter.Core.dll");
            Log("core_dll", "file " + dll + " exists=" + File.Exists(dll));
            try
            {
                string config = Path.Combine(UserDir, "Config.xml");
                if (File.Exists(config))
                {
                    string[] lines = File.ReadAllLines(config).Where(l => l.IndexOf("AllWin", StringComparison.OrdinalIgnoreCase) >= 0).ToArray();
                    Log("core_dll", "Config.xml lines naming AllWin: " + lines.Length + (lines.Length > 0 ? " | " + string.Join(" | ", lines.Select(l => l.Trim()).ToArray()) : ""));
                }
            }
            catch (Exception ex) { Log("core_dll", "Config.xml unreadable: " + ex.Message); }
            Log("core_dll", "already loaded by NT: " + LoadedVersion("AllWin.Adapter.Core"));
            if (!File.Exists(dll)) return;
            try
            {
                Assembly core = AppDomain.CurrentDomain.GetAssemblies().FirstOrDefault(x => x.GetName().Name == "AllWin.Adapter.Core") ?? Assembly.LoadFrom(dll);
                Type codec = core.GetType("AllWin.Adapter.Core.Protocol.ProtocolCodec", true);
                Type heartbeat = core.GetType("AllWin.Adapter.Core.Protocol.HeartbeatPayload", true);
                MethodInfo encode = codec.GetMethod("Encode", new[] { typeof(object), typeof(string), typeof(DateTime), typeof(string) });
                object frame = encode.Invoke(null, new object[] { Activator.CreateInstance(heartbeat), Guid.NewGuid().ToString(), DateTime.UtcNow, "1.2" });
                Log("core_dll", "OK " + core.GetName().Version + " encoded heartbeat: " + frame);
            }
            catch (Exception ex)
            {
                Exception inner = ex is TargetInvocationException && ex.InnerException != null ? ex.InnerException : ex;
                Log("core_dll", "FAILED " + inner.GetType().Name + ": " + inner.Message);
            }
        }

        /// <summary>
        /// Copies, by allowlist, only the files of %LOCALAPPDATA%\AllWin\logs and \journal (ADR-0005 N12) to a Desktop folder,
        /// writes inventory.txt, then zips it. Nothing else is read: not the token or consent files, not NT8's log/trace.
        /// </summary>
        public string ExportDiagnostics()
        {
            string stamp = DateTime.UtcNow.ToString("yyyyMMdd'T'HHmmss'Z'", CultureInfo.InvariantCulture);
            string name = "AllWin-diag-" + Version() + "-" + stamp;
            string desktop = Environment.GetFolderPath(Environment.SpecialFolder.DesktopDirectory);
            string dir = Path.Combine(desktop, name);
            Directory.CreateDirectory(dir);
            Log("diag", "export to " + dir);
            CopyAll(LogDir, Path.Combine(dir, "allwin-logs"), "*");
            CopyAll(AllWinRuntime.JournalDir, Path.Combine(dir, "allwin-journal"), "*");
            // Never NT8's own log/trace (N12, Esquadro): this export is a product feature used with live accounts, and those
            // files name other connections and account numbers. In test rounds they are a separate, filtered script step.
            File.WriteAllText(Path.Combine(dir, "versions.txt"), "allwin=" + Version() + Environment.NewLine + "nt=" + NtVersion() + Environment.NewLine
                + "clr=" + Environment.Version + Environment.NewLine + "os=" + Environment.OSVersion + Environment.NewLine);
            // Allowlist only (ADR-0005 N12: token and consent share %LOCALAPPDATA%\AllWin): the inventory lists
            // every collected file so QA can check that no token or consent file is in the zip.
            File.WriteAllLines(Path.Combine(dir, "inventory.txt"), Directory.GetFiles(dir, "*", SearchOption.AllDirectories)
                .Select(f => f.Substring(dir.Length + 1) + " " + new FileInfo(f).Length).ToArray());
            string zip = dir + ".zip";
            try
            {
                Type zipFile = Type.GetType("System.IO.Compression.ZipFile, System.IO.Compression.FileSystem, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089", true);
                zipFile.GetMethod("CreateFromDirectory", new[] { typeof(string), typeof(string) }).Invoke(null, new object[] { dir, zip });
                return "Diagnóstico salvo / diagnostics saved: " + zip;
            }
            catch (Exception ex)
            {
                Log("diag", "zip failed: " + ex.Message);
                return "Pasta salva (compacte com botão direito → Enviar para → Pasta compactada) / folder saved: " + dir;
            }
        }

        private static void CopyAll(string from, string to, string pattern)
        {
            if (!Directory.Exists(from)) return;
            Directory.CreateDirectory(to);
            foreach (string f in Directory.GetFiles(from, pattern))
            {
                try
                {
                    // Copy through a shared read: NT8 keeps its current log open.
                    using (var src = new FileStream(f, FileMode.Open, FileAccess.Read, FileShare.ReadWrite | FileShare.Delete))
                    using (var dst = File.Create(Path.Combine(to, Path.GetFileName(f)))) src.CopyTo(dst);
                }
                catch (Exception) { }
            }
        }

        // ------------------------------------------------------------------ reflection helpers

        private static object Prop(object o, string name)
        {
            if (o == null) return null;
            try
            {
                PropertyInfo p = o.GetType().GetProperty(name, BindingFlags.Public | BindingFlags.Instance);
                return p == null ? null : p.GetValue(o, null);
            }
            catch (Exception) { return null; }
        }

        private static string Str(object o)
        {
            if (o == null) return "?";
            return Convert.ToString(o, CultureInfo.InvariantCulture);
        }

        private static string AccountName(object sender)
        {
            Account a = sender as Account;
            return a == null ? "?" : a.Name;
        }

        private static string AccountValue(Account a, string item)
        {
            try
            {
                Type itemType = typeof(Account).Assembly.GetType("NinjaTrader.Cbi.AccountItem");
                Type currencyType = typeof(Account).Assembly.GetType("NinjaTrader.Cbi.Currency");
                if (itemType == null || currencyType == null) return "?";
                MethodInfo get = typeof(Account).GetMethod("Get", new[] { itemType, currencyType });
                if (get == null) return "?";
                object value = get.Invoke(a, new[] { Enum.Parse(itemType, item), Enum.Parse(currencyType, "UsDollar") });
                return Str(value);
            }
            catch (Exception) { return "?"; }
        }

        /// <summary>One line with every simple public property (strings, numbers, enums, times, and the Name of nested objects).</summary>
        private static string Dump(object o)
        {
            if (o == null) return "null";
            var parts = new List<string>();
            foreach (PropertyInfo p in o.GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance))
            {
                if (p.GetIndexParameters().Length > 0) continue;
                object v;
                try { v = p.GetValue(o, null); }
                catch (Exception) { continue; }
                if (v == null) continue;
                Type t = v.GetType();
                if (t.IsPrimitive || t.IsEnum || v is string || v is decimal || v is DateTime || v is TimeSpan)
                    parts.Add(p.Name + "=" + Str(v));
                else if (p.Name == "Order" || p.Name == "Instrument" || p.Name == "Account" || p.Name == "Execution")
                    parts.Add(p.Name + "=" + Str(Prop(v, "Name") ?? Prop(v, "FullName") ?? v));
            }
            return o.GetType().Name + "{" + string.Join("; ", parts.ToArray()) + "}";
        }

        private static string LoadedVersion(string assemblyName)
        {
            Assembly a = AppDomain.CurrentDomain.GetAssemblies().FirstOrDefault(x => x.GetName().Name == assemblyName);
            return a == null ? "not loaded" : a.GetName().Version.ToString();
        }

        public static string NtVersion()
        {
            return LoadedVersion("NinjaTrader.Core");
        }
    }
}
