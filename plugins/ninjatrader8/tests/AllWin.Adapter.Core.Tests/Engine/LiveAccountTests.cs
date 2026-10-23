using System;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;
using Xunit;
using static AllWin.Adapter.Core.Tests.Engine.EngineHarness;
using static AllWin.Adapter.Core.Tests.Engine.LiveAccountTableTests;

namespace AllWin.Adapter.Core.Tests.Engine
{
    /// <summary>ADR-0005 N11 (U15): the two gates, the opt-in document and what the plugin does on its own.</summary>
    public class LiveAccountTests
    {
        private const string E1 = "0192b3c4-0000-7000-8000-000000000d01";
        private const string E2 = "0192b3c4-0000-7000-8000-000000000d02";
        private const string F1 = "0192b3c4-0000-7000-8000-000000000d11";

        private static AccountView View(EngineHarness h) => h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101);

        private static bool Submitted(EngineHarness h, string id) => h.Platform.CallLog.Any(c => c.StartsWith("submit", StringComparison.Ordinal) && c.Contains(id));

        private static void AssertRefused(EngineHarness h, string id)
        {
            var final = h.Reports(id).Last();
            Assert.Equal(ReportStatus.Rejected, final.Status);
            Assert.Equal(ReasonCodes.CommandAccountNotSimulation, final.ReasonCode);
            Assert.False(Submitted(h, id));
        }

        [Fact]
        public void State_B_both_gates_open_executes_normally()
        {
            using var h = LiveHarness();
            Assert.Equal(LiveState.B, View(h).LiveState);
            Assert.True(View(h).ExecutionAllowed);
            Assert.True(h.Engine.LiveAccountsAllowed);
            h.Deliver(Place(E1, Side.Buy, 1, protection: Individual(40, 80)));
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
            Assert.NotNull(h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s"));
        }

        [Fact]
        public void Default_off_a_live_account_gets_no_platform_write_at_all()
        {
            var options = new EngineOptions { Build = AdapterBuild.Release };
            using var h = new EngineHarness(p => p.AddAccount(Sim101, AccountKind.Live, OcoLocation.Local, Connection, Provider), options); // consent missing = off
            h.Platform.SetPrice("MNQ", 21000m);
            var welcome = DefaultWelcome();
            welcome.SimulationOnly = false;
            h.Connect(welcome);
            Assert.Equal(LiveState.A, View(h).LiveState);
            Assert.False(h.Engine.LiveAccountsAllowed);
            h.Deliver(Place(E1, Side.Buy, 1));
            AssertRefused(h, E1);
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m);
            h.Settle();
            h.Deliver(Flatten(F1));
            AssertRefused(h, F1);
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ")); // not even closing (Vigia M1)
        }

        [Fact]
        public void Opt_in_without_this_account_confirmed_is_state_A()
        {
            using var h = LiveHarness(confirm: false);
            Assert.Equal(LiveState.A, View(h).LiveState);
            h.Deliver(Place(E1, Side.Buy, 1));
            AssertRefused(h, E1);
        }

        [Fact]
        public void Core_gate_simulation_only_true_keeps_a_confirmed_account_out_of_state_B()
        {
            using var h = LiveHarness(simulationOnly: true);
            Assert.Equal(LiveState.C0, View(h).LiveState);
            h.Deliver(Place(E1, Side.Buy, 1));
            AssertRefused(h, E1);
        }

