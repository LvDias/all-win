using System;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Engine
{
    public enum TickRounding { Nearest, Up, Down }

    /// <summary>Exact decimal tick arithmetic (no floats).</summary>
    public static class TickMath
    {
        public static decimal Round(decimal price, decimal tickSize, TickRounding mode)
        {
            if (tickSize <= 0) throw new ArgumentOutOfRangeException(nameof(tickSize));
            var ticks = price / tickSize;
            decimal whole;
            switch (mode)
            {
                case TickRounding.Up: whole = decimal.Ceiling(ticks); break;
                case TickRounding.Down: whole = decimal.Floor(ticks); break;
                default: whole = decimal.Round(ticks, 0, MidpointRounding.AwayFromZero); break;
            }
            return Normalize(whole * tickSize);
        }

        /// <summary>NT8 double price → exact decimal on the tick grid (plugin boundary rule, CLAUDE.md §2).</summary>
        public static decimal FromPlatformDouble(double price, decimal tickSize)
        {
            if (double.IsNaN(price) || double.IsInfinity(price)) throw new ArgumentOutOfRangeException(nameof(price));
            return Round((decimal)price, tickSize, TickRounding.Nearest);
        }

        public static bool IsOnTick(decimal price, decimal tickSize) => price / tickSize == decimal.Truncate(price / tickSize);

        /// <summary>
        /// Stop/target prices for a position entered on <paramref name="entrySide"/> at <paramref name="basePrice"/>.
        /// Off-tick results are rounded toward the entry, i.e. the tighter distance (Architect answer 7, Q-N1).
        /// </summary>
        public static (decimal? Stop, decimal? Target) Protective(Side entrySide, decimal basePrice, decimal tickSize, int? stopTicks, int? targetTicks)
        {
            decimal? stop = null, target = null;
            if (entrySide == Side.Buy)
            {
                if (stopTicks.HasValue) stop = Round(basePrice - stopTicks.Value * tickSize, tickSize, TickRounding.Up);
                if (targetTicks.HasValue) target = Round(basePrice + targetTicks.Value * tickSize, tickSize, TickRounding.Down);
            }
            else
            {
                if (stopTicks.HasValue) stop = Round(basePrice + stopTicks.Value * tickSize, tickSize, TickRounding.Down);
                if (targetTicks.HasValue) target = Round(basePrice - targetTicks.Value * tickSize, tickSize, TickRounding.Up);
            }
            return (stop, target);
        }

        /// <summary>Volume-weighted average, kept to at most 10 fraction digits (contract price precision).</summary>
        public static decimal Vwap(decimal notional, int quantity) => Normalize(decimal.Round(notional / quantity, 10, MidpointRounding.ToEven));

        /// <summary>Drops trailing fractional zeros so prices stay within the contract pattern.</summary>
        public static decimal Normalize(decimal value) => value / 1.0000000000000000000000000000m;
    }
}
