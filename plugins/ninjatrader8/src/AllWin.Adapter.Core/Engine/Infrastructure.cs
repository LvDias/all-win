using System;

namespace AllWin.Adapter.Core.Engine
{
    public interface IClock
    {
        DateTime UtcNow { get; }
    }

    public sealed class SystemClock : IClock
    {
        public static readonly SystemClock Instance = new SystemClock();
        public DateTime UtcNow => DateTime.UtcNow;
    }

    public enum LogLevel { Debug, Info, Warning, Error }

    /// <summary>
    /// Structured log sink. Callers pass ids and codes only: never tokens, passwords or payload bodies.
    /// Hosts write it to a redacted log file (profile §13.3).
    /// </summary>
    public interface IAdapterLog
    {
        void Write(LogLevel level, string eventName, string message);
    }

    public sealed class NullLog : IAdapterLog
    {
        public static readonly NullLog Instance = new NullLog();
        public void Write(LogLevel level, string eventName, string message) { }
    }

    public static class Ids
    {
        /// <summary>Lowercase RFC 4122 UUID as the contract requires.</summary>
        public static string New() => Guid.NewGuid().ToString("D");
    }

    /// <summary>Masks device tokens for logs: only the key_id is kept (ADR-0004 §2).</summary>
    public static class TokenMask
    {
        public static string Mask(string? token)
        {
            if (string.IsNullOrEmpty(token)) return "(none)";
            var parts = token!.Split('_');
            return parts.Length >= 3 && parts[0] == "awd" && parts[1].Length == 12 ? "awd_" + parts[1] + "_***" : "***";
        }
    }
}
