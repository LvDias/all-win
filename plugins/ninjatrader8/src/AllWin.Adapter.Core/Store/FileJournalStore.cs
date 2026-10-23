using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace AllWin.Adapter.Core.Store
{
    /// <summary>
    /// Durable key/value collections backed by an append-only journal plus a compacted snapshot.
    /// <para>
    /// Every <see cref="Commit"/> is one CRC-protected journal line flushed to disk before it returns,
    /// so a batch is all-or-nothing across crashes. A torn last line (crash mid-write) is dropped on
    /// open (no trailing newline, or a bad CRC on the last line; see <see cref="DroppedTornTail"/>); damage before
    /// the last line is reported as <see cref="StoreCorruptException"/> and the adapter
    /// fails closed. The directory is locked so two processes cannot share one store.
    /// </para>
    /// </summary>
    public sealed class FileJournalStore : IDurableStore
    {
        public const string JournalFile = "journal.log";
        public const string SnapshotFile = "state.snap";
        private const string LockFile = "store.lock";
        private const int FormatVersion = 1;

        private readonly string _directory;
        private readonly long _compactThresholdBytes;
        private readonly Dictionary<string, Dictionary<string, string>> _data = new Dictionary<string, Dictionary<string, string>>(StringComparer.Ordinal);
        private readonly FileStream _lock;
        private FileStream _journal;
        private long _seq;
        private bool _disposed;

        public FileJournalStore(string directory, long compactThresholdBytes = 4 * 1024 * 1024)
        {
            _directory = directory;
            _compactThresholdBytes = compactThresholdBytes;
            Directory.CreateDirectory(directory);
            try
            {
                _lock = new FileStream(Path.Combine(directory, LockFile), FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None);
            }
            catch (IOException e)
            {
                throw new StoreLockedException("Store directory is in use by another process.", e);
            }

            try
            {
                LoadSnapshot();
                var validLength = ReplayJournal();
                _journal = OpenJournal(validLength);
                if (_journal.Length > _compactThresholdBytes) Compact();
            }
            catch
            {
                _lock.Dispose();
                throw;
            }
        }

        /// <summary>The last journal line was an unconfirmed (torn) write and was dropped on open.</summary>
        public bool DroppedTornTail { get; private set; }

        public bool RecoveredFromTornWrite => DroppedTornTail;

        public IReadOnlyDictionary<string, string> Collection(string name) =>
            _data.TryGetValue(name, out var c) ? c : (IReadOnlyDictionary<string, string>)EmptyCollection;

        public string? Get(string collection, string key) =>
            _data.TryGetValue(collection, out var c) && c.TryGetValue(key, out var v) ? v : null;

        public void Commit(IReadOnlyList<StoreOp> ops)
        {
            if (_disposed) throw new ObjectDisposedException(nameof(FileJournalStore));
            if (ops.Count == 0) return;
            var seq = _seq + 1;
            var line = EncodeLine(seq, ops);
            var bytes = Encoding.UTF8.GetBytes(line);
            _journal.Write(bytes, 0, bytes.Length);
            _journal.Flush(true);
            _seq = seq;
            Apply(ops);
            if (_journal.Length > _compactThresholdBytes) Compact();
        }

        /// <summary>Writes the full state to a new snapshot (atomic rename) and empties the journal.</summary>
        public void Compact()
        {
            var tmp = Path.Combine(_directory, SnapshotFile + ".tmp");
            var target = Path.Combine(_directory, SnapshotFile);
            using (var fs = new FileStream(tmp, FileMode.Create, FileAccess.Write, FileShare.None))
            {
                var all = _data.SelectMany(c => c.Value.Select(kv => new StoreOp(c.Key, kv.Key, kv.Value))).ToList();
                var bytes = Encoding.UTF8.GetBytes(EncodeLine(_seq, all));
                fs.Write(bytes, 0, bytes.Length);
                fs.Flush(true);
            }
            if (File.Exists(target)) File.Replace(tmp, target, null);
            else File.Move(tmp, target);

            // Crash before this truncation is safe: replay skips journal lines with seq <= snapshot seq.
            _journal.SetLength(0);
            _journal.Flush(true);
        }

        public void Dispose()
        {
            if (_disposed) return;
            _disposed = true;
            _journal?.Dispose();
            _lock.Dispose();
        }

        private void LoadSnapshot()
        {
            var path = Path.Combine(_directory, SnapshotFile);
            if (!File.Exists(path)) return;
            var text = File.ReadAllText(path, Encoding.UTF8);
            if (!TryDecodeLine(text.TrimEnd('\n'), out var seq, out var ops))
                throw new StoreCorruptException("Snapshot file is damaged: " + path);
            Apply(ops);
            _seq = seq;
        }

        private long ReplayJournal()
        {
            var path = Path.Combine(_directory, JournalFile);
            if (!File.Exists(path)) return 0;
            var bytes = File.ReadAllBytes(path);
            long offset = 0;
            while (offset < bytes.Length)
            {
                var newline = Array.IndexOf(bytes, (byte)'\n', (int)offset);
                // Torn tail (Architect, L3 option 2): the LAST line is an unconfirmed write if it has no newline
                // or fails its CRC (after a power loss its sectors may land out of order). It is dropped and
                // reported; nothing was acted on, because the store is write-ahead. Damage elsewhere is fatal.
                if (newline < 0)
                {
                    DroppedTornTail = true;
                    break;
                }
                var line = Encoding.UTF8.GetString(bytes, (int)offset, (int)(newline - offset));
                if (!TryDecodeLine(line, out var seq, out var ops))
                {
                    if (newline == bytes.Length - 1)
                    {
                        DroppedTornTail = true;
                        break;
                    }
                    throw new StoreCorruptException("Journal is damaged before its last line: " + path);
                }
                if (seq > _seq)
                {
                    if (seq != _seq + 1) throw new StoreCorruptException("Journal sequence gap in " + path);
                    Apply(ops);
                    _seq = seq;
                }
                offset = newline + 1;
            }
            return offset;
        }

        private FileStream OpenJournal(long validLength)
        {
            var fs = new FileStream(Path.Combine(_directory, JournalFile), FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.Read,
                4096, FileOptions.WriteThrough);
            if (fs.Length != validLength)
            {
                fs.SetLength(validLength);
                fs.Flush(true);
            }
            fs.Seek(0, SeekOrigin.End);
            return fs;
        }

        private void Apply(IEnumerable<StoreOp> ops)
        {
            foreach (var op in ops)
            {
                if (!_data.TryGetValue(op.Collection, out var c))
                {
                    c = new Dictionary<string, string>(StringComparer.Ordinal);
                    _data[op.Collection] = c;
                }
                if (op.Value == null) c.Remove(op.Key);
                else c[op.Key] = op.Value;
            }
        }

        private static string EncodeLine(long seq, IReadOnlyList<StoreOp> ops)
        {
            var array = new JArray();
            foreach (var op in ops)
                array.Add(new JObject { ["c"] = op.Collection, ["k"] = op.Key, ["v"] = op.Value == null ? JValue.CreateNull() : new JValue(op.Value) });
            var body = new JObject { ["f"] = FormatVersion, ["s"] = seq, ["ops"] = array }.ToString(Formatting.None);
            return Crc32.Compute(Encoding.UTF8.GetBytes(body)).ToString("x8", CultureInfo.InvariantCulture) + " " + body + "\n";
        }

        private static bool TryDecodeLine(string line, out long seq, out List<StoreOp> ops)
        {
            seq = 0;
            ops = new List<StoreOp>();
            if (line.Length < 10 || line[8] != ' ') return false;
            var body = line.Substring(9);
            if (!uint.TryParse(line.Substring(0, 8), NumberStyles.HexNumber, CultureInfo.InvariantCulture, out var crc)) return false;
            if (Crc32.Compute(Encoding.UTF8.GetBytes(body)) != crc) return false;
            JObject obj;
            // Strict parser: values stay strings (a default JObject.Parse would turn timestamps into DateTime).
            try { obj = Protocol.ProtocolJson.ParseObject(body); }
            catch (JsonException) { return false; }
            if ((int?)obj["f"] != FormatVersion) throw new StoreCorruptException("Unsupported store format.");
            seq = (long)obj["s"]!;
            foreach (var item in (JArray)obj["ops"]!)
                ops.Add(new StoreOp((string)item["c"]!, (string)item["k"]!, item["v"]!.Type == JTokenType.Null ? null : (string)item["v"]!));
            return true;
        }

        private static readonly Dictionary<string, string> EmptyCollection = new Dictionary<string, string>();
    }

    public sealed class StoreCorruptException : Exception
    {
        public StoreCorruptException(string message) : base(message) { }
    }

    public sealed class StoreLockedException : Exception
    {
        public StoreLockedException(string message, Exception inner) : base(message, inner) { }
    }

    internal static class Crc32
    {
        private static readonly uint[] Table = BuildTable();

        public static uint Compute(byte[] data)
        {
            var crc = 0xFFFFFFFFu;
            foreach (var b in data) crc = Table[(crc ^ b) & 0xFF] ^ (crc >> 8);
            return ~crc;
        }

        private static uint[] BuildTable()
        {
            var table = new uint[256];
            for (uint i = 0; i < 256; i++)
            {
                var c = i;
                for (var k = 0; k < 8; k++) c = (c & 1) != 0 ? 0xEDB88320u ^ (c >> 1) : c >> 1;
                table[i] = c;
            }
            return table;
        }
    }
}
