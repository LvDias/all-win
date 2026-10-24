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

    /// <summary>
    /// Device token shape (OpenAPI pairing response): awd_&lt;12 alphanumerics&gt;_&lt;43 base64url&gt;. A token file with anything else
    /// is refused before it is ever sent (Guarita I2); the same rule as pair-allwin.ps1.
    /// </summary>
    public static class DeviceTokenFormat
    {
        private static readonly System.Text.RegularExpressions.Regex Shape =
            new System.Text.RegularExpressions.Regex("^awd_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}\\z", System.Text.RegularExpressions.RegexOptions.CultureInvariant);

        public static bool IsWellFormed(string? token) => token != null && Shape.IsMatch(token);
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
