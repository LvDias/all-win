using System;
using System.IO;
using Xunit;

namespace AllWin.Adapter.Core.Tests.Harness
{
    public sealed class TokenFileTests : IDisposable
    {
        private readonly string _path = Path.Combine(Path.GetTempPath(), "allwin-token-" + Guid.NewGuid().ToString("N"));

        public void Dispose()
        {
            if (File.Exists(_path)) File.Delete(_path);
        }

        [Fact]
        public void Fake_core_token_file_is_created_owner_only()
        {
            if (OperatingSystem.IsWindows()) return; // ACL-based on Windows
            var token = Cli.ReadOrCreateToken(_path);
            Assert.StartsWith("awd_", token);
            Assert.Equal(UnixFileMode.UserRead | UnixFileMode.UserWrite, File.GetUnixFileMode(_path));
        }

        [Fact]
        public void An_existing_token_file_with_wider_permissions_is_tightened()
        {
            if (OperatingSystem.IsWindows()) return;
            File.WriteAllText(_path, "awd_AbCdEfGhIjKl_" + new string('x', 43));
            File.SetUnixFileMode(_path, UnixFileMode.UserRead | UnixFileMode.UserWrite | UnixFileMode.GroupRead | UnixFileMode.OtherRead);
            Cli.ReadOrCreateToken(_path);
            Assert.Equal(UnixFileMode.UserRead | UnixFileMode.UserWrite, File.GetUnixFileMode(_path));
        }
    }
}
