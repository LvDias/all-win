using System;
using System.Collections.Generic;
using System.Linq;
using AllWin.Adapter.Core.Engine;
using AllWin.Adapter.Core.Platform;
using AllWin.Adapter.Core.Protocol;
using Xunit;
using static AllWin.Adapter.Core.Tests.Engine.EngineHarness;

namespace AllWin.Adapter.Core.Tests.Engine
{
    /// <summary>
    /// Regressions for QA [Plugins] (Pêndulo, V1-S2 @ ada3a21) and Code Review 6 (Bisturi), with the
    /// Architect's answers A–G. Each test names the finding it closes.
    /// </summary>
    public class ReviewFixTests
    {
        private const string C1 = "0192b3c4-0000-7000-8000-000000000701";
        private const string C2 = "0192b3c4-0000-7000-8000-000000000702";
        private const string C3 = "0192b3c4-0000-7000-8000-000000000703";
        private const string C4 = "0192b3c4-0000-7000-8000-000000000704";

        private static EngineHarness Ready(bool oco = true, decimal price = 21034.50m, Action<Sim.SimulatedPlatform>? setup = null, WelcomePayload? welcome = null)
        {
            var h = new EngineHarness(setup);
            h.Platform.OcoEnforced = oco;
            h.Platform.SetPrice("MNQ", price);
            h.Connect(welcome);
            return h;
        }

        private static PlatformOrder Leg(EngineHarness h, string commandId, char role) => h.Platform.OrderByName(Sim101, "aw:" + commandId + ":" + role)!;

        // ------------------------------------------------------------------ QA H1: protection never exceeds the position

