using System;
using AllWin.Adapter.Core.Protocol;

namespace AllWin.Adapter.Core.Engine
{
    /// <summary>
    /// <c>Order.Name</c> reference "aw:&lt;command_id&gt;:&lt;role&gt;" (ADR-0005 N5): 41 characters, under the
    /// NT8 50-character limit. It is the reconciliation key after a restart.
    /// </summary>
    public static class OrderName
    {
        public const string Prefix = "aw:";
        public const int MaxLength = 50;

        public static string Format(string commandId, ChildRole role) => Prefix + commandId + ":" + Letter(role);

        public static bool TryParse(string? name, out string commandId, out char roleLetter)
        {
            commandId = "";
            roleLetter = '\0';
            if (name == null || name.Length != Prefix.Length + 36 + 2 || !name.StartsWith(Prefix, StringComparison.Ordinal)) return false;
            if (name[Prefix.Length + 36] != ':') return false;
            var id = name.Substring(Prefix.Length, 36);
            if (!Guid.TryParseExact(id, "D", out _) || id != id.ToLowerInvariant()) return false;
            var letter = name[name.Length - 1];
            if (letter != 'e' && letter != 's' && letter != 't' && letter != 'x') return false;
            commandId = id;
            roleLetter = letter;
            return true;
        }

        public static char Letter(ChildRole role)
        {
            switch (role)
            {
                case ChildRole.Entry: return 'e';
                case ChildRole.Stop: return 's';
                case ChildRole.Target: return 't';
                default: return 'x';
            }
        }

        /// <summary>Role from a letter; 'x' is resolved by the owning command's kind (flatten vs reduce).</summary>
        public static ChildRole RoleOf(char letter, bool isFlattenCommand)
        {
            switch (letter)
            {
                case 'e': return ChildRole.Entry;
                case 's': return ChildRole.Stop;
                case 't': return ChildRole.Target;
                default: return isFlattenCommand ? ChildRole.Flatten : ChildRole.Exit;
            }
        }
    }
}
