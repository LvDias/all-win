using System;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;
using Xunit;
using static AllWin.Adapter.Core.Tests.Engine.EngineHarness;

namespace AllWin.Adapter.Core.Tests.Engine
{
    /// <summary>Fill corrections, restarts, ADR-0004 §6.1 bracket integrity and account discontinuities.</summary>
    public class RecoveryTests
    {
        private const string C1 = "0192b3c4-0000-7000-8000-000000000601";
        private const string C2 = "0192b3c4-0000-7000-8000-000000000602";

        private static EngineHarness Ready(bool oco = true)
        {
            var h = new EngineHarness();
            h.Platform.OcoEnforced = oco;
            h.Platform.SetPrice("MNQ", 21034.50m);
            h.Connect();
            return h;
        }

        private static string ExecutionOf(EngineHarness h, string commandId) =>
            h.Reports(commandId).First(r => r.Fill != null).Fill!.FillId;

        [Fact]
        public void Amended_fill_is_reported_with_the_original_fill_id_and_a_corrected_position()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2));
            var fillId = ExecutionOf(h, C1);
            h.Platform.AmendExecution(Sim101, fillId, 2, 21034.75m);
            h.Settle();
            var amended = h.Reports(C1).Last(r => r.Fill != null);
            Assert.Equal(fillId, amended.Fill!.FillId);
            Assert.Equal(FillOperation.Amend, amended.Fill.Operation);
            Assert.Equal(21034.75m, amended.Fill.Price);
            Assert.Equal(ReportStatus.Filled, amended.Status);
            Assert.Equal(21034.75m, amended.PositionAfter!.AveragePrice);
        }

        [Fact]
        public void Removed_fill_keeps_its_id_and_is_labelled_partially_filled_at_zero()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            var fillId = ExecutionOf(h, C1);
            h.Platform.RemoveExecution(Sim101, fillId);
            h.Settle();
            var removed = h.Reports(C1).Last(r => r.Fill != null);
            Assert.Equal(FillOperation.Remove, removed.Fill!.Operation);
            Assert.Equal(1, removed.Fill.Quantity);
            Assert.Equal(ReportStatus.PartiallyFilled, removed.Status); // answer 10
            Assert.Equal(0, removed.PositionAfter!.Quantity);
        }

        [Fact]
        public void Position_after_is_queried_when_the_host_cannot_provide_it()
        {
            using var h = Ready();
            h.Platform.IncludePositionAfter = false;
            h.Deliver(Place(C1, Side.Buy, 2));
            Assert.Equal(2, h.Reports(C1).Single(r => r.Status == ReportStatus.Filled).PositionAfter!.Quantity);
        }

        [Fact]
        public void Executions_replayed_after_a_restart_are_not_reported_again()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual()));
            Assert.Equal(1, h.Reports().Count(r => r.Fill != null));
            h.AckAll();
            h.Restart(); // the platform still holds the execution; the host feeds it again
            h.Connect();
            Assert.DoesNotContain(h.Reports(), r => r.Fill != null);
        }

        [Fact]
        public void Reports_created_while_offline_are_delivered_after_welcome_and_before_the_snapshot()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual()));
            h.AckAll();
            h.Disconnect();
            h.Platform.SetPrice("MNQ", 21024.50m); // stop fills while the Core session is down
            h.Settle();
            h.Connect();
            var frames = h.Sink.Frames.Where(f => MessageTypes.IsReliable(f.Type)).ToList();
            var snapshotIndex = frames.FindIndex(f => f.Type == MessageTypes.PositionSnapshot);
            var stopFillIndex = frames.FindIndex(f => f.Type == MessageTypes.ExecutionReport && f.PayloadAs<ExecutionReportPayload>().ChildRole == ChildRole.Stop
                && f.PayloadAs<ExecutionReportPayload>().Status == ReportStatus.Filled);
            Assert.True(stopFillIndex >= 0 && stopFillIndex < snapshotIndex);
        }

        [Fact]
        public void Command_lost_between_write_ahead_and_the_platform_is_unknown_and_never_resubmitted()
        {
            using var h = Ready();
            h.Platform.CrashNextSubmit = true; // the process dies inside Submit: intent durable, order never created
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Restart();
            h.Connect();
            Assert.Equal(ReportStatus.Unknown, h.Reports(C1).Last().Status);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal) && c.Contains(C1) && h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e") != null);
            Assert.Null(h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e"));
            h.Deliver(Place(C1, Side.Buy, 1)); // re-delivery: duplicate, still not executed
            Assert.Equal(ReportStatus.Duplicate, h.Reports(C1).Last().Status);
            Assert.Null(h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e"));
        }

        [Fact]
        public void Orphan_leg_found_at_restart_is_cancelled_and_surfaced()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual()));
            h.AckAll();
            // NT8 is down (events lost) while the stop fills; the local OCO cannot cancel the target.
            h.Restart();
            h.Platform.SetPrice("MNQ", 21024.50m);
            h.Restart();
            Assert.Equal(PlatformOrderState.Cancelled, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":t")!.State);
            h.Restart(renewHandles: false); // a local hint survives restarts until the Core's next welcome
            Assert.Contains(ReasonCodes.SyncBracketOrphanLeg, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).LocalSyncReasons);
            h.Connect();
            Assert.Contains(h.Reports(C1), r => r.ChildRole == ChildRole.Stop && r.Status == ReportStatus.Filled);
            Assert.Contains(h.Reports(C1), r => r.ChildRole == ChildRole.Target && r.Status == ReportStatus.Cancelled && r.ReasonCode == ReasonCodes.OrderOrphanCancelled);
            Assert.DoesNotContain(ReasonCodes.SyncBracketOrphanLeg, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).LocalSyncReasons);
        }

        [Fact]
        public void Double_fill_while_down_is_surfaced_and_left_to_the_core()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual()));
            h.AckAll();
            h.Disconnect();
            h.Restart(); // events of the dead process are lost from here
            h.Platform.SetPrice("MNQ", 21054.50m); // target fills
            h.Platform.SetPrice("MNQ", 21024.50m); // stop fills too: unintended short 1
            h.Restart();
            Assert.Equal(-1, h.Platform.Position(Sim101, "MNQ"));
            var view = h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101);
            Assert.Contains(ReasonCodes.SyncBracketDoubleFill, view.LocalSyncReasons);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit aw:" + C1 + ":x", StringComparison.Ordinal)); // no adapter-side flatten

            h.Connect();
            var snapshot = h.Snapshots().Last();
            Assert.Equal(-1, snapshot.Accounts.Single().Positions.Single().Position.Quantity);
            h.Deliver(Flatten(C2, FlattenReason.OrphanFill)); // Q-O1 (b) from the Core
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Reconnect_snapshot_sends_stop_price_and_limit_price_valid_against_the_current_contract()
        {
            // M1 check (Maestro 2026-10-24): adapter-sim once sent order.price for a bracket leg and the Core refused it.
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual()));
            h.AckAll();
            var stop = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!;
            var target = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":t")!;
            h.Restart();
            h.Connect();

            var raw = h.Sink.RawFrames.Last(f => (string?)Newtonsoft.Json.Linq.JObject.Parse(f)["type"] == MessageTypes.PositionSnapshot);
            Assert.True(Contracts.ContractFileSchema.IsValidAdapterFrame(raw), raw);
            // Negative control: the adapter-sim defect (order.price instead of stop_price) is refused by the same validator.
            Assert.False(Contracts.ContractFileSchema.IsValidAdapterFrame(raw.Replace("\"stop_price\"", "\"price\"")));
            var payload = Newtonsoft.Json.Linq.JObject.Parse(raw)["payload"]!;
            Assert.Equal("reconnect", (string?)payload["trigger"]);
            var orders = payload["accounts"]![0]!["working_orders"]!;

            var stopSpec = orders.Single(o => (string?)o["child_role"] == "stop")["order"]!;
            Assert.Equal("stop", (string?)stopSpec["type"]);
            Assert.Equal(Newtonsoft.Json.Linq.JTokenType.String, stopSpec["stop_price"]!.Type);   // money is a string on the wire
            Assert.Equal(stop.StopPrice, decimal.Parse((string)stopSpec["stop_price"]!, System.Globalization.CultureInfo.InvariantCulture));
            Assert.Null(stopSpec["limit_price"]);
            Assert.Null(stopSpec["price"]);

            var targetSpec = orders.Single(o => (string?)o["child_role"] == "target")["order"]!;
            Assert.Equal("limit", (string?)targetSpec["type"]);
            Assert.Equal(Newtonsoft.Json.Linq.JTokenType.String, targetSpec["limit_price"]!.Type);
            Assert.Equal(target.LimitPrice, decimal.Parse((string)targetSpec["limit_price"]!, System.Globalization.CultureInfo.InvariantCulture));
            Assert.Null(targetSpec["stop_price"]);
            Assert.Null(targetSpec["price"]);
        }

        [Fact]
        public void Unknown_leg_after_restart_is_reported_and_marks_the_bracket_incomplete()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual()));
            h.AckAll();
            h.Restart();
            h.Platform.MarkUnknown(Sim101, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!.Handle);
            h.Restart();
            h.Connect();
            Assert.Contains(h.Reports(C1), r => r.ChildRole == ChildRole.Stop && r.Status == ReportStatus.Unknown);
            var order = h.Snapshots().Last().Accounts.Single().WorkingOrders.Single(o => o.ChildRole == ChildRole.Stop);
            Assert.Equal(WorkingOrderState.Unknown, order.State);
            Assert.Equal(C1, order.CommandId);
        }

        [Fact]
        public void Bracket_name_reference_survives_restart_and_still_drives_orphan_protection()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual()));
            h.Restart();
            h.Connect();
            h.Platform.SetPrice("MNQ", 21054.50m); // target hit after the restart, handles renewed
            h.Settle();
            Assert.Contains(h.Reports(C1), r => r.ChildRole == ChildRole.Stop && r.ReasonCode == ReasonCodes.OrderOrphanCancelled);
        }

        [Fact]
        public void Account_reset_is_a_discontinuity_never_fills()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual()));
            var fillReports = h.Reports().Count(r => r.Fill != null);
            h.Platform.ResetAccount(Sim101);
            h.Settle();
            Assert.Equal(fillReports, h.Reports().Count(r => r.Fill != null));
            var snapshot = h.Snapshots().Last();
            Assert.Equal(SnapshotTrigger.AccountReset, snapshot.Trigger);
            Assert.Empty(snapshot.Accounts.Single().Positions);
            Assert.Contains(ReasonCodes.SyncSimulationReset, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).LocalSyncReasons);
        }

        [Fact]
        public void Broker_disconnect_sends_a_change_snapshot_and_shows_bracket_integrity_unknown()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual()));
            h.Platform.SetConnected(Sim101, false);
            h.Settle();
            var snapshot = h.Snapshots().Last();
            Assert.Equal(SnapshotTrigger.Change, snapshot.Trigger);
            Assert.False(snapshot.Accounts.Single().Connected);
            var view = h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101);
            Assert.Contains(ReasonCodes.SyncBracketIntegrityUnknown, view.LocalSyncReasons);
            Assert.True(view.ShowLocalOcoNotice);

            h.Platform.SetConnected(Sim101, true);
            h.Settle();
            Assert.True(h.Snapshots().Last().Accounts.Single().Connected);
            Assert.DoesNotContain(ReasonCodes.SyncBracketIntegrityUnknown, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).LocalSyncReasons);
        }

        [Fact]
        public void Core_session_drop_with_working_protection_shows_bracket_integrity_unknown()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual()));
            h.Disconnect();
            Assert.Contains(ReasonCodes.SyncBracketIntegrityUnknown, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).AllSyncReasons);
        }

        [Fact]
        public void Welcome_reasons_are_shown_and_account_state_updates_respect_changed_at()
        {
            using var h = new EngineHarness();
            h.Connect(DefaultWelcome(SyncState.OutOfSync, "sync.bracket_integrity_unknown", "sync.disconnected_in_trade"));
            var view = h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101);
            Assert.Equal(SyncState.OutOfSync, view.CoreSyncState);
            Assert.Contains("sync.disconnected_in_trade", view.CoreSyncReasons);

            var t = h.Clock.UtcNow;
            h.Engine.ApplyCoreAccountState(Sim101, SyncState.InSync, new string[0], t);
            h.Engine.ApplyCoreAccountState(Sim101, SyncState.OutOfSync, new[] { "sync.x_y" }, t.AddSeconds(-1)); // older: ignored
            Assert.Equal(SyncState.InSync, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).CoreSyncState);
            h.Engine.ApplyCoreAccountState(Sim101, SyncState.OutOfSync, new[] { "sync.x_y" }, t); // equal: later received wins
            Assert.Equal(SyncState.OutOfSync, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).CoreSyncState);
        }

        [Fact]
        public void Unmapped_accounts_produce_no_reports()
        {
            using var h = new EngineHarness(p => p.AddAccount("Sim102"));
            h.Platform.SetPrice("MNQ", 21000m);
            h.Connect();
            h.Platform.ExternalFill("Sim102", Mnq, Side.Buy, 1, 21000m);
            h.Settle();
            Assert.DoesNotContain(h.Reports(), r => r.PlatformAccount == "Sim102");
            Assert.DoesNotContain(h.Snapshots().SelectMany(s => s.Accounts), a => a.PlatformAccount == "Sim102");
        }

        [Fact]
        public void Pruning_keeps_non_final_commands()
        {
            using var h = Ready();
            h.Platform.SetPrice("MNQ", 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Tick(TimeSpan.FromDays(8));
            h.Restart();
            h.Connect();
            h.Deliver(Cancel(C2, C1, ChildRole.Entry));
            Assert.Contains(h.Reports(C2), r => r.Status == ReportStatus.Cancelled); // C1 still known after pruning
        }
    }
}
