using System;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;
using Xunit;
using static AllWin.Adapter.Core.Tests.Engine.EngineHarness;

namespace AllWin.Adapter.Core.Tests.Engine
{
    /// <summary>S2 AC-3, S9 (orphan protection), Architect answers 1, 7, 9 and 4.</summary>
    public class BracketTests
    {
        private const string C1 = "0192b3c4-0000-7000-8000-000000000401";
        private const string C2 = "0192b3c4-0000-7000-8000-000000000402";
        private const string C3 = "0192b3c4-0000-7000-8000-000000000403";
        private const string C4 = "0192b3c4-0000-7000-8000-000000000404";

        private static EngineHarness Ready(decimal price = 21034.50m, bool oco = true)
        {
            var h = new EngineHarness();
            h.Platform.OcoEnforced = oco;
            h.Platform.SetPrice("MNQ", price);
            h.Connect();
            return h;
        }

        [Fact]
        public void Entry_fill_places_stop_and_target_at_the_tick_distances_from_the_fill_price()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            var stop = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!;
            var target = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":t")!;
            Assert.Equal(21024.50m, stop.StopPrice);
            Assert.Equal(OrderType.Stop, stop.Type);
            Assert.Equal(21054.50m, target.LimitPrice);
            Assert.Equal(Side.Sell, stop.Side);
            Assert.Equal(2, stop.Quantity);
            Assert.Equal(TimeInForce.Gtc, stop.TimeInForce);
            var reports = h.Reports(C1);
            Assert.Contains(reports, r => r.ChildRole == ChildRole.Stop && r.Status == ReportStatus.Working && r.Order!.StopPrice == 21024.50m);
            Assert.Contains(reports, r => r.ChildRole == ChildRole.Target && r.Status == ReportStatus.Working && r.Order!.LimitPrice == 21054.50m);
        }

        [Fact]
        public void Short_entry_mirrors_the_distances()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Sell, 1, protection: Individual(40, 80)));
            Assert.Equal(21044.50m, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!.StopPrice);
            Assert.Equal(21014.50m, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":t")!.LimitPrice);
        }

        [Fact]
        public void Partial_fills_submit_on_the_first_fill_then_resize_and_reprice_from_the_vwap()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 3, protection: Individual(4, 80), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20990m }));
            var entry = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e")!;
            h.Platform.FillOrder(Sim101, entry.Handle, 1, 20990m);
            h.Settle();
            var stop = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!;
            Assert.Equal(1, stop.Quantity);
            Assert.Equal(20989m, stop.StopPrice);

            h.Platform.FillOrder(Sim101, entry.Handle, 2, 20990.25m); // VWAP 20990.1666…: off tick
            h.Settle();
            stop = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!;
            var target = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":t")!;
            Assert.Equal(3, stop.Quantity);
            Assert.Equal(3, target.Quantity);
            // Rounded toward the entry (tighter): stop up, target down.
            Assert.Equal(20989.25m, stop.StopPrice);   // 20989.1666… → 20989.25
            Assert.Equal(21010.00m, target.LimitPrice); // 21010.1666… → 21010.00
            Assert.Single(h.Platform.CallLog, c => c.StartsWith("submit aw:" + C1 + ":s", StringComparison.Ordinal));
            Assert.Contains(h.Reports(C1), r => r.Status == ReportStatus.PartiallyFilled && r.Fill!.Quantity == 1);
        }

        [Fact]
        public void Stop_hit_flattens_and_the_target_is_cancelled_by_the_oco()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            h.Platform.SetPrice("MNQ", 21024.50m);
            h.Settle();
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
            Assert.Contains(h.Reports(C1), r => r.ChildRole == ChildRole.Stop && r.Status == ReportStatus.Filled && r.PositionAfter!.Quantity == 0);
            Assert.Contains(h.Reports(C1), r => r.ChildRole == ChildRole.Target && r.Status == ReportStatus.Cancelled);
        }

        [Fact]
        public void Without_oco_the_orphan_rule_cancels_the_remaining_leg_when_flat()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            h.Platform.SetPrice("MNQ", 21054.50m); // target hit
            h.Settle();
            var cancelled = Assert.Single(h.Reports(C1), r => r.ChildRole == ChildRole.Stop && r.Status == ReportStatus.Cancelled);
            Assert.Equal(ReasonCodes.OrderOrphanCancelled, cancelled.ReasonCode);
            Assert.Equal(PlatformOrderStateCancelled, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!.State);
        }

        [Fact]
        public void Manual_close_in_the_platform_also_cancels_the_orphans()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80)));
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 21030m);
            h.Settle();
            Assert.Equal(2, h.Reports(C1).Count(r => r.Status == ReportStatus.Cancelled && r.ReasonCode == ReasonCodes.OrderOrphanCancelled));
            var external = h.Reports().Single(r => r.Origin == OrderOrigin.External && r.Status == ReportStatus.Filled);
            Assert.Null(external.CommandId);
            Assert.Null(external.ChildRole);
        }

        [Fact]
        public void Individual_mode_cancels_only_the_closed_entrys_opposite_leg()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80)));
            h.Platform.SetPrice("MNQ", 21040m);
            h.Deliver(Place(C2, Side.Buy, 1, protection: Individual(4, 80), expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            // C2 stop at 21039; C1 stop at 21024.50. Price drops to 21039: only C2's stop fills.
            h.Platform.SetPrice("MNQ", 21039m);
            h.Settle();
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
            Assert.Equal(ReasonCodes.OrderOrphanCancelled, h.Reports(C2).Single(r => r.ChildRole == ChildRole.Target && r.Status == ReportStatus.Cancelled).ReasonCode);
            Assert.DoesNotContain(h.Reports(C1), r => r.Status == ReportStatus.Cancelled);
        }

        [Fact]
        public void Shared_mode_owner_keeps_one_bracket_resized_and_repriced_on_adds()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Shared(8, 16)));
            h.Platform.SetPrice("MNQ", 21002m);
            h.Deliver(Place(C2, Side.Buy, 1, protection: Shared(40, 80), expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            // Average 21001; owner's ticks (8/16 = 2/4 points) are used, the add's ticks are ignored.
            var stop = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!;
            var target = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":t")!;
            Assert.Equal(2, stop.Quantity);
            Assert.Equal(20999m, stop.StopPrice);
            Assert.Equal(21005m, target.LimitPrice);
            Assert.Null(h.Platform.OrderByName(Sim101, "aw:" + C2 + ":s"));
            Assert.All(h.Reports(C2), r => Assert.Equal(ChildRole.Entry, r.ChildRole)); // add: entry reports only
            Assert.Contains(h.Reports(C1), r => r.ChildRole == ChildRole.Stop && r.Quantity == 2);

            h.Deliver(Modify(C3, C2, ChildRole.Stop, delta: -1m));
            Assert.Equal(ReasonCodes.CommandTargetNotFound, h.Reports(C3).Last().ReasonCode);
        }

        [Fact]
        public void Shared_ownership_ends_when_flat_and_the_next_opening_command_owns()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Shared(4, 8)));
            h.Deliver(Flatten(C2));
            h.Deliver(Place(C3, Side.Buy, 1, protection: Shared(4, 8), expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            Assert.NotNull(h.Platform.OrderByName(Sim101, "aw:" + C3 + ":s"));
            Assert.True(h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!.IsTerminal);
        }

        [Fact]
        public void Protective_submit_failure_is_reported_protection_failed()
        {
            using var h = new EngineHarness();
            h.Platform.SetPrice("MNQ", 21034.50m);
            h.Connect();
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 21000m }));
            h.Platform.FailNextSubmit = true;
            h.Platform.FillOrder(Sim101, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e")!.Handle, 1, 21000m);
            h.Settle();
            var failed = h.Reports(C1).Where(r => r.ReasonCode == ReasonCodes.OrderProtectionFailed).ToList();
            Assert.Equal(new[] { ChildRole.Stop, ChildRole.Target }, failed.Select(r => r.ChildRole!.Value));
            Assert.All(failed, r => Assert.Equal(ReportStatus.Rejected, r.Status));
        }

        [Fact]
        public void Modify_moves_the_stop_with_an_outcome_report_without_fill_and_a_lifecycle_report()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            h.Deliver(Modify(C2, C1, ChildRole.Stop, delta: -2.5m));
            Assert.Equal(21022.00m, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!.StopPrice);
            var outcome = Assert.Single(h.Reports(C2), r => r.Status == ReportStatus.Working);
            Assert.Equal(ChildRole.Stop, outcome.ChildRole);
            Assert.Equal(21022.00m, outcome.Order!.StopPrice);
            Assert.Null(outcome.Fill);
            Assert.Null(outcome.PositionAfter);
            Assert.Contains(h.Reports(C1), r => r.ChildRole == ChildRole.Stop && r.Status == ReportStatus.Working && r.Order!.StopPrice == 21022.00m);

            h.Deliver(Place(C3, Side.Buy, 1, dependsOn: C2, expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            Assert.Contains(h.Reports(C3), r => r.Status == ReportStatus.Filled); // all targets applied = success
        }

        [Fact]
        public void Modify_quantity_and_failed_target_make_the_command_fail_for_dependents()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            var modify = Modify(C2, C1, ChildRole.Stop, quantity: 1);
            modify.Modify!.Targets.Add(new OrderTarget { CommandId = C4, ChildRole = ChildRole.Target });
            h.Deliver(modify);
            Assert.Equal(1, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")!.Quantity); // applied as is, no rollback
            Assert.Contains(h.Reports(C2), r => r.Status == ReportStatus.Rejected && r.ReasonCode == ReasonCodes.CommandTargetNotFound);
            h.Deliver(Flatten(C3));
            var dependent = Place(C4, Side.Buy, 1, dependsOn: C2, expiresAt: h.Clock.UtcNow.AddSeconds(5));
            h.Deliver(dependent);
            Assert.Equal(ReasonCodes.CommandPrerequisiteFailed, h.Reports(C4).Last().ReasonCode);
        }

        [Fact]
        public void Cancel_confirms_from_the_platform_state()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Deliver(Cancel(C2, C1, ChildRole.Entry));
            Assert.Contains(h.Reports(C2), r => r.Status == ReportStatus.Cancelled && r.ChildRole == ChildRole.Entry && r.Fill == null);
            Assert.Contains(h.Reports(C1), r => r.Status == ReportStatus.Cancelled);
        }

        [Fact]
        public void Flatten_cancels_all_working_orders_first_then_closes_the_position()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            h.Deliver(Flatten(C2));
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
            var log = h.Platform.CallLog;
            var cancelIndex = log.FindIndex(c => c.StartsWith("cancel", StringComparison.Ordinal));
            var closeIndex = log.FindIndex(c => c == "submit aw:" + C2 + ":x");
            Assert.True(cancelIndex >= 0 && cancelIndex < closeIndex);
            var fill = Assert.Single(h.Reports(C2), r => r.Status == ReportStatus.Filled);
            Assert.Equal(ChildRole.Flatten, fill.ChildRole);
            Assert.Equal(0, fill.PositionAfter!.Quantity);
            Assert.Null(fill.PositionAfter.AveragePrice);
        }

        [Fact]
        public void Flatten_when_already_flat_is_cancelled_already_flat_and_a_success()
        {
            using var h = Ready();
            h.Deliver(Flatten(C1, FlattenReason.Reverse));
            var report = h.Reports(C1).Last();
            Assert.Equal(ReportStatus.Cancelled, report.Status);
            Assert.Equal(ReasonCodes.CommandAlreadyFlat, report.ReasonCode);
            Assert.Equal(ChildRole.Flatten, report.ChildRole);
            h.Deliver(Place(C2, Side.Sell, 1, dependsOn: C1));
            Assert.Contains(h.Reports(C2), r => r.Status == ReportStatus.Filled);
        }

        [Fact]
        public void Flatten_with_reason_orphan_fill_closes_only_what_is_there()
        {
            // Q-O1 (b): the Core sends flatten(orphan_fill) with step orphan_flatten:<event_id>; the adapter executes it as any flatten.
            using var h = Ready();
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 2, 21030m);
            h.Settle();
            h.Deliver(Flatten(C1, FlattenReason.OrphanFill));
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
            Assert.Equal(Side.Buy, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":x")!.Side);
        }

        [Fact]
        public void Flatten_fails_when_cancels_are_never_confirmed()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Platform.FailNextCancel = true;
            h.Deliver(Flatten(C2));
            var report = h.Reports(C2).Last();
            Assert.Equal(ReportStatus.Rejected, report.Status);
            Assert.Equal(ReasonCodes.OrderCancelFailed, report.ReasonCode);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.Contains(C2 + ":x"));
        }

        private const Core.Platform.PlatformOrderState PlatformOrderStateCancelled = Core.Platform.PlatformOrderState.Cancelled;
    }
}
