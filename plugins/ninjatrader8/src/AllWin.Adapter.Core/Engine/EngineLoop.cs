using System;
using System.Collections.Concurrent;
using System.Threading;
using System.Threading.Tasks;
using AllWin.Adapter.Core.Platform;

namespace AllWin.Adapter.Core.Engine
{
    /// <summary>
    /// The single consumer of every engine input (ADR-0005 N6). Platform callbacks and the WebSocket
    /// task only enqueue and return; nothing here blocks a caller's thread. UI code reads state with
    /// <see cref="InvokeAsync{T}"/> and must marshal <see cref="AdapterEngine.Changed"/> to its dispatcher
    /// with Dispatcher.InvokeAsync (never Invoke).
    /// </summary>
    public sealed class EngineLoop : IDisposable
    {
        private readonly BlockingCollection<Action> _queue = new BlockingCollection<Action>();
        private readonly AdapterEngine _engine;
        private readonly IAdapterLog _log;
        private readonly Thread _thread;
        private readonly Timer _timer;
        private int _disposed;

        public EngineLoop(AdapterEngine engine, IAdapterLog? log = null, TimeSpan? tick = null)
        {
            _engine = engine;
            _log = log ?? NullLog.Instance;
            _thread = new Thread(Work) { IsBackground = true, Name = "AllWin.Engine" };
            _thread.Start();
            Post(e => e.Start());
            var period = tick ?? TimeSpan.FromMilliseconds(250);
            _timer = new Timer(_ => Post(e => e.OnTimer()), null, period, period);
        }

        public void Post(Action<AdapterEngine> work)
        {
            if (Volatile.Read(ref _disposed) != 0) return;
            try { _queue.Add(() => work(_engine)); }
            catch (InvalidOperationException) { } // completed during shutdown
        }

        /// <summary>Copy the event out of the platform callback and return immediately.</summary>
        public void PostPlatformEvent(PlatformEvent platformEvent) => Post(e => e.OnPlatformEvent(platformEvent));

        public Task<T> InvokeAsync<T>(Func<AdapterEngine, T> query)
        {
            var tcs = new TaskCompletionSource<T>(TaskCreationOptions.RunContinuationsAsynchronously);
            Post(e =>
            {
                try { tcs.SetResult(query(e)); }
                catch (Exception ex) { tcs.SetException(ex); }
            });
            return tcs.Task;
        }

        private void Work()
        {
            foreach (var work in _queue.GetConsumingEnumerable())
            {
                try { work(); }
                catch (Exception e) { _log.Write(LogLevel.Error, "loop.error", e.GetType().Name + ": " + e.Message); }
            }
        }

        public void Dispose()
        {
            if (Interlocked.Exchange(ref _disposed, 1) != 0) return;
            _timer.Dispose();
            _queue.CompleteAdding();
            if (Thread.CurrentThread != _thread) _thread.Join(TimeSpan.FromSeconds(5));
            _queue.Dispose();
        }
    }
}
