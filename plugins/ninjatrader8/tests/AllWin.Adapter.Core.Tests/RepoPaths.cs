using System;
using System.IO;

namespace AllWin.Adapter.Core.Tests
{
    internal static class RepoPaths
    {
        private static readonly Lazy<string> RootLazy = new Lazy<string>(() =>
        {
            var dir = new DirectoryInfo(AppContext.BaseDirectory);
            while (dir != null && !Directory.Exists(Path.Combine(dir.FullName, "contracts", "adapter-protocol")))
                dir = dir.Parent;
            return dir?.FullName ?? throw new InvalidOperationException("Repository root with contracts/ not found.");
        });

        public static string Root => RootLazy.Value;
        public static string Contracts => Path.Combine(Root, "contracts");
        public static string AdapterExamples => Path.Combine(Contracts, "adapter-protocol", "v1", "examples");
        public static string InstructionExamples => Path.Combine(Contracts, "instruction", "v1", "examples");
    }
}
