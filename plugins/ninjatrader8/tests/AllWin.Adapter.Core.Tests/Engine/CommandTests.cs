using System;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Protocol;
using Xunit;
using static AllWin.Adapter.Core.Tests.Engine.EngineHarness;

namespace AllWin.Adapter.Core.Tests.Engine
{
    public class CommandTests
    {
        private const string C1 = "0192b3c4-0000-7000-8000-000000000201";
        private const string C2 = "0192b3c4-0000-7000-8000-000000000202";
        private const string C3 = "0192b3c4-0000-7000-8000-000000000203";

        private static EngineHarness Ready(Action<Sim.SimulatedPlatform>? setup = null, WelcomePayload? welcome = null, TimeSpan? coreAhead = null)
        {
            var h = new EngineHarness(setup);
            h.Platform.SetPrice("MNQ", 21034.50m);
            h.Connect(welcome, coreAhead);
            return h;
        }

        [Fact]
        public void Entry_is_acked_reported_received_and_submitted_with_the_name_reference_and_day_tif()
        {
            using var h = Ready();
            var msgId = h.Deliver(Place(C1, Side.Buy, 2));
            Assert.Contains(msgId, h.Acks());
            var reports = h.Reports(C1);
            Assert.Equal(ReportStatus.Received, reports[0].Status);
            Assert.Equal(ChildRole.Entry, reports[0].ChildRole);
            var order = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e")!;
            Assert.Equal(41, order.Name.Length);
            Assert.Equal(TimeInForce.Day, order.TimeInForce);
            Assert.Contains(reports, r => r.Status == ReportStatus.Working && r.PlatformOrderId == order.PlatformOrderId);
            var filled = Assert.Single(reports, r => r.Status == ReportStatus.Filled);
            Assert.Equal(21034.50m, filled.Fill!.Price);
            Assert.Equal(2, filled.PositionAfter!.Quantity);
        }

        [Fact]
        public void Entry_received_after_expires_at_on_the_skew_corrected_clock_is_expired_and_never_submitted()
        {
            // Local clock 14:31:00; Core clock is 6 s ahead; expires_at 14:31:05 → already past for the Core.
            using var h = Ready(coreAhead: TimeSpan.FromSeconds(6));
            h.Deliver(Place(C1, Side.Buy, 1));
            var report = Assert.Single(h.Reports(C1));
            Assert.Equal(ReportStatus.Expired, report.Status);
            Assert.Equal(ReasonCodes.CommandExpired, report.ReasonCode);
            Assert.Empty(h.Platform.CallLog);
        }

        [Fact]
        public void Same_entry_without_skew_is_executed()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            Assert.Contains(h.Reports(C1), r => r.Status == ReportStatus.Filled);
        }

