using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Protocol;
using AllWin.FakeCore;
using Newtonsoft.Json.Linq;

// Scripted fake Core for adapter protocol v1. See plugins/ninjatrader8/tools/README.md.
//   dotnet run --project plugins/ninjatrader8/tools/AllWin.FakeCore -- -with-sim                 (all scenarios, Linux)
//   dotnet run --project plugins/ninjatrader8/tools/AllWin.FakeCore -- -scenario <file> -port 8765 -token-file <path>   (NT8 on Windows)
var options = Cli.Parse(args);
var files = options.Scenarios.Count > 0 ? options.Scenarios : Directory.GetFiles(Path.Combine(AppContext.BaseDirectory, "scenarios"), "*.json").OrderBy(f => f).ToList();
using var transcript = options.Transcript != null ? new StreamWriter(options.Transcript, append: false) : null;
void Trace(string line)
{
    var stamped = DateTime.UtcNow.ToString("HH:mm:ss.fff") + " " + line;
    lock (Console.Out)
    {
        transcript?.WriteLine(stamped);
        if (options.Verbose) Console.WriteLine(stamped);
    }
}

var token = options.WithSim ? Cli.NewTestToken() : Cli.ReadOrCreateToken(options.TokenFile);
var results = new List<ScenarioResult>();
foreach (var file in files)
{
    var scenario = ProtocolJson.ParseObject(File.ReadAllText(file));
    var requiresSim = (bool?)scenario["requires_sim"] ?? false;
    if (requiresSim && !options.WithSim)
    {
        Console.WriteLine("skip " + scenario["name"] + " (needs -with-sim)");
        continue;
    }
    if (((bool?)scenario["nt8_only"] ?? false) && options.WithSim)
    {
        Console.WriteLine("skip " + scenario["name"] + " (NT8 only: depends on the NT8 simulator)");
        continue;
    }
    var accounts = ScenarioRunner.AccountsOf(scenario);
    await using var core = new FakeCoreServer(accounts, token, Trace);
    await core.StartAsync(options.WithSim ? 0 : options.Port);
    Trace("scenario " + scenario["name"] + " on " + core.Endpoint);
    using var sim = options.WithSim ? new SimAdapterHost(scenario, core.Endpoint!, token, Trace) : null;
    sim?.Start();
    if (!options.WithSim) Console.WriteLine("waiting for the NT8 plugin on " + core.Endpoint + " (scenario " + scenario["name"] + ")");
    var result = await new ScenarioRunner(scenario, core, sim, Trace).RunAsync();
    results.Add(result);
    Console.WriteLine((result.Passed ? "PASS " : "FAIL ") + result.Name + (result.Failure != null ? " — " + result.Failure : ""));
    if (!result.Passed || options.Verbose) foreach (var step in result.Steps) Console.WriteLine("    " + step);
}
Console.WriteLine(results.Count(r => r.Passed) + "/" + results.Count + " scenarios passed");
return results.All(r => r.Passed) ? 0 : 1;

internal sealed class Cli
{
    public List<string> Scenarios { get; } = new List<string>();
    public bool WithSim { get; private set; }
    public bool Verbose { get; private set; }
    public int Port { get; private set; } = 8765;
    public string? TokenFile { get; private set; }
    public string? Transcript { get; private set; }

    public static Cli Parse(string[] args)
    {
        var cli = new Cli();
        for (var i = 0; i < args.Length; i++)
        {
            switch (args[i])
            {
                case "-scenario": cli.Scenarios.Add(args[++i]); break;
                case "-with-sim": cli.WithSim = true; break;
                case "-v": cli.Verbose = true; break;
                case "-port": cli.Port = int.Parse(args[++i]); break;
                case "-token-file": cli.TokenFile = args[++i]; break;
                case "-transcript": cli.Transcript = args[++i]; break;
                default: throw new ArgumentException("unknown argument " + args[i]);
            }
        }
        return cli;
    }

    /// <summary>
    /// Creates the token file readable by its owner only (mode 0600 on Unix, QA Security L2). On Windows
    /// the file inherits the user profile's ACL, which is already private to the user.
    /// </summary>
    internal static void WriteOwnerOnly(string path, string content)
    {
        var options = new FileStreamOptions { Mode = FileMode.CreateNew, Access = FileAccess.Write, Share = FileShare.None };
        if (!OperatingSystem.IsWindows()) options.UnixCreateMode = UnixFileMode.UserRead | UnixFileMode.UserWrite;
        using var stream = new FileStream(path, options);
        var bytes = Encoding.UTF8.GetBytes(content);
        stream.Write(bytes, 0, bytes.Length);
    }

    /// <summary>A throwaway device token for the local harness only (format awd_&lt;key_id&gt;_&lt;secret&gt;).</summary>
    public static string NewTestToken()
    {
        const string alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        var key = new StringBuilder();
        foreach (var b in RandomNumberGenerator.GetBytes(12)) key.Append(alphabet[b % alphabet.Length]);
        var secret = Convert.ToBase64String(RandomNumberGenerator.GetBytes(32)).TrimEnd('=').Replace('+', '-').Replace('/', '_');
        return "awd_" + key + "_" + secret;
    }

    /// <summary>
    /// Standalone mode: the plugin is configured with the same test token. It is kept in a file (never
    /// printed); only the key_id is shown.
    /// </summary>
    public static string ReadOrCreateToken(string? path)
    {
        path ??= Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".allwin-fakecore-token");
        if (!File.Exists(path)) WriteOwnerOnly(path, NewTestToken());
        else if (!OperatingSystem.IsWindows()) File.SetUnixFileMode(path, UnixFileMode.UserRead | UnixFileMode.UserWrite);
        var token = File.ReadAllText(path).Trim();
        Console.WriteLine("test token file: " + path + " (key " + AllWin.Adapter.Core.Engine.TokenMask.Mask(token) + ")");
        return token;
    }
}
