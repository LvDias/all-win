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
        public void A_kind_change_invalidates_the_confirmation_until_it_is_confirmed_again()
        {
            using var h = LiveHarness();
            h.Platform.SetKind(Sim101, AccountKind.Unknown);
            h.Settle();
            Assert.Equal(LiveState.A, View(h).LiveState);
            Assert.Empty(h.Consent.Confirmed);
            Assert.Contains(h.Log, l => l.Contains("live.confirmation_invalidated"));
            h.Platform.SetKind(Sim101, AccountKind.Live); // back to live: still not confirmed
            h.Settle();
            Assert.Equal(LiveState.A, View(h).LiveState);
            h.Deliver(Place(E1, Side.Buy, 1));
            AssertRefused(h, E1);
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
