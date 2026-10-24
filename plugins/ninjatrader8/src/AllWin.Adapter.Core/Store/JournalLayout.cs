using System;
using System.IO;
using System.Text;
using AllWin.Adapter.Core.Transport;

namespace AllWin.Adapter.Core.Store
{
    /// <summary>
    /// One journal directory per Core origin (Pêndulo M1 finding): the engine keys limits, versions, outbox and commands by
    /// account only, so a journal written against one Core must never be opened against another. Limit versions of the local
    /// fake Core (Unix ms x10) would otherwise make the real Core's limits look older and be ignored.
    /// Layout: &lt;root&gt;\fakecore, or &lt;root&gt;\core-&lt;first 12 hex of SHA-256 of the normalized endpoint&gt;.
    /// </summary>
    public static class JournalLayout
    {
        public const string FakeCoreDir = "fakecore";

        public static string DirName(Uri endpoint, bool fakeCore)
        {
            if (endpoint == null) throw new ArgumentNullException(nameof(endpoint));
            if (fakeCore) return FakeCoreDir;
            return "core-" + CertificatePin.Sha256Hex(Encoding.UTF8.GetBytes(Normalize(endpoint))).Substring(0, 12);
        }

        public static string DirFor(string root, Uri endpoint, bool fakeCore) => Path.Combine(root, DirName(endpoint, fakeCore));

        /// <summary>Scheme, host, explicit or default port and path, lower case; query, fragment and user info never count.</summary>
        public static string Normalize(Uri endpoint)
        {
            var path = endpoint.AbsolutePath.TrimEnd('/');
            return (endpoint.Scheme + "://" + endpoint.IdnHost + ":" + endpoint.Port + path).ToLowerInvariant();
        }
    }
}
