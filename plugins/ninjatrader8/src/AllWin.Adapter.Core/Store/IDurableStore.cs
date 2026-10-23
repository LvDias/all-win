using System;
using System.Collections.Generic;

namespace AllWin.Adapter.Core.Store
{
    /// <summary>Durable string collections. <see cref="Commit"/> is atomic and durable when it returns.</summary>
    public interface IDurableStore : IDisposable
    {
        IReadOnlyDictionary<string, string> Collection(string name);
        string? Get(string collection, string key);
        void Commit(IReadOnlyList<StoreOp> ops);

        /// <summary>An unconfirmed last write was dropped when the store was opened (shown in the plugin).</summary>
        bool RecoveredFromTornWrite { get; }
    }

    /// <summary>Upsert (<see cref="Value"/> not null) or delete (null) of one key.</summary>
    public sealed class StoreOp
    {
        public StoreOp(string collection, string key, string? value)
        {
            Collection = collection;
            Key = key;
            Value = value;
        }

        public string Collection { get; }
        public string Key { get; }
        public string? Value { get; }
    }

    /// <summary>Volatile store for unit tests that do not exercise restarts.</summary>
    public sealed class InMemoryStore : IDurableStore
    {
        private readonly Dictionary<string, Dictionary<string, string>> _data = new Dictionary<string, Dictionary<string, string>>(StringComparer.Ordinal);

        public int Commits { get; private set; }

        public bool RecoveredFromTornWrite => false;

        public IReadOnlyDictionary<string, string> Collection(string name) =>
            _data.TryGetValue(name, out var c) ? c : new Dictionary<string, string>();

        public string? Get(string collection, string key) =>
            _data.TryGetValue(collection, out var c) && c.TryGetValue(key, out var v) ? v : null;

        public void Commit(IReadOnlyList<StoreOp> ops)
        {
            foreach (var op in ops)
            {
                if (!_data.TryGetValue(op.Collection, out var c)) _data[op.Collection] = c = new Dictionary<string, string>(StringComparer.Ordinal);
                if (op.Value == null) c.Remove(op.Key);
                else c[op.Key] = op.Value;
            }
            Commits++;
        }

        public void Dispose() { }
    }
}
