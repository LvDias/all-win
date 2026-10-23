using System;

namespace AllWin.Adapter.Core.Transport
{
    /// <summary>Reconnect delay: starts at 1 s, factor 2, capped at 30 s, ±20 % jitter (ADR-0004 §4).</summary>
    public sealed class Backoff
    {
        private readonly Func<double> _random;

        public Backoff(Func<double>? random = null)
        {
            var rng = new Random();
            _random = random ?? (() => { lock (rng) return rng.NextDouble(); });
        }

        public TimeSpan Delay(int attempt)
        {
            var baseSeconds = Math.Min(30.0, Math.Pow(2, Math.Min(attempt, 10)));
            var jitter = 1.0 + (_random() * 0.4 - 0.2);
            return TimeSpan.FromMilliseconds(baseSeconds * 1000.0 * jitter);
        }
    }
}
