using System;
using System.Globalization;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Platform
{
    /// <summary>NT8 futures instrument full name ("MNQ 12-26") ↔ contract instrument (MNQ, 2026-12).</summary>
    public static class NtInstrumentName
    {
        public static string Format(Instrument instrument)
        {
            var year = int.Parse(instrument.ContractMonth.Substring(0, 4), CultureInfo.InvariantCulture);
            var month = instrument.ContractMonth.Substring(5, 2);
            return instrument.Symbol + " " + month + "-" + (year % 100).ToString("D2", CultureInfo.InvariantCulture);
        }

        public static bool TryParse(string? fullName, out Instrument instrument)
        {
            instrument = new Instrument();
            if (string.IsNullOrEmpty(fullName)) return false;
            var space = fullName!.LastIndexOf(' ');
            if (space <= 0 || fullName.Length - space - 1 != 5 || fullName[space + 3] != '-') return false;
            var symbol = fullName.Substring(0, space);
            if (symbol.Length > 10) return false;
            foreach (var c in symbol) if (!(c >= 'A' && c <= 'Z') && !(c >= '0' && c <= '9')) return false;
            if (!int.TryParse(fullName.Substring(space + 1, 2), NumberStyles.None, CultureInfo.InvariantCulture, out var month) || month < 1 || month > 12) return false;
            if (!int.TryParse(fullName.Substring(space + 4, 2), NumberStyles.None, CultureInfo.InvariantCulture, out var yy)) return false;
            instrument = new Instrument(symbol, (2000 + yy).ToString(CultureInfo.InvariantCulture) + "-" + month.ToString("D2", CultureInfo.InvariantCulture));
            return true;
        }
    }
}