        [Fact]
        public void Exits_never_expire()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2));
            h.Clock.Advance(TimeSpan.FromMinutes(10));
            h.Deliver(Place(C2, Side.Sell, 1, PositionEffect.Reduce, expiresAt: h.Clock.UtcNow.AddMinutes(-5)));
            Assert.Contains(h.Reports(C2), r => r.Status == ReportStatus.Filled && r.ChildRole == ChildRole.Exit);
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Reverse_place_waits_for_the_flatten_and_continues_when_flat()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Flatten(C2, FlattenReason.Reverse));
            h.Deliver(Place(C3, Side.Sell, 1, dependsOn: C2));
            Assert.Contains(h.Reports(C2), r => r.Status == ReportStatus.Filled && r.ChildRole == ChildRole.Flatten);
            Assert.Contains(h.Reports(C3), r => r.Status == ReportStatus.Filled);
            Assert.Equal(-1, h.Platform.Position(Sim101, "MNQ"));
            var submits = h.Platform.CallLog.Where(c => c.StartsWith("submit", StringComparison.Ordinal)).ToList();
            Assert.Equal(new[] { "submit aw:" + C1 + ":e", "submit aw:" + C2 + ":x", "submit aw:" + C3 + ":e" }, submits);
        }

        [Fact]
        public void Dependent_entry_that_expired_while_waiting_is_expired_at_dispatch()
        {
            using var h = Ready();
            // The prerequisite limit order rests unfilled, so the dependent entry waits in the queue.
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Deliver(Place(C2, Side.Buy, 1, dependsOn: C1));
            Assert.DoesNotContain(h.Platform.CallLog, c => c.Contains(C2));
            h.Clock.Advance(TimeSpan.FromSeconds(10));
            h.Platform.FillOrder(Sim101, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e")!.Handle, 1, 20000m);
            h.Settle();
            var report = h.Reports(C2).Last();
            Assert.Equal(ReportStatus.Expired, report.Status);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.Contains(C2));
        }

        [Fact]
        public void Failed_prerequisite_rejects_the_dependent_command()
        {
            using var h = Ready();
            h.Platform.RejectNextOrder = true;
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Place(C2, Side.Buy, 1, dependsOn: C1));
            Assert.Equal(ReasonCodes.OrderPlatformRejected, h.Reports(C1).Last().ReasonCode);
            var dependent = h.Reports(C2).Last();
            Assert.Equal(ReportStatus.Rejected, dependent.Status);
            Assert.Equal(ReasonCodes.CommandPrerequisiteFailed, dependent.ReasonCode);
        }

        [Fact]
        public void Unknown_prerequisite_rejects_the_dependent_command()
        {
            using var h = Ready();
            h.Deliver(Place(C2, Side.Buy, 1, dependsOn: C1));
            Assert.Equal(ReasonCodes.CommandPrerequisiteFailed, h.Reports(C2).Last().ReasonCode);
        }

        [Fact]
        public void Commands_for_unmapped_accounts_are_rejected()
        {
            using var h = Ready();
            var command = Place(C1, Side.Buy, 1);
            command.AccountId = "0192b3c4-0000-7000-8000-0000000000ff";
            h.Deliver(command);
            var report = Assert.Single(h.Reports(C1));
            Assert.Equal(ReasonCodes.CommandAccountNotMapped, report.ReasonCode);
            Assert.Equal(command.AccountId, report.AccountId); // answered under the Core's own id
            Assert.Empty(h.Platform.CallLog);

            var wrongName = Place(C2, Side.Buy, 1);
            wrongName.PlatformAccount = "Sim102";
            h.Deliver(wrongName);
            Assert.Equal(ReasonCodes.CommandAccountNotMapped, h.Reports(C2).Single().ReasonCode);
            Assert.Empty(h.Platform.CallLog);
        }

        [Fact]
        public void Master_only_accounts_do_not_execute()
        {
            var welcome = DefaultWelcome();
            welcome.Accounts[0].Roles = new System.Collections.Generic.List<AccountRole> { AccountRole.Master };
            using var h = Ready(welcome: welcome);
            h.Deliver(Place(C1, Side.Buy, 1));
            Assert.Equal(ReasonCodes.CommandAccountNotDestination, h.Reports(C1).Single().ReasonCode);
            Assert.Empty(h.Platform.CallLog);
        }

        [Theory]
        [InlineData(AccountKind.Live)]
        [InlineData(AccountKind.Unknown)]
        public void Development_build_refuses_non_simulation_accounts_even_if_the_core_maps_them(AccountKind kind)
        {
            var welcome = DefaultWelcome();
            welcome.SimulationOnly = false;
            welcome.Accounts[0].PlatformAccount = "Live1";
            using var h = Ready(p => p.AddAccount("Live1", kind, OcoLocation.Server, "Rithmic"), welcome);
            var command = Place(C1, Side.Buy, 1);
            command.PlatformAccount = "Live1";
            h.Deliver(command);
            Assert.Equal(ReasonCodes.CommandAccountNotSimulation, h.Reports(C1).Single().ReasonCode);
            Assert.Empty(h.Platform.CallLog);
        }

        [Fact]
        public void Automatic_actions_never_touch_an_account_refused_by_the_guard()
        {
            var welcome = DefaultWelcome();
            welcome.Accounts[0].PlatformAccount = "Live1";
            using var h = Ready(p => p.AddAccount("Live1", AccountKind.Live, OcoLocation.Local, "Rithmic"), welcome);
            h.Platform.SetPrice("MNQ", 21000m);
            // An order carrying an All Win name exists on the live account (placed by someone else), then the position goes flat.
            h.Platform.Submit("Live1", new[] { new Core.Platform.OrderRequest { Name = "aw:" + C1 + ":s", Instrument = Mnq, Side = Side.Sell, Type = OrderType.Stop, Quantity = 1, StopPrice = 20000m } });
            h.Platform.ExternalFill("Live1", Mnq, Side.Buy, 1, 21000m);
            h.Platform.ExternalFill("Live1", Mnq, Side.Sell, 1, 21000m);
            h.Platform.CallLog.Clear();
            h.Settle();
            Assert.Empty(h.Platform.CallLog); // no orphan cancel on a non-simulation account
            Assert.Contains(h.Log, l => l.Contains("guard.refused"));
        }

        [Fact]
        public void Release_build_obeys_simulation_only()
        {
            var welcome = DefaultWelcome();
            welcome.Accounts[0].PlatformAccount = "Live1";
            using var h = new EngineHarness(p => p.AddAccount("Live1", AccountKind.Live), new EngineOptions { Build = AdapterBuild.Release });
            h.Connect(welcome);
            var command = Place(C1, Side.Buy, 1);
            command.PlatformAccount = "Live1";
            h.Deliver(command);
            Assert.Equal(ReasonCodes.CommandAccountNotSimulation, h.Reports(C1).Single().ReasonCode);
            Assert.Empty(h.Platform.CallLog);
        }

        [Fact]
        public void Disconnected_platform_account_rejects_without_submitting()
        {
            using var h = Ready();
            h.Platform.SetConnected(Sim101, false);
            h.Settle();
            h.Deliver(Place(C1, Side.Buy, 1));
            Assert.Equal(ReasonCodes.CommandPlatformDisconnected, h.Reports(C1).Last().ReasonCode);
            Assert.Empty(h.Platform.CallLog);
        }

        [Fact]
        public void Reduce_when_flat_is_cancelled_nothing_to_reduce_and_never_opens()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Sell, 1, PositionEffect.Reduce));
            var report = h.Reports(C1).Last();
            Assert.Equal(ReportStatus.Cancelled, report.Status);
            Assert.Equal(ReasonCodes.CommandNothingToReduce, report.ReasonCode);
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Reduce_on_the_same_side_never_adds()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Place(C2, Side.Buy, 1, PositionEffect.Reduce));
            Assert.Equal(ReasonCodes.CommandNothingToReduce, h.Reports(C2).Last().ReasonCode);
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Reduce_larger_than_the_position_is_capped_and_never_flips()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2));
            h.Deliver(Place(C2, Side.Sell, 5, PositionEffect.Reduce));
            var working = h.Reports(C2).First(r => r.Status == ReportStatus.Working);
            Assert.Equal(2, working.Quantity);
            Assert.Equal(ReasonCodes.OrderQuantityCapped, working.ReasonCode);
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Submit_refused_by_the_platform_is_rejected_with_platform_reason()
        {
            using var h = Ready();
            h.Platform.FailNextSubmit = true;
            h.Deliver(Place(C1, Side.Buy, 1));
            var report = h.Reports(C1).Last();
            Assert.Equal(ReportStatus.Rejected, report.Status);
            Assert.Equal(ReasonCodes.OrderPlatformRejected, report.ReasonCode);
        }

        [Fact]
        public void Unknown_instrument_is_rejected()
        {
            using var h = Ready();
            var command = Place(C1, Side.Buy, 1);
            command.Instrument = new Instrument("ZZ", "2026-12");
            h.Deliver(command);
            Assert.Equal(ReasonCodes.CommandInstrumentUnavailable, h.Reports(C1).Last().ReasonCode);
        }

        [Fact]
        public void Flatten_executes_even_when_entries_are_blocked()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(new LimitsPayload
            {
                Accounts =
                {
                    new AccountLimits
                    {
                        AccountId = AccountId, PlatformAccount = Sim101, Version = 1, MaxPosition = 5, WarningThresholdPct = 80,
                        TradingDay = new TradingDaySpec { ResetLocalTime = "17:00", TimeZone = "America/Chicago" }, BlockedUntil = h.Clock.UtcNow.AddHours(1),
                    },
                },
            });
            h.Deliver(Place(C2, Side.Buy, 1, expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            Assert.Equal(ReasonCodes.RiskAccountBlocked, h.Reports(C2).Last().ReasonCode);
            h.Deliver(Flatten(C3));
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Commands_of_one_account_are_submitted_in_arrival_order()
        {
            using var h = Ready();
            h.Platform.SetPrice("MNQ", 0.25m);
            var ids = Enumerable.Range(0, 5).Select(i => "0192b3c4-0000-7000-8000-0000000003" + i.ToString("D2")).ToList();
            foreach (var id in ids)
                h.Deliver(Place(id, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 0.25m }));
            var submitted = h.Platform.CallLog.Where(c => c.StartsWith("submit", StringComparison.Ordinal)).Select(c => c.Substring(10, 36)).ToList();
            Assert.Equal(ids, submitted);
        }
    }
}