        [Fact]
        public void A_new_welcome_with_simulation_only_true_applies_the_same_table()
        {
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Disconnect();
            h.Connect(DefaultWelcome()); // simulation_only = true
            Assert.Equal(PlatformOrderState.Cancelled, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":e")!.State);
        }

        [Fact]
        public void Development_build_never_writes_to_a_live_account_even_with_both_gates_open()
        {
            using var h = LiveHarness(build: AdapterBuild.Development);
            Assert.Null(View(h).LiveState);
            Assert.False(View(h).ExecutionAllowed);
            h.Deliver(Place(E1, Side.Buy, 1));
            AssertRefused(h, E1);
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m);
            h.Settle();
            h.Deliver(Flatten(F1));
            AssertRefused(h, F1);
            Assert.Empty(h.Platform.CallLog); // no submit, change or cancel of any kind (N4)
        }

        [Fact]
        public void Opt_in_applies_to_commands_received_afterwards()
        {
            using var h = LiveHarness(optInAt: new DateTime(2026, 10, 22, 14, 32, 0, DateTimeKind.Utc)); // after the harness clock
            h.Deliver(Place(E1, Side.Buy, 1));
            AssertRefused(h, E1);
            h.Clock.Advance(TimeSpan.FromMinutes(2));
            h.Deliver(Place(E2, Side.Buy, 1, expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            Assert.True(Submitted(h, E2));
        }

        [Fact]
        public void Live_unknown_live_keeps_the_confirmation_and_returns_to_its_previous_state()
        {
            using var h = LiveHarness();
            h.Platform.SetKind(Sim101, AccountKind.Unknown); // e.g. NT8 start-up or a reconnect
            h.Settle();
            Assert.Equal(LiveState.A, View(h).LiveState);
            Assert.Single(h.Consent.Confirmed);
            h.Deliver(Place(E1, Side.Buy, 1));
            AssertRefused(h, E1); // every Core write is refused while unknown (fail closed)
            h.Platform.SetKind(Sim101, AccountKind.Live);
            h.Settle();
            Assert.Equal(LiveState.B, View(h).LiveState);
            h.Deliver(Place(E2, Side.Buy, 1));
            Assert.True(Submitted(h, E2));
        }

        [Fact]
        public void Live_unknown_simulation_invalidates_the_confirmation()
        {
            using var h = LiveHarness();
            h.Platform.SetKind(Sim101, AccountKind.Unknown);
            h.Settle();
            h.Platform.SetKind(Sim101, AccountKind.Simulation);
            h.Settle();
            Assert.Empty(h.Consent.Confirmed);
            Assert.Contains(h.Log, l => l.Contains("live.confirmation_invalidated"));
            h.Platform.SetKind(Sim101, AccountKind.Live);
            h.Settle();
            Assert.Equal(LiveState.A, View(h).LiveState);
        }

        [Fact]
        public void Transient_unknown_keeps_consented_orders_and_still_reacts_to_its_own_legs()
        {
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 2, protection: Individual()));
            h.Deliver(Place(E2, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Platform.SetKind(Sim101, AccountKind.Unknown);
            h.Settle();
            Assert.NotEqual(PlatformOrderState.Cancelled, h.Platform.OrderByName(Sim101, "aw:" + E2 + ":e")!.State); // no row-2 self-cancel
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 21000m);
            h.Settle();
            Assert.Equal(1, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity); // row 4 on its own
        }

        [Fact]
        public void Unknown_for_more_than_10_seconds_alerts_and_the_alert_clears_when_known()
        {
            using var h = LiveHarness();
            h.Platform.SetKind(Sim101, AccountKind.Unknown);
            h.Settle();
            h.Tick(TimeSpan.FromSeconds(5));
            Assert.DoesNotContain(AdapterEngine.NoticeKindUnknown, View(h).Notices);
            h.Tick(TimeSpan.FromSeconds(6));
            Assert.Contains(AdapterEngine.NoticeKindUnknown, View(h).Notices);
            h.Platform.SetKind(Sim101, AccountKind.Live);
            h.Settle();
            h.Tick(TimeSpan.FromMilliseconds(250));
            Assert.DoesNotContain(AdapterEngine.NoticeKindUnknown, View(h).Notices);
        }

        [Theory]
        [InlineData(LiveOffPolicy.CancelEntriesKeepExits)]
        [InlineData(LiveOffPolicy.RefuseAll)]
        public void Off_cancels_working_All_Win_entries_at_once_and_never_touches_other_orders(LiveOffPolicy u19)
        {
            using var h = LiveHarness(u19);
            h.Deliver(Place(E1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Platform.Submit(Sim101, new[] { new OrderRequest { Name = "Manual", Instrument = Mnq, Side = Side.Buy, Type = OrderType.Limit, Quantity = 1, LimitPrice = 19000m } });
            h.Settle();
            h.Consent.Block();
            h.Engine.OnLiveConsentChanged();
            h.Settle();
            Assert.Equal(PlatformOrderState.Cancelled, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":e")!.State);
            Assert.Contains(h.Reports(E1), r => r.Status == ReportStatus.Cancelled);
            Assert.NotEqual(PlatformOrderState.Cancelled, h.Platform.OrderByName(Sim101, "Manual")!.State);
            Assert.Contains(h.Log, l => l.Contains("live.consent") && l.Contains("at 2026-10-22T"));
            Assert.Contains(h.Log, l => l.Contains("live.action"));
            Assert.False(h.Engine.LiveAccountsAllowed);
        }

        [Theory]
        [InlineData(LiveState.A, LiveOffPolicy.CancelEntriesKeepExits, true)]
        [InlineData(LiveState.C1, LiveOffPolicy.CancelEntriesKeepExits, false)] // U19 (a): exits continue after "off"
        [InlineData(LiveState.C1, LiveOffPolicy.RefuseAll, true)]               // U19 (b): behaves as A
        public void Working_All_Win_exit_is_undone_on_its_own_where_exits_are_refused(LiveState state, LiveOffPolicy u19, bool cancelled)
        {
            using var h = LiveHarness(u19);
            h.Deliver(Place(E1, Side.Buy, 2));
            h.Deliver(Place(E2, Side.Sell, 1, PositionEffect.Reduce, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 22000m }, positionRef: E1));
            Enter(h, state);
            var exit = h.Platform.OrderByName(Sim101, "aw:" + E2 + ":x")!;
            Assert.Equal(cancelled, exit.State == PlatformOrderState.Cancelled);
            if (cancelled)
                Assert.Contains(h.Reports(E2), r => r.Status == ReportStatus.Cancelled && r.ChildRole == ChildRole.Exit
                    && r.ReasonCode == ReasonCodes.OrderLiveConsentCancelled);
        }

        [Theory]
        [InlineData(LiveState.A)]
        [InlineData(LiveState.C1)]
        public void Modify_of_a_working_entry_is_row_1_and_refused_outside_B(LiveState state)
        {
            // order_targets cannot name an exit (contract v1), so row 1 is checked on an entry whose cancel at the
            // transition failed and is still working.
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1));
            h.Deliver(Place(E2, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Platform.FailNextCancel = true;
            Enter(h, state);
            Assert.NotEqual(PlatformOrderState.Cancelled, h.Platform.OrderByName(Sim101, "aw:" + E2 + ":e")!.State);
            h.Deliver(Modify(F1, E2, ChildRole.Entry, delta: 500m));
            AssertRefused(h, F1);
            Assert.Equal(20000m, h.Platform.OrderByName(Sim101, "aw:" + E2 + ":e")!.LimitPrice);
        }

        [Theory]
        [InlineData(LiveState.A)]
        [InlineData(LiveState.C1)]
        [InlineData(LiveState.C2)]
        public void A_write_that_names_no_row_is_refused_outside_B(LiveState state)
        {
            // Vigia L1: a modify of a stop with its current quantity and no price change classifies to no row.
            using var h = LiveHarness();
            if (state == LiveState.C2)
            {
                h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m);
                h.Settle();
            }
            h.Deliver(Place(E1, Side.Buy, 1, protection: Individual(40, 80)));
            Enter(h, state);
            h.Platform.CallLog.Clear();
            h.Deliver(Modify(F1, E1, ChildRole.Stop, quantity: h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity));
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("change", StringComparison.Ordinal));
            AssertRefused(h, F1);
        }

        [Fact]
        public void A_table_check_while_a_fill_is_queued_never_persists_an_unknown_identity()
        {
            // Code Review M1: C1 after "off"; an external add is still queued when the Core sends a flatten.
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1));
            Enter(h, LiveState.C1);
            var queued = new System.Collections.Generic.List<PlatformEvent>();
            var sink = h.Platform.EventSink;
            h.Platform.EventSink = queued.Add;
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m);
            h.Deliver(Flatten(F1)); // the position is ahead of the record: C2 for this check only (fail closed)
            AssertRefused(h, F1);
            h.Platform.EventSink = sink;
            foreach (var e in queued) h.Engine.OnPlatformEvent(e);
            h.Settle();
            Assert.Equal(LiveState.C1, View(h).LiveState); // the add kept the All Win identity
            h.Deliver(Flatten(E2));
            Assert.Contains(h.Reports(E2), r => r.Status == ReportStatus.Filled);
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void Two_partials_of_a_consented_entry_after_off_keep_the_All_Win_identity()
        {
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1));
            h.Deliver(Place(E2, Side.Buy, 2, protection: Individual(40, 80), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Platform.DeferCancels = true;
            Enter(h, LiveState.C1);
            var handle = h.Platform.OrderByName(Sim101, "aw:" + E2 + ":e")!.Handle;
            var queued = new System.Collections.Generic.List<PlatformEvent>();
            var sink = h.Platform.EventSink;
            h.Platform.EventSink = queued.Add;
            h.Platform.FillOrder(Sim101, handle, 1, 20000m);
            h.Platform.FillOrder(Sim101, handle, 1, 20000m); // fill N+1 already moved the platform while N is processed
            h.Platform.EventSink = sink;
            foreach (var e in queued) h.Engine.OnPlatformEvent(e);
            h.Settle();
            Assert.Equal(LiveState.C1, View(h).LiveState);
            Assert.DoesNotContain(h.Log, l => l.Contains("identity.unknown"));
            h.Deliver(Flatten(F1));
            Assert.Contains(h.Reports(F1), r => r.Status == ReportStatus.Filled);
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Theory]
        [InlineData(AccountKind.Simulation)]
        [InlineData(AccountKind.Playback)]
        public void A_known_non_live_kind_invalidates_the_confirmation_even_back_to_live(AccountKind via)
        {
            // QA M1: Live → Simulation/Playback → Live must not come back as state B.
            using var h = LiveHarness();
            h.Platform.SetKind(Sim101, via);
            h.Settle();
            h.Platform.SetKind(Sim101, AccountKind.Live);
            h.Settle();
            Assert.Empty(h.Consent.Confirmed);
            h.Deliver(Place(E1, Side.Buy, 1));
            AssertRefused(h, E1);
        }

        [Fact]
        public void Kind_change_while_unmapped_and_disconnected_still_invalidates()
        {
            var welcome = DefaultWelcome();
            welcome.SimulationOnly = false;
            welcome.Accounts.Clear(); // the confirmed account is not mapped in this session
            using var h = LiveHarness();
            h.Disconnect();
            h.Connect(welcome);
            h.Platform.SetConnected(Sim101, false);
            h.Settle();
            h.Platform.SetKind(Sim101, AccountKind.Simulation);
            h.Settle();
            Assert.Empty(h.Consent.Confirmed);
        }

        [Theory]
        [InlineData(ProtectionMode.Shared)]
        [InlineData(ProtectionMode.Individual)]
        public void A_consented_add_filled_in_the_cancel_race_on_A_is_protected(ProtectionMode mode)
        {
            // QA M2: in shared mode the owner's legs protect the add, and that growth is row 6 (Y on A), not row 7.
            using var h = LiveHarness();
            var protection = mode == ProtectionMode.Shared ? Shared() : Individual();
            h.Deliver(Place(E1, Side.Buy, 1, protection: protection));
            h.Deliver(Place(E2, Side.Buy, 1, protection: protection, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20995m }));
            h.Platform.DeferCancels = true;
            Enter(h, LiveState.A); // the plugin cancels E2 (row 2); the cancel is pending
            h.Platform.FillOrder(Sim101, h.Platform.OrderByName(Sim101, "aw:" + E2 + ":e")!.Handle, 1, 20995m);
            h.Settle();
            Assert.Equal(2, h.Platform.Position(Sim101, "MNQ"));
            var protectedQuantity = mode == ProtectionMode.Shared
                ? h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity
                : h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity + h.Platform.OrderByName(Sim101, "aw:" + E2 + ":s")!.Quantity;
            Assert.Equal(2, protectedQuantity);
        }

        [Fact]
        public void Shared_row_6_cap_ignores_entries_of_earlier_closed_positions()
        {
            // QA LN1: an earlier consented shared position must not raise the cap of the current one.
            using var h = LiveHarness();
            h.Deliver(Place(F1, Side.Buy, 2, protection: Shared()));
            h.Deliver(Flatten("0192b3c4-0000-7000-8000-000000000d21"));
            h.Deliver(Place(E1, Side.Buy, 1, protection: Shared()));
            h.Deliver(Place(E2, Side.Buy, 1, protection: Shared(), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20995m }));
            h.Platform.DeferCancels = true;
            Enter(h, LiveState.A);
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m); // a manual add: row 7, refused on A
            h.Settle();
            h.Platform.FillOrder(Sim101, h.Platform.OrderByName(Sim101, "aw:" + E2 + ":e")!.Handle, 1, 20995m);
            h.Settle();
            Assert.Equal(3, h.Platform.Position(Sim101, "MNQ"));
            Assert.Equal(2, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity); // E1 + E2, not the manual add
        }

        [Fact]
        public void Shared_row_6_counts_by_fill_an_add_received_before_the_owner_but_filled_after()
        {
            // QA M10 (probe Q8): E2 rests before E1 opens the position as the shared owner; E2 fills in the cancel race on A.
            using var h = LiveHarness();
            h.Deliver(Place(E2, Side.Buy, 1, protection: Shared(), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20995m }));
            h.Deliver(Place(E1, Side.Buy, 1, protection: Shared()));
            Assert.Equal(1, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity);
            h.Platform.DeferCancels = true;
            Enter(h, LiveState.A);
            h.Platform.FillOrder(Sim101, h.Platform.OrderByName(Sim101, "aw:" + E2 + ":e")!.Handle, 1, 20995m);
            h.Settle();
            Assert.Equal(2, h.Platform.Position(Sim101, "MNQ"));
            Assert.Equal(2, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity);
        }

        [Fact]
        public void Position_numbers_survive_a_restart()
        {
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1, protection: Shared()));
            h.Deliver(Place(E2, Side.Buy, 1, protection: Shared(), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20995m }));
            h.AckAll();
            h.Restart();
            h.Connect(new Func<WelcomePayload>(() => { var w = DefaultWelcome(); w.SimulationOnly = false; return w; })());
            h.Platform.DeferCancels = true;
            Enter(h, LiveState.A);
            h.Platform.FillOrder(Sim101, h.Platform.OrderByName(Sim101, "aw:" + E2 + ":e")!.Handle, 1, 20995m);
            h.Settle();
            Assert.Equal(2, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity);
        }

        [Fact]
        public void Fills_of_a_position_open_past_retention_are_kept_for_the_row_6_cap()
        {
            // Code Review low: a shared position open for more than 7 days, whose executions NT8 no longer lists.
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1, protection: Shared()));
            h.Deliver(Place(E2, Side.Buy, 1, protection: Shared(), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20995m }));
            h.AckAll();
            h.Platform.ForgetExecutions(Sim101);
            h.Tick(TimeSpan.FromDays(8)); // pruning runs
            h.Platform.DeferCancels = true;
            Enter(h, LiveState.A);
            h.Platform.FillOrder(Sim101, h.Platform.OrderByName(Sim101, "aw:" + E2 + ":e")!.Handle, 1, 20995m);
            h.Settle();
            Assert.Equal(2, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity);
        }

        [Fact]
        public void Shared_growth_from_a_non_consented_fill_on_A_is_still_refused()
        {
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1, protection: Shared()));
            Enter(h, LiveState.A);
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m); // row 7 on A: N
            h.Settle();
            Assert.Equal(1, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity);
        }

        [Theory]
        [InlineData(LiveState.A, LiveOffPolicy.CancelEntriesKeepExits, false)]
        [InlineData(LiveState.C1, LiveOffPolicy.RefuseAll, false)]      // behaves as A
        [InlineData(LiveState.C1, LiveOffPolicy.CancelEntriesKeepExits, true)] // the table's C1 column applies to the Core
        public void Core_shrink_of_a_leg_is_accepted_only_where_the_Core_may_write(LiveState state, LiveOffPolicy u19, bool applied)
        {
            // QA L1: on A the Core is accepted only for row 2 and rows 3/5; row 4 there is the plugin's own initiative.
            using var h = LiveHarness(u19);
            h.Deliver(Place(E1, Side.Buy, 2, protection: Individual()));
            Enter(h, state);
            h.Deliver(Modify(F1, E1, ChildRole.Stop, quantity: 1));
            Assert.Equal(applied ? 1 : 2, h.Platform.OrderByName(Sim101, "aw:" + E1 + ":s")!.Quantity);
            if (!applied) AssertRefused(h, F1);
        }

        [Fact]
        public void Actions_on_state_A_raise_the_window_alert()
        {
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            Enter(h, LiveState.A);
            Assert.Contains(AdapterEngine.NoticeUnconsentedActions, View(h).Notices);
        }

        [Fact]
        public void Off_never_raises_the_unconsented_alert_under_U19_a()
        {
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            Enter(h, LiveState.C0);
            Assert.DoesNotContain(AdapterEngine.NoticeUnconsentedActions, View(h).Notices);
        }

        [Fact]
        public void Flatten_that_fails_after_cancelling_alerts_an_unprotected_position()
        {
            using var h = LiveHarness();
            h.Deliver(Place(E1, Side.Buy, 1, protection: Individual(40, 80)));
            h.Platform.RejectNextOrder = true;
            h.Deliver(Flatten(F1));
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
            Assert.Contains(AdapterEngine.NoticeUnprotectedPosition, View(h).Notices);
            Assert.Contains(h.Log, l => l.StartsWith("Error live.unprotected_position", StringComparison.Ordinal));
        }

        [Fact]
        public void Simulation_accounts_are_unaffected_in_release_builds()
        {
            using var h = new EngineHarness(options: new EngineOptions { Build = AdapterBuild.Release });
            h.Platform.SetPrice("MNQ", 21000m);
            h.Connect();
            Assert.Null(View(h).LiveState);
            h.Deliver(Place(E1, Side.Buy, 1));
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
        }

        // ------------------------------------------------------------------ the opt-in document (fail closed)

        [Fact]
        public void Consent_document_round_trips()
        {
            var consent = LiveConsentState.Off();
            consent.Allow(new[] { new LiveAccountKey(Connection, Provider, "APEX-1"), new LiveAccountKey(Connection, Provider, "APEX-2") }, OptInAt);
            var parsed = LiveConsentState.Parse(consent.Serialize());
            Assert.True(parsed.Enabled);
            Assert.Equal(OptInAt, parsed.EnabledAt);
            Assert.True(parsed.IsConfirmed(new LiveAccountKey(Connection, Provider, "APEX-2")));
            Assert.False(parsed.IsConfirmed(new LiveAccountKey("Other", Provider, "APEX-2")));
            Assert.Equal(consent.Serialize(), parsed.Serialize()); // canonical form (Vigia I1)
            consent.Block();
            var blocked = LiveConsentState.Parse(consent.Serialize());
            Assert.False(blocked.Enabled);
            Assert.Equal(2, blocked.Confirmed.Count); // kept, so the C states apply after "off"
        }

        [Theory]
        [InlineData(null)]
        [InlineData("")]
        [InlineData("not json")]
        [InlineData("[]")]
        [InlineData("{\"v\":2,\"enabled\":true,\"enabled_at\":\"2026-10-22T14:00:00.000Z\",\"accounts\":[]}")]
        [InlineData("{\"v\":1,\"enabled\":\"true\",\"enabled_at\":\"2026-10-22T14:00:00.000Z\",\"accounts\":[]}")]
        [InlineData("{\"v\":1,\"enabled\":true,\"accounts\":[]}")]
        [InlineData("{\"v\":1,\"enabled\":true,\"enabled_at\":\"yesterday\",\"accounts\":[]}")]
        [InlineData("{\"v\":1,\"enabled\":true,\"enabled_at\":\"2026-10-22T14:00:00.000Z\",\"accounts\":[],\"extra\":1}")]
        [InlineData("{\"v\":1,\"enabled\":true,\"enabled_at\":\"2026-10-22T14:00:00.000Z\",\"accounts\":[{\"connection\":\"c\",\"provider\":\"p\"}]}")]
        [InlineData("{\"v\":99999999999999999999,\"enabled\":true,\"enabled_at\":\"2026-10-22T14:00:00.000Z\",\"accounts\":[]}")] // Vigia L2
        [InlineData("{\"v\":1, \"enabled\":true,\"enabled_at\":\"2026-10-22T14:00:00.000Z\",\"accounts\":[]}")] // not canonical (I1)
        [InlineData("{\"enabled\":true,\"v\":1,\"enabled_at\":\"2026-10-22T14:00:00.000Z\",\"accounts\":[]}")]
        [InlineData("{\"v\":1,\"enabled\":true,\"enabled_at\":\"2026-10-22T14:00:00Z\",\"accounts\":[]}")]
        [InlineData("{\"v\":1,\"enabled\":true,\"enabled\":true,\"enabled_at\":\"2026-10-22T14:00:00.000Z\",\"accounts\":[]}")]
        public void Anything_but_a_valid_document_is_off_with_nothing_confirmed(string? text)
        {
            var parsed = LiveConsentState.Parse(text);
            Assert.False(parsed.Enabled);
            Assert.Empty(parsed.Confirmed);
        }

        [Theory]
        [InlineData("a", LiveOffPolicy.CancelEntriesKeepExits, ProtectiveCancelPolicy.AllowAfterOff)]
        [InlineData("b", LiveOffPolicy.RefuseAll, ProtectiveCancelPolicy.Refuse)]
        [InlineData(" B ", LiveOffPolicy.RefuseAll, ProtectiveCancelPolicy.Refuse)]
        [InlineData(null, LiveOffPolicy.CancelEntriesKeepExits, ProtectiveCancelPolicy.Refuse)]
        [InlineData("", LiveOffPolicy.CancelEntriesKeepExits, ProtectiveCancelPolicy.Refuse)]
        [InlineData("c", LiveOffPolicy.CancelEntriesKeepExits, ProtectiveCancelPolicy.Refuse)]
        public void Policies_missing_or_invalid_are_the_recommended_options(string? value, LiveOffPolicy u19, ProtectiveCancelPolicy u191)
        {
            Assert.Equal(u19, LiveAccountTable.ParseOffPolicy(value));
            Assert.Equal(u191, LiveAccountTable.ParseProtectiveCancelPolicy(value));
        }

        [Fact]
        public void Engine_defaults_are_the_recommended_options()
        {
            var options = new EngineOptions();
            Assert.Equal(LiveOffPolicy.CancelEntriesKeepExits, options.LiveOffPolicy);
            Assert.Equal(ProtectiveCancelPolicy.Refuse, options.ProtectiveCancelPolicy);
        }
    }
}