        [Fact]
        public void QA_H1_partial_reduce_resizes_individual_protection_and_a_stop_cannot_flip()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            h.Deliver(Place(C2, Side.Sell, 1, PositionEffect.Reduce));
            Assert.Equal(1, Leg(h, C1, 's').Quantity);
            Assert.Equal(1, Leg(h, C1, 't').Quantity);
            h.Platform.SetPrice("MNQ", 21024.50m);
            h.Settle();
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void QA_H1_partial_reduce_resizes_the_shared_bracket()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 2, protection: Shared(8, 16)));
            h.Deliver(Place(C2, Side.Sell, 1, PositionEffect.Reduce));
            Assert.Equal(1, Leg(h, C1, 's').Quantity);
            h.Platform.SetPrice("MNQ", 20998m);
            h.Settle();
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void QA_H1_manual_partial_close_in_the_platform_resizes_too()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 21034.50m);
            h.Settle();
            Assert.Equal(1, Leg(h, C1, 's').Quantity);
            h.Platform.SetPrice("MNQ", 21024.50m);
            h.Settle();
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void QA_H1_partial_exit_reduces_the_oldest_entry_first()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80)));
            h.Deliver(Place(C2, Side.Buy, 1, protection: Individual(40, 80), expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            h.Deliver(Place(C3, Side.Sell, 1, PositionEffect.Reduce));
            Assert.All(h.Reports(C1).Where(r => r.Status == ReportStatus.Cancelled), r => Assert.Equal(ReasonCodes.OrderOrphanCancelled, r.ReasonCode));
            Assert.True(Leg(h, C1, 's').IsTerminal && Leg(h, C1, 't').IsTerminal);
            Assert.False(Leg(h, C2, 's').IsTerminal);
            Assert.Equal(1, Leg(h, C2, 's').Quantity);
        }

        [Fact]
        public void QA_H1_legs_left_on_the_wrong_side_after_a_flip_are_cancelled()
        {
            using var h = Ready(oco: false);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80)));
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 2, 21034.50m); // flips to short 1
            h.Settle();
            Assert.True(Leg(h, C1, 's').IsTerminal && Leg(h, C1, 't').IsTerminal);
            Assert.Equal(2, h.Reports(C1).Count(r => r.Status == ReportStatus.Cancelled && r.ReasonCode == ReasonCodes.OrderOrphanCancelled));
            Assert.Equal(-1, h.Platform.Position(Sim101, "MNQ")); // nothing added to the unintended position
        }

        // ------------------------------------------------------------------ QA H2 / CR6 H1: flatten never wedges the queue

        [Fact]
        public void QA_H2_flatten_whose_close_died_in_flight_is_unknown_and_the_queue_advances()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Platform.CrashNextSubmit = true;
            h.Deliver(Flatten(C2));
            Assert.Equal(ReportStatus.Unknown, h.Reports(C2).Last().Status);
            Assert.Single(h.Platform.CallLog, c => c.Contains(C2 + ":x")); // never submitted again
            h.Deliver(Place(C3, Side.Sell, 1, PositionEffect.Reduce));
            Assert.Contains(h.Reports(C3), r => r.Status == ReportStatus.Filled);
        }

        [Theory]
        [InlineData(false, ReportStatus.Unknown)]
        [InlineData(true, ReportStatus.Cancelled)]
        public void CR6_H1_flatten_closing_at_restart_without_its_close_order_never_resends_and_frees_the_queue(bool flatMeanwhile, ReportStatus expected)
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Platform.ClearPrice("MNQ"); // the close order rests
            h.Deliver(Flatten(C2));
            Assert.NotNull(h.Platform.OrderByName(Sim101, "aw:" + C2 + ":x"));
            // The process died inside Submit: the platform never got the close order.
            h.Platform.ForgetOrder(Sim101, "aw:" + C2 + ":x");
            if (flatMeanwhile) h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 21000m);
            h.LoseEvents();
            h.Restart();
            h.Connect();
            Assert.Equal(expected, h.Reports(C2).Last().Status);
            Assert.Single(h.Platform.CallLog, c => c.Contains(C2 + ":x"));
            h.Platform.SetPrice("MNQ", 21000m);
            h.Deliver(Place(C3, Side.Buy, 1, expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            Assert.Contains(h.Reports(C3), r => r.Status == ReportStatus.Filled);
        }

        [Fact]
        public void CR6_H1_flatten_cancelling_at_restart_resumes_and_finishes_already_flat()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Platform.DeferCancels = true;
            h.Deliver(Flatten(C2));
            Assert.DoesNotContain(h.Reports(C2), r => r.Status == ReportStatus.Cancelled);
            h.Platform.ConfirmCancels();
            h.LoseEvents(); // NT8 dies before the confirmation reaches the plugin
            h.Restart();
            h.Connect();
            var final = h.Reports(C2).Last();
            Assert.Equal(ReportStatus.Cancelled, final.Status);
            Assert.Equal(ReasonCodes.CommandAlreadyFlat, final.ReasonCode);
        }

        [Fact]
        public void Flatten_close_order_that_never_completes_releases_the_queue_after_the_timeout()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Platform.ClearPrice("MNQ");
            h.Deliver(Flatten(C2));
            h.Deliver(Place(C3, Side.Sell, 1, PositionEffect.Reduce));
            Assert.DoesNotContain(h.Platform.CallLog, c => c.Contains(C3));
            for (var i = 0; i < 7; i++)
            {
                h.Deliver(new HeartbeatPayload()); // the Core stays alive meanwhile
                h.Tick(TimeSpan.FromSeconds(9));
            }
            Assert.Equal(ReportStatus.Unknown, h.Reports(C2).Last().Status);
            Assert.Contains(h.Platform.CallLog, c => c.Contains(C3)); // the queue moved on
        }

        // ------------------------------------------------------------------ QA H3: mapping re-checked at dispatch

        [Fact]
        public void QA_H3_queued_command_is_rejected_when_the_new_welcome_makes_the_account_master_only()
        {
            using var h = Ready();
            h.Disconnect();
            h.Engine.OnConnected(h.Sink);
            h.Deliver(Place(C1, Side.Buy, 1));
            var welcome = DefaultWelcome();
            welcome.Accounts[0].Roles = new List<AccountRole> { AccountRole.Master };
            h.Deliver(welcome);
            Assert.Equal(ReasonCodes.CommandAccountNotDestination, h.Reports(C1).Last().ReasonCode);
            Assert.Empty(h.Platform.CallLog);
        }

        [Fact]
        public void QA_H3_queued_command_is_rejected_when_the_new_welcome_unmaps_the_account()
        {
            using var h = Ready();
            h.Disconnect();
            h.Engine.OnConnected(h.Sink);
            h.Deliver(Place(C1, Side.Buy, 1));
            var welcome = DefaultWelcome();
            welcome.Accounts.Clear();
            h.Deliver(welcome);
            var report = h.Reports(C1).Last();
            Assert.Equal(ReasonCodes.CommandAccountNotMapped, report.ReasonCode);
            Assert.Equal(AccountId, report.AccountId);
            Assert.Empty(h.Platform.CallLog);
        }

        // ------------------------------------------------------------------ QA M1 / CR6 M1: unexpected host exceptions

        [Fact]
        public void QA_M1_place_whose_submit_died_is_unknown_never_resubmitted_and_the_queue_continues()
        {
            using var h = Ready();
            h.Platform.CrashNextSubmit = true;
            h.Deliver(Place(C1, Side.Buy, 1));
            Assert.Equal(ReportStatus.Unknown, h.Reports(C1).Last().Status);
            h.Deliver(Flatten(C2));
            Assert.Equal(ReasonCodes.CommandAlreadyFlat, h.Reports(C2).Last().ReasonCode);
            h.Deliver(Place(C1, Side.Buy, 1));
            Assert.Equal(ReportStatus.Duplicate, h.Reports(C1).Last().Status);
            Assert.Single(h.Platform.CallLog, c => c.Contains(C1 + ":e"));
        }

        [Fact]
        public void Store_failure_mid_dispatch_fails_closed_before_any_platform_call()
        {
            using var h = Ready();
            h.FailNextCommit(); // the command record cannot be made durable
            h.Deliver(Place(C1, Side.Buy, 1));
            Assert.Equal(SessionState.Stopped, h.Engine.Session);
            Assert.Equal(StopReason.StoreFailure, h.Engine.StopReason);
            Assert.Empty(h.Platform.CallLog);
            Assert.Empty(h.Acks()); // never acked: the Core will re-deliver
        }

        // ------------------------------------------------------------------ QA Q1 = answer A: catch-up

        [Fact]
        public void A_entry_filled_while_down_gets_no_late_bracket_and_is_flagged_incomplete()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20990m }));
            h.AckAll();
            h.Restart();                    // the plugin is down from here: events are lost
            h.Platform.SetPrice("MNQ", 20990m);
            h.Restart();
            Assert.Null(h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s"));
            Assert.Contains(ReasonCodes.SyncBracketIncomplete, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).LocalSyncReasons);
            h.Connect();
            Assert.Contains(h.Reports(C1), r => r.Status == ReportStatus.Filled); // the fill is still reported
            var withheld = h.Reports(C1).Where(r => r.ReasonCode == ReasonCodes.OrderProtectionNotSubmitted).ToList();
            Assert.Equal(new[] { ChildRole.Stop, ChildRole.Target }, withheld.Select(r => r.ChildRole!.Value));
            Assert.All(withheld, r => Assert.Equal(ReportStatus.Rejected, r.Status));
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 20990m); // a later live fill does not bring it back
            h.Settle();
            Assert.Null(h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s"));
            Assert.Empty(h.Snapshots().Last().Accounts.Single().WorkingOrders);
        }

        [Fact]
        public void A_catch_up_resizes_existing_legs_down_but_never_up_or_reprices()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            var stopPrice = Leg(h, C1, 's').StopPrice;
            h.AckAll();
            h.Restart();
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 21000m); // partial close while down
            h.Restart();
            Assert.Equal(1, Leg(h, C1, 's').Quantity);
            Assert.Equal(stopPrice, Leg(h, C1, 's').StopPrice);

            h.Restart();
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 3, 21000m); // the position grew while down
            h.Restart();
            Assert.Equal(1, Leg(h, C1, 's').Quantity); // not extended on our own
            Assert.Contains(ReasonCodes.SyncBracketIncomplete, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).LocalSyncReasons);
        }

        // ------------------------------------------------------------------ CR6 H2: pruning keeps live brackets

        [Fact]
        public void CR6_H2_shared_owner_outlives_retention_while_its_bracket_lives()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Shared(8, 16)));
            h.AckAll();
            h.Tick(TimeSpan.FromDays(8)); // pruning runs
            h.Disconnect();
            h.Connect();
            h.Deliver(Place(C3, Side.Buy, 1, protection: Shared(8, 16), expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            Assert.Equal(2, Leg(h, C1, 's').Quantity);
            Assert.Null(h.Platform.OrderByName(Sim101, "aw:" + C3 + ":s"));
            h.Platform.SetPrice("MNQ", 20998m);
            h.Settle();
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }

        [Fact]
        public void CR6_H2_individual_bracket_outlives_retention_and_still_resizes()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            h.AckAll();
            h.Tick(TimeSpan.FromDays(8));
            h.Disconnect();
            h.Connect();
            h.Deliver(Place(C2, Side.Sell, 1, PositionEffect.Reduce));
            Assert.Equal(1, Leg(h, C1, 's').Quantity);
        }

        [Theory]
        [InlineData(false)] // account not visible yet (NT8 started before the connection)
        [InlineData(true)]  // account visible but its connection is down
        public void CR6_recheck_H3_startup_prune_keeps_brackets_of_accounts_it_cannot_observe(bool visibleButDisconnected)
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            h.AckAll();
            h.Clock.Advance(TimeSpan.FromDays(8));
            h.Restart();
            if (visibleButDisconnected) h.Platform.SetConnected(Sim101, false);
            else h.Platform.SetVisible(Sim101, false);
            h.LoseEvents();
            h.Restart(renewHandles: false); // Start prunes while the account cannot be observed
            h.Platform.SetVisible(Sim101, true);
            h.Platform.SetConnected(Sim101, true);
            h.Settle();
            h.Connect();
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 21034.50m); // manual partial close
            h.Settle();
            Assert.Equal(1, Leg(h, C1, 's').Quantity); // invariant B still holds: the record survived
            Assert.Equal(1, Leg(h, C1, 't').Quantity);
            Assert.DoesNotContain(h.Reports(C1), r => r.Fill != null && r.ChildRole == ChildRole.Entry); // fill records kept: no replay
        }

        [Fact]
        public void Fill_records_are_kept_while_the_platform_still_holds_the_execution()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.AckAll();
            h.Tick(TimeSpan.FromDays(8));
            h.Restart();
            h.Connect();
            Assert.DoesNotContain(h.Reports(), r => r.Fill != null);
        }

        // ------------------------------------------------------------------ answers D, E, F, G and QA L-items

        [Fact]
        public void D_modify_quantity_above_the_position_is_capped_on_a_protective_leg()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80)));
            h.Deliver(Modify(C2, C1, ChildRole.Stop, quantity: 5));
            Assert.Equal(2, Leg(h, C1, 's').Quantity);
            var outcome = h.Reports(C2).Single(r => r.Status == ReportStatus.Working);
            Assert.Equal(ReasonCodes.OrderQuantityCapped, outcome.ReasonCode);
            Assert.Equal(2, outcome.Quantity);
        }

        [Fact]
        public void F_account_state_updates_the_view_on_a_1_1_session_only()
        {
            using var h = new EngineHarness();
            var welcome = DefaultWelcome();
            welcome.ProtocolVersion = "1.1";
            h.Connect(welcome);
            var state = new AccountStatePayload { AccountId = AccountId, PlatformAccount = Sim101, SyncState = SyncState.OutOfSync, SyncReasons = { "sync.disconnected_in_trade" }, ChangedAt = h.Clock.UtcNow };
            h.Engine.OnFrame(ProtocolCodec.Encode(state, Ids.New(), h.Clock.UtcNow, "1.1"));
            Assert.Equal(SyncState.OutOfSync, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).CoreSyncState);
            // Frames created in the 1.1 session carry 1.1.
            Assert.Equal("1.1", h.Sent(MessageTypes.PositionSnapshot).Last().ProtocolVersion);

            var cleared = new AccountStatePayload { AccountId = AccountId, PlatformAccount = Sim101, SyncState = SyncState.InSync, ChangedAt = h.Clock.UtcNow };
            h.Engine.OnFrame(ProtocolCodec.Encode(cleared, Ids.New(), h.Clock.UtcNow, "1.1"));
            Assert.Equal(SyncState.InSync, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).CoreSyncState);
        }

        [Fact]
        public void F_outbox_frames_keep_their_original_version_bytes_in_a_1_1_session()
        {
            using var h = Ready();
            var before = h.Sent(MessageTypes.PositionSnapshot).Single();
            h.Disconnect();
            var welcome = DefaultWelcome();
            welcome.ProtocolVersion = "1.1";
            h.Connect(welcome);
            var resent = h.Sent(MessageTypes.PositionSnapshot).First(f => f.MsgId == before.MsgId);
            Assert.Equal("1.0", resent.ProtocolVersion);
        }

        [Fact]
        public void G_an_order_already_on_the_platform_with_the_exact_reference_is_adopted_not_resubmitted()
        {
            using var h = Ready(price: 21000m);
            h.Platform.Submit(Sim101, new[] { new OrderRequest { Name = "aw:" + C1 + ":e", Instrument = Mnq, Side = Side.Buy, Type = OrderType.Limit, Quantity = 1, LimitPrice = 20000m } });
            h.Platform.CallLog.Clear();
            h.Settle();
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal));
            Assert.Contains(h.Reports(C1), r => r.Status == ReportStatus.Working && r.ChildRole == ChildRole.Entry);
            Assert.Contains(h.Log, l => l.Contains("order.adopted"));
        }

        [Fact]
        public void G_two_live_orders_with_the_same_reference_are_ambiguous_and_nothing_is_submitted()
        {
            using var h = Ready(price: 21000m);
            for (var i = 0; i < 2; i++)
                h.Platform.Submit(Sim101, new[] { new OrderRequest { Name = "aw:" + C1 + ":e", Instrument = Mnq, Side = Side.Buy, Type = OrderType.Limit, Quantity = 1, LimitPrice = 20000m } });
            h.Platform.CallLog.Clear();
            h.Settle();
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            var report = h.Reports(C1).Last();
            Assert.Equal(ReportStatus.Rejected, report.Status);
            Assert.Equal(ReasonCodes.OrderAmbiguousReference, report.ReasonCode);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal));
        }

        [Fact]
        public void G_a_terminal_order_with_the_reference_counts_as_submitted_for_a_place()
        {
            using var h = Ready(price: 21000m);
            h.Platform.Submit(Sim101, new[] { new OrderRequest { Name = "aw:" + C1 + ":e", Instrument = Mnq, Side = Side.Buy, Type = OrderType.Limit, Quantity = 1, LimitPrice = 20000m } });
            h.Platform.Cancel(Sim101, new[] { h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e")!.Handle });
            h.Platform.CallLog.Clear();
            h.Settle();
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal)); // never resubmitted
            Assert.Equal(ReportStatus.Cancelled, h.Reports(C1).Last().Status); // its last state is reported
        }

        [Fact]
        public void G_crash_window_submit_then_immediate_fill_then_death_is_adopted_with_no_second_order()
        {
            // ADR-0004 §10 (Crivo): the market order fills at once, the call dies before "submitted" is known.
            using var h = Ready(price: 21000m);
            h.Platform.CrashAfterSubmit = true;
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80)));
            Assert.Single(h.Platform.CallLog, c => c == "submit aw:" + C1 + ":e");
            Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
            Assert.Contains(h.Reports(C1), r => r.Status == ReportStatus.Filled && r.ChildRole == ChildRole.Entry);
            Assert.DoesNotContain(h.Reports(C1), r => r.Status == ReportStatus.Unknown);
            Assert.NotNull(h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s")); // the bracket follows the adopted fill
        }

        [Fact]
        public void G_terminal_and_live_matches_for_a_place_are_ambiguous()
        {
            using var h = Ready(price: 21000m);
            h.Platform.Submit(Sim101, new[] { new OrderRequest { Name = "aw:" + C1 + ":e", Instrument = Mnq, Side = Side.Buy, Type = OrderType.Market, Quantity = 1 } });
            h.Platform.Submit(Sim101, new[] { new OrderRequest { Name = "aw:" + C1 + ":e", Instrument = Mnq, Side = Side.Buy, Type = OrderType.Limit, Quantity = 1, LimitPrice = 20000m } });
            h.Platform.CallLog.Clear();
            h.Settle();
            h.Deliver(Place(C1, Side.Buy, 1));
            Assert.Equal(ReasonCodes.OrderAmbiguousReference, h.Reports(C1).Last().ReasonCode);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal));
        }

        [Fact]
        public void G_flatten_retry_reusing_the_close_reference_still_closes_the_remaining_position()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Platform.ClearPrice("MNQ");
            h.Deliver(Flatten(C2));                                   // close order for 1 rests
            h.Platform.ExternalFill(Sim101, Mnq, Side.Buy, 1, 21000m); // the position grows to 2 meanwhile
            h.Settle();
            h.Platform.SetPrice("MNQ", 21000m);                        // the close fills 1: position 1, close order terminal
            h.Settle();
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));       // the re-try re-used aw:<cmd>:x and closed the rest
            Assert.Equal(2, h.Platform.CallLog.Count(c => c == "submit aw:" + C2 + ":x"));
            Assert.Contains(h.Reports(C2), r => r.Status == ReportStatus.Filled && r.PositionAfter!.Quantity == 0);
        }

        [Fact]
        public void G_order_created_by_a_submit_that_died_is_adopted_after_the_crash()
        {
            using var h = Ready(price: 21000m);
            h.Platform.CrashAfterSubmit = true;
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            Assert.Contains(h.Log, l => l.Contains("order.adopted"));
            Assert.Contains(h.Reports(C1), r => r.Status == ReportStatus.Working && r.ChildRole == ChildRole.Entry);
            Assert.DoesNotContain(h.Reports(C1), r => r.Status == ReportStatus.Unknown);
            Assert.Single(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal));
        }

        [Fact]
        public void QA_L2_blocked_until_is_compared_on_the_core_clock()
        {
            using var h = new EngineHarness();
            h.Platform.SetPrice("MNQ", 21000m);
            h.Connect(coreClockAhead: TimeSpan.FromHours(2));
            h.Deliver(new LimitsPayload
            {
                Accounts =
                {
                    new AccountLimits
                    {
                        AccountId = AccountId, PlatformAccount = Sim101, Version = 1, MaxPosition = 5, WarningThresholdPct = 80,
                        TradingDay = new TradingDaySpec { ResetLocalTime = "17:00", TimeZone = "America/Chicago" },
                        BlockedUntil = h.Clock.UtcNow.AddHours(1), // already past on the Core clock
                    },
                },
            });
            h.Deliver(Place(C1, Side.Buy, 1, expiresAt: h.Engine.CoreNow.AddSeconds(5)));
            Assert.Contains(h.Reports(C1), r => r.Status == ReportStatus.Filled);
        }

        [Fact]
        public void QA_L3_command_before_the_first_ever_welcome_is_queued_not_rejected()
        {
            using var h = new EngineHarness();
            h.Platform.SetPrice("MNQ", 21000m);
            h.Engine.OnConnected(h.Sink);
            h.Deliver(Place(C1, Side.Buy, 1));
            Assert.DoesNotContain(h.Reports(C1), r => r.Status == ReportStatus.Rejected);
            h.Deliver(DefaultWelcome());
            Assert.Contains(h.Reports(C1), r => r.Status == ReportStatus.Filled);
        }

        // ------------------------------------------------------------------ QA R1 / Architect R1: only live fills get a bracket

        [Fact]
        public void R1_b_entry_filled_after_a_broker_reconnect_gets_no_bracket()
        {
            using var h = Ready(price: 21034.50m);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 21000m }));
            h.Platform.SetConnected(Sim101, false);
            h.Platform.SetConnected(Sim101, true);
            h.Settle();
            h.Platform.SetPrice("MNQ", 20990m); // the execution reaches us after the reconnect
            h.Settle();
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit aw:" + C1 + ":s", StringComparison.Ordinal));
            var withheld = h.Reports(C1).Where(r => r.ReasonCode == ReasonCodes.OrderProtectionNotSubmitted).Select(r => r.ChildRole).ToList();
            Assert.Equal(new ChildRole?[] { ChildRole.Stop, ChildRole.Target }, withheld);
        }

        [Fact]
        public void R1_a_fill_received_while_the_connection_is_down_gets_no_bracket()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Platform.SetConnected(Sim101, false);
            h.Settle();
            h.Platform.FillOrder(Sim101, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e")!.Handle, 1, 20000m);
            h.Settle();
            Assert.Contains(h.Reports(C1), r => r.ReasonCode == ReasonCodes.OrderProtectionNotSubmitted);
            Assert.Null(h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s"));
        }

        [Theory]
        [InlineData(5, true)]
        [InlineData(11, false)]
        public void R1_c_fill_older_than_max_bracket_delay_gets_no_bracket(int ageSeconds, bool bracketPlaced)
        {
            using var h = Ready(price: 21000m);
            h.Platform.ExecutionDelay = TimeSpan.FromSeconds(ageSeconds);
            h.Deliver(Place(C1, Side.Buy, 1, protection: Individual(40, 80)));
            Assert.Equal(bracketPlaced, h.Platform.OrderByName(Sim101, "aw:" + C1 + ":s") != null);
            Assert.Equal(!bracketPlaced, h.Reports(C1).Any(r => r.ReasonCode == ReasonCodes.OrderProtectionNotSubmitted));
        }

        [Fact]
        public void R1_existing_bracket_is_not_extended_by_a_late_fill_but_still_shrinks()
        {
            using var h = Ready(price: 20000m);
            h.Deliver(Place(C1, Side.Buy, 2, protection: Individual(40, 80), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 19990m }));
            var entry = h.Platform.OrderByName(Sim101, "aw:" + C1 + ":e")!;
            h.Platform.FillOrder(Sim101, entry.Handle, 1, 19990m);
            h.Settle();
            Assert.Equal(1, Leg(h, C1, 's').Quantity);
            h.Platform.ExecutionDelay = TimeSpan.FromSeconds(30);
            h.Platform.FillOrder(Sim101, entry.Handle, 1, 19990m); // second part delivered late
            h.Settle();
            Assert.Equal(1, Leg(h, C1, 's').Quantity); // not extended
            Assert.Contains(ReasonCodes.SyncBracketIncomplete, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).LocalSyncReasons);
            h.Platform.ExecutionDelay = TimeSpan.Zero;
            h.Platform.ExternalFill(Sim101, Mnq, Side.Sell, 1, 20000m); // a live partial close still shrinks protection
            h.Settle();
            Assert.Equal(1, Leg(h, C1, 's').Quantity);
        }

        // ------------------------------------------------------------------ L3 option 2, rollback, L4, deferred flatten

        [Fact]
        public void L3_unconfirmed_last_write_is_dropped_and_shown()
        {
            using var h = Ready();
            h.Deliver(Place(C1, Side.Buy, 1));
            h.CorruptLastJournalLineOnNextRestart();
            h.Restart();
            Assert.True(h.Engine.StoreRecoveredTornWrite);
            Assert.Contains(h.Log, l => l.Contains("store.torn_tail_dropped"));
            Assert.Equal(SessionState.Offline, h.Engine.Session); // still runs
        }

        [Fact]
        public void Rollback_account_state_on_a_1_0_session_is_refused_with_unsupported_value()
        {
            using var h = Ready();
            var msgId = Ids.New();
            var state = new AccountStatePayload { AccountId = AccountId, PlatformAccount = Sim101, SyncState = SyncState.OutOfSync, SyncReasons = { "sync.x_y" }, ChangedAt = h.Clock.UtcNow };
            h.DeliverRaw(ProtocolCodec.Encode(state, msgId, h.Clock.UtcNow, "1.1"));
            var error = h.Sent(MessageTypes.Error).Single().PayloadAs<ErrorPayload>();
            Assert.Equal(ReasonCodes.ProtocolUnsupportedValue, error.Code);
            Assert.Equal(msgId, error.RefMsgId);
            Assert.Equal(SyncState.InSync, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).CoreSyncState);
        }

        [Fact]
        public void L4_a_failed_order_lookup_never_lets_a_submission_through()
        {
            using var h = Ready(price: 21000m);
            h.Platform.FailNextGetOrders = true;
            h.Deliver(Place(C1, Side.Buy, 1));
            Assert.DoesNotContain(h.Platform.CallLog, c => c.StartsWith("submit", StringComparison.Ordinal));
            var report = h.Reports(C1).Last();
            Assert.Equal(ReportStatus.Rejected, report.Status);
            Assert.Equal(ReasonCodes.CommandPlatformDisconnected, report.ReasonCode);
        }

        [Fact]
        public void Flatten_recovery_waits_for_the_account_to_be_observable()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Platform.ClearPrice("MNQ");
            h.Deliver(Flatten(C2));
            h.Platform.ForgetOrder(Sim101, "aw:" + C2 + ":x");
            h.LoseEvents();
            h.Restart();
            h.Platform.SetVisible(Sim101, false);
            h.Restart(renewHandles: false);
            Assert.NotEqual(SessionState.Stopped, h.Engine.Session); // no fault on an absent account
            h.Platform.SetVisible(Sim101, true);
            h.Platform.SetConnected(Sim101, true);
            h.Settle();
            h.Connect();
            Assert.Equal(ReportStatus.Unknown, h.Reports(C2).Last().Status);
        }

        // ------------------------------------------------------------------ missing tests listed by QA

        [Fact]
        public void Flatten_cancelling_across_a_core_reconnect_completes()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1));
            h.Deliver(Place(C2, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }, expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            h.Platform.DeferCancels = true;
            h.Deliver(Flatten(C3));
            h.Disconnect();
            h.Platform.ConfirmCancels();
            h.Settle(); // the flatten continues locally while the Core session is down
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
            h.Connect();
            Assert.Contains(h.Reports(C3), r => r.Status == ReportStatus.Filled && r.ChildRole == ChildRole.Flatten);
        }

        [Fact]
        public void Two_accounts_keep_their_own_fifo_and_do_not_block_each_other()
        {
            const string other = "0192b3c4-0000-7000-8000-0000000000b2";
            var welcome = DefaultWelcome();
            welcome.Accounts.Add(new WelcomeAccount { AccountId = other, PlatformAccount = "Sim102", Roles = { AccountRole.Destination }, SyncState = SyncState.InSync });
            using var h = Ready(price: 21000m, setup: p => p.AddAccount("Sim102"), welcome: welcome);
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Platform.DeferCancels = true;
            h.Deliver(Flatten(C2)); // Sim101 waits for its cancel confirmation
            var onOther = Place(C3, Side.Buy, 1);
            onOther.AccountId = other;
            onOther.PlatformAccount = "Sim102";
            h.Deliver(onOther);
            var later = Place(C4, Side.Buy, 1);
            h.Deliver(later); // queued behind the flatten on Sim101
            Assert.Contains(h.Reports(C3), r => r.Status == ReportStatus.Filled);
            Assert.DoesNotContain(h.Platform.CallLog, c => c.Contains(C4));
            h.Platform.ConfirmCancels();
            h.Settle();
            Assert.Contains(h.Reports(C4), r => r.Status == ReportStatus.Filled || r.Status == ReportStatus.Expired);
        }

        [Fact]
        public void Depends_on_survives_a_restart()
        {
            using var h = Ready(price: 21000m);
            h.Deliver(Place(C1, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
            h.Deliver(Flatten(C2, FlattenReason.Close)); // cancels C1's entry: flat → success
            h.Deliver(Place(C3, Side.Sell, 1, PositionEffect.Reduce, dependsOn: C2));
            h.Restart();
            h.Connect();
            Assert.Equal(ReasonCodes.CommandNothingToReduce, h.Reports(C3).Last().ReasonCode);
        }

        [Fact]
        public void Pump_is_not_reentrant_when_a_flatten_finishes_during_dispatch()
        {
            // flatten (already flat, finishes synchronously) followed by commands queued behind it while offline.
            using var h = Ready();
            h.Disconnect();
            h.Engine.OnConnected(h.Sink);
            h.Deliver(Flatten(C1, FlattenReason.Reverse));
            h.Deliver(Place(C2, Side.Sell, 1, dependsOn: C1, expiresAt: h.Clock.UtcNow.AddSeconds(5)));
            h.Deliver(Place(C3, Side.Buy, 1, PositionEffect.Reduce));
            h.Deliver(DefaultWelcome());
            Assert.Contains(h.Reports(C2), r => r.Status == ReportStatus.Filled);
            var c3 = h.Reports(C3).Last();
            Assert.True(c3.Status == ReportStatus.Filled, c3.Status + " " + c3.ReasonCode);
            Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
        }
    }
}
