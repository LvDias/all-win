using System;
using System.IO;
using System.Text;
using AllWin.Adapter.Core.Store;
using Xunit;

namespace AllWin.Adapter.Core.Tests.Store
{
    public sealed class FileJournalStoreTests : IDisposable
    {
        private readonly string _dir = Path.Combine(Path.GetTempPath(), "allwin-store-" + Guid.NewGuid().ToString("N"));

        public void Dispose()
        {
            if (Directory.Exists(_dir)) Directory.Delete(_dir, true);
        }

        [Fact]
        public void Committed_batches_survive_reopen()
        {
            using (var s = new FileJournalStore(_dir))
            {
                s.Commit(new[] { new StoreOp("a", "1", "x"), new StoreOp("b", "2", "y") });
                s.Commit(new[] { new StoreOp("a", "1", null), new StoreOp("a", "3", "z") });
            }
            using (var s = new FileJournalStore(_dir))
            {
                Assert.Null(s.Get("a", "1"));
                Assert.Equal("z", s.Get("a", "3"));
                Assert.Equal("y", s.Get("b", "2"));
            }
        }

        [Fact]
        public void Torn_last_line_is_dropped_and_store_stays_writable()
        {
            using (var s = new FileJournalStore(_dir)) s.Commit(new[] { new StoreOp("a", "1", "kept") });
            var journal = Path.Combine(_dir, FileJournalStore.JournalFile);
            File.AppendAllText(journal, "deadbeef {\"f\":1,\"s\":2,\"ops\":[{\"c\":\"a\",\"k\":\"2\",\"v\":\"lo");
            using (var s = new FileJournalStore(_dir))
            {
                Assert.Equal("kept", s.Get("a", "1"));
                Assert.Null(s.Get("a", "2"));
                s.Commit(new[] { new StoreOp("a", "2", "after") });
            }
            using (var s = new FileJournalStore(_dir)) Assert.Equal("after", s.Get("a", "2"));
        }

        [Fact]
        public void Corruption_before_the_end_fails_closed()
        {
            using (var s = new FileJournalStore(_dir))
            {
                s.Commit(new[] { new StoreOp("a", "1", "one") });
                s.Commit(new[] { new StoreOp("a", "2", "two") });
            }
            var journal = Path.Combine(_dir, FileJournalStore.JournalFile);
            var bytes = File.ReadAllBytes(journal);
            var i = Encoding.UTF8.GetString(bytes).IndexOf("one", StringComparison.Ordinal);
            bytes[i] = (byte)'X';
            File.WriteAllBytes(journal, bytes);
            Assert.Throws<StoreCorruptException>(() => new FileJournalStore(_dir));
        }

        [Fact]
        public void Compaction_keeps_state_and_tolerates_crash_before_truncate()
        {
            using (var s = new FileJournalStore(_dir, compactThresholdBytes: 512))
            {
                for (var i = 0; i < 50; i++) s.Commit(new[] { new StoreOp("c", "k" + (i % 5), "v" + i) });
            }
            Assert.True(File.Exists(Path.Combine(_dir, FileJournalStore.SnapshotFile)));
            Assert.True(new FileInfo(Path.Combine(_dir, FileJournalStore.JournalFile)).Length < 512);

            // Simulate a crash after the snapshot rename but before the journal was emptied.
            string journalCopy;
            using (var s = new FileJournalStore(_dir, compactThresholdBytes: 1 << 20))
            {
                s.Commit(new[] { new StoreOp("c", "extra", "1") });
                journalCopy = File.ReadAllText(Path.Combine(_dir, FileJournalStore.JournalFile));
                s.Compact();
            }
            File.WriteAllText(Path.Combine(_dir, FileJournalStore.JournalFile), journalCopy);
            using (var s = new FileJournalStore(_dir))
            {
                Assert.Equal("1", s.Get("c", "extra"));
                Assert.Equal("v49", s.Get("c", "k4"));
                Assert.Equal(6, s.Collection("c").Count);
            }
        }

        [Fact]
        public void Second_process_cannot_open_the_same_store()
        {
            using var s = new FileJournalStore(_dir);
            Assert.Throws<StoreLockedException>(() => new FileJournalStore(_dir));
        }
    }
}

namespace AllWin.Adapter.Core.Tests.Store
{
    public sealed class FileJournalStoreValueTests : System.IDisposable
    {
        private readonly string _dir = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "allwin-store-" + System.Guid.NewGuid().ToString("N"));

        public void Dispose()
        {
            if (System.IO.Directory.Exists(_dir)) System.IO.Directory.Delete(_dir, true);
        }

        [Xunit.Theory]
        [Xunit.InlineData("2026-10-22T14:31:00.000Z")]
        [Xunit.InlineData("21034.50")]
        [Xunit.InlineData("{\"a\":\"2026-10-22T14:31:00Z\"}")]
        public void Values_are_returned_byte_for_byte(string value)
        {
            using (var s = new AllWin.Adapter.Core.Store.FileJournalStore(_dir)) s.Commit(new[] { new AllWin.Adapter.Core.Store.StoreOp("c", "k", value) });
            using (var s = new AllWin.Adapter.Core.Store.FileJournalStore(_dir)) Xunit.Assert.Equal(value, s.Get("c", "k"));
        }
    }
}
