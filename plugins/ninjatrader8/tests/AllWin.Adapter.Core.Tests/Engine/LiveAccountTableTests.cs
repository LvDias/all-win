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
    /// ADR-0005 §N11 decision table: one EngineHarness test per Y/N cell, in each of the four U19 × U19.1 combinations.
    /// "Live" here is the kind the in-memory simulated platform reports; no broker or real account is involved.
    /// </summary>
    public class LiveAccountTableTests
    {
        internal const string Connection = "Rithmic Live";
        internal const string Provider = "Rithmic";
        internal static readonly DateTime OptInAt = new DateTime(2026, 10, 22, 14, 0, 0, DateTimeKind.Utc);

        private const string E1 = "0192b3c4-0000-7000-8000-000000000c01";
        private const string E2 = "0192b3c4-0000-7000-8000-000000000c02";
        private const string K1 = "0192b3c4-0000-7000-8000-000000000c11";

        // ------------------------------------------------------------------ the ADR table, transcribed (not the code under test)

        /// <summary>States marked Y in ADR-0005 §N11 with the recommended options (U19 a, U19.1 b).</summary>
        private static readonly Dictionary<LiveRow, LiveState[]> AdrYes = new Dictionary<LiveRow, LiveState[]>
        {
            [LiveRow.Entry] = new LiveState[0],
            [LiveRow.CancelEntry] = new[] { LiveState.A, LiveState.C0, LiveState.C1, LiveState.C2 },
            [LiveRow.CancelOrphanLeg] = new[] { LiveState.A, LiveState.C0 },
            [LiveRow.ShrinkLeg] = new[] { LiveState.A, LiveState.C1, LiveState.C2 },
            [LiveRow.CancelWrongSideLeg] = new[] { LiveState.A, LiveState.C1, LiveState.C2 },
            [LiveRow.InitialBracket] = new[] { LiveState.A, LiveState.C1, LiveState.C2 },
            [LiveRow.UpsizeLeg] = new[] { LiveState.C1 },
            [LiveRow.RepriceLeg] = new[] { LiveState.C1 },
            [LiveRow.Flatten] = new[] { LiveState.C0, LiveState.C1 },
            [LiveRow.Reduce] = new[] { LiveState.C0, LiveState.C1 },
            [LiveRow.CancelProtectingLeg] = new LiveState[0],
        };

        /// <summary>"—" cells: the condition cannot exist in that state.</summary>
        private static readonly HashSet<(LiveRow, LiveState)> NotApplicable = new HashSet<(LiveRow, LiveState)>
        {
            (LiveRow.CancelOrphanLeg, LiveState.C1), (LiveRow.CancelOrphanLeg, LiveState.C2),
            (LiveRow.ShrinkLeg, LiveState.C0), (LiveRow.CancelWrongSideLeg, LiveState.C0), (LiveRow.InitialBracket, LiveState.C0),
            (LiveRow.UpsizeLeg, LiveState.C0), (LiveRow.RepriceLeg, LiveState.C0), (LiveRow.CancelProtectingLeg, LiveState.C0),
        };

        /// <summary>
        /// Architect, 2026-10-23: U19 (b) = a confirmed account after "off" behaves as state A; U19.1 (a) = row 11 Y in C1
        /// and C2, still N in A.
        /// </summary>
        private static bool Expected(LiveRow row, LiveState state, LiveOffPolicy u19, ProtectiveCancelPolicy u191)
        {
            var effective = u19 == LiveOffPolicy.RefuseAll && state != LiveState.A ? LiveState.A : state;
            if (row == LiveRow.CancelProtectingLeg && u191 == ProtectiveCancelPolicy.AllowAfterOff && (effective == LiveState.C1 || effective == LiveState.C2))
                return true;
            return AdrYes[row].Contains(effective);
        }

        public static IEnumerable<object[]> Cells()
        {
            foreach (var u19 in new[] { LiveOffPolicy.CancelEntriesKeepExits, LiveOffPolicy.RefuseAll })
                foreach (var u191 in new[] { ProtectiveCancelPolicy.Refuse, ProtectiveCancelPolicy.AllowAfterOff })
                    foreach (LiveRow row in Enum.GetValues(typeof(LiveRow)))
                        foreach (var state in new[] { LiveState.A, LiveState.C0, LiveState.C1, LiveState.C2 })
                            if (!NotApplicable.Contains((row, state))) yield return new object[] { row, state, u19, u191 };
        }

        [Fact]
        public void Every_cell_of_the_table_is_covered_in_each_combination()
        {
            Assert.Equal(4 * 36, Cells().Count());
        }

        [Theory]
        [MemberData(nameof(Cells))]
        public void Cell(LiveRow row, LiveState state, LiveOffPolicy u19, ProtectiveCancelPolicy u191)
        {
            using var h = LiveHarness(u19, u191);
            var performed = Perform(h, row, state);
            Assert.True(Expected(row, state, u19, u191) == performed,
                "row " + (int)row + " " + state + " " + u19 + "/" + u191 + ": performed=" + performed + "\n" + string.Join("\n", h.Log.Where(l => l.Contains("live.") || l.Contains("guard."))));
            // The pure table agrees with the transcription.
            Assert.Equal(Expected(row, state, u19, u191), LiveAccountTable.Allows(row, state, u19, u191));
        }

        // ------------------------------------------------------------------ scenarios per row

        internal static EngineHarness LiveHarness(LiveOffPolicy u19 = LiveOffPolicy.CancelEntriesKeepExits, ProtectiveCancelPolicy u191 = ProtectiveCancelPolicy.Refuse,
            AdapterBuild build = AdapterBuild.Release, bool simulationOnly = false, DateTime? optInAt = null, bool confirm = true)
        {
            var consent = LiveConsentState.Off();
            consent.Allow(confirm ? new[] { new LiveAccountKey(Connection, Provider, Sim101) } : new LiveAccountKey[0], optInAt ?? OptInAt);
            var options = new EngineOptions { Build = build, LiveOffPolicy = u19, ProtectiveCancelPolicy = u191 };
            var h = new EngineHarness(p => p.AddAccount(Sim101, AccountKind.Live, OcoLocation.Local, Connection, Provider), options, consent);
            h.Platform.SetPrice("MNQ", 21000m);
            var welcome = DefaultWelcome();
            welcome.SimulationOnly = simulationOnly;
            h.Connect(welcome);
            return h;
        }

        /// <summary>
        /// A: the account reports a known non-live kind, which invalidates the confirmation, then live again (unconfirmed);
        /// C*: the user turns the opt-in off. (A transient unknown is state A for Core writes only: LiveAccountTests.)
        /// </summary>
        internal static void Enter(EngineHarness h, LiveState state)
        {
            if (state == LiveState.A)
            {
                h.Platform.SetKind(Sim101, AccountKind.Simulation);
                h.Settle();
                h.Platform.SetKind(Sim101, AccountKind.Live);
            }
            else
            {
                h.Consent.Block();
                h.Engine.OnLiveConsentChanged();
            }
            h.Settle();
        }

        private static void AssertState(EngineHarness h, LiveState expected) =>
            Assert.Equal(expected, h.Engine.GetAccountViews().Single(v => v.PlatformAccount == Sim101).LiveState);

        private static void OpenAllWin(EngineHarness h, string id, int quantity, ProtectionSpec? protection = null) =>
            h.Deliver(Place(id, Side.Buy, quantity, protection: protection));

        private static void External(EngineHarness h, Side side, int quantity)
        {
            h.Platform.ExternalFill(Sim101, Mnq, side, quantity, 21000m);
            h.Settle();
        }

        /// <summary>The position before the transition: C0 flat, C1 opened by All Win, C2 opened outside All Win.</summary>
        private static void Prepare(EngineHarness h, LiveState state, string openId = E1, int quantity = 1, ProtectionSpec? protection = null)
        {
            if (state == LiveState.C2)
            {
                External(h, Side.Buy, 1);
                if (protection != null) OpenAllWin(h, openId, quantity, protection);
            }
            else if (state == LiveState.C1 || (state == LiveState.A && protection != null)) OpenAllWin(h, openId, quantity, protection);
        }

        private static PlatformOrder? Order(EngineHarness h, string id, char role) => h.Platform.OrderByName(Sim101, "aw:" + id + ":" + role);

        private static ExecutionReportPayload Final(EngineHarness h, string id) => h.Reports(id).Last(r => r.Status != ReportStatus.Received);

        private static bool RefusedByConsent(EngineHarness h, string id)
        {
            var final = Final(h, id);
            return final.Status == ReportStatus.Rejected && final.ReasonCode == ReasonCodes.CommandAccountNotSimulation;
        }

        private static bool Perform(EngineHarness h, LiveRow row, LiveState state)
        {
            switch (row)
            {
                case LiveRow.Entry:
                {
                    if (state != LiveState.A) Prepare(h, state);
                    Enter(h, state);
                    AssertState(h, state);
                    h.Deliver(Place(E2, Side.Buy, 1));
                    var performed = h.Platform.CallLog.Any(c => c.Contains(E2));
                    if (!performed) Assert.True(RefusedByConsent(h, E2));
                    return performed;
                }
                case LiveRow.CancelEntry:
                {
                    if (state != LiveState.A) Prepare(h, state);
                    h.Deliver(Place(E2, Side.Buy, 1, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
                    Enter(h, state); // the plugin cancels it on its own (rows 2: U19 a at "off", or state A)
                    AssertState(h, state);
                    var performed = Order(h, E2, 'e')!.State == PlatformOrderState.Cancelled;
                    if (performed)
                        Assert.Contains(h.Reports(E2), r => r.Status == ReportStatus.Cancelled && r.ChildRole == ChildRole.Entry
                            && r.ReasonCode == ReasonCodes.OrderLiveConsentCancelled);
                    return performed;
                }
                case LiveRow.CancelOrphanLeg:
                {
                    OpenAllWin(h, E1, 1, Individual(40, 80));
                    Enter(h, state == LiveState.A ? LiveState.A : LiveState.C1);
                    External(h, Side.Sell, 1); // flat
                    AssertState(h, state);
                    return Order(h, E1, 's')!.State == PlatformOrderState.Cancelled && Order(h, E1, 't')!.State == PlatformOrderState.Cancelled;
                }
                case LiveRow.ShrinkLeg:
                {
                    Prepare(h, state, E1, 2, Individual(40, 80));
                    Enter(h, state);
                    External(h, Side.Sell, h.Platform.Position(Sim101, "MNQ") - 1); // leaves 1
                    AssertState(h, state);
                    return Order(h, E1, 's')!.Quantity == 1;
                }
                case LiveRow.CancelWrongSideLeg:
                {
                    if (state == LiveState.C1)
                    {
                        // A flip with an All Win identity after "off": a consented entry whose cancel at "off" failed fills.
                        OpenAllWin(h, E1, 1, Individual(40, 800));
                        h.Deliver(Place(E2, Side.Sell, 2, order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 21100m }));
                        h.Platform.FailNextCancel = true;
                        Enter(h, LiveState.C1);
                        h.Platform.SetPrice("MNQ", 21100m);
                        h.Settle();
                    }
                    else
                    {
                        OpenAllWin(h, E1, 1, Individual(40, 800));
                        Enter(h, state == LiveState.A ? LiveState.A : LiveState.C1);
                        External(h, Side.Sell, 2);
                    }
                    Assert.Equal(-1, h.Platform.Position(Sim101, "MNQ"));
                    AssertState(h, state);
                    return Order(h, E1, 's')!.State == PlatformOrderState.Cancelled && Order(h, E1, 't')!.State == PlatformOrderState.Cancelled;
                }
                case LiveRow.InitialBracket:
                {
                    if (state != LiveState.A) Prepare(h, state);
                    h.Deliver(Place(E2, Side.Buy, 2, protection: Individual(40, 80), order: new OrderSpec { Type = OrderType.Limit, LimitPrice = 20000m }));
                    h.Platform.DeferCancels = true; // the fill races the cancel at the transition
                    Enter(h, state);
                    h.Platform.FillOrder(Sim101, Order(h, E2, 'e')!.Handle, 1, 20000m);
                    h.Settle();
                    AssertState(h, state);
                    var stop = Order(h, E2, 's');
                    if (stop != null) Assert.Equal(1, stop.Quantity); // capped to what that entry filled
                    return stop != null;
                }
                case LiveRow.UpsizeLeg:
                {
                    Prepare(h, state, E1, 1, Shared(40, 80));
                    var before = Order(h, E1, 's')!.Quantity;
                    Enter(h, state);
                    External(h, Side.Buy, 1);
                    AssertState(h, state);
                    return Order(h, E1, 's')!.Quantity > before;
                }
                case LiveRow.RepriceLeg:
                {
                    Prepare(h, state, E1, 1, Individual(40, 80));
                    var before = Order(h, E1, 's')!.StopPrice;
                    Enter(h, state);
                    AssertState(h, state);
                    h.Deliver(Modify(K1, E1, ChildRole.Stop, delta: -1.00m));
                    var performed = Order(h, E1, 's')!.StopPrice != before;
                    if (!performed) Assert.True(RefusedByConsent(h, K1));
                    return performed;
                }
                case LiveRow.Flatten:
                {
                    if (state == LiveState.A) OpenAllWin(h, E1, 1);
                    else Prepare(h, state);
                    Enter(h, state);
                    AssertState(h, state);
                    h.Deliver(Flatten(K1));
                    var performed = !RefusedByConsent(h, K1);
                    if (performed && state == LiveState.C1) Assert.Equal(0, h.Platform.Position(Sim101, "MNQ"));
                    if (!performed) Assert.DoesNotContain(h.Platform.CallLog, c => c.Contains(K1));
                    return performed;
                }
                case LiveRow.Reduce:
                {
                    if (state == LiveState.A || state == LiveState.C1) OpenAllWin(h, E1, 2);
                    else Prepare(h, state);
                    Enter(h, state);
                    AssertState(h, state);
                    h.Deliver(Place(K1, Side.Sell, 1, PositionEffect.Reduce, positionRef: E1));
                    // Gate order: consent first, then the table, then the command's own checks (never position_changed on A/C2).
                    var performed = !RefusedByConsent(h, K1);
                    if (performed && state == LiveState.C1) Assert.Equal(1, h.Platform.Position(Sim101, "MNQ"));
                    if (performed && state == LiveState.C0) Assert.Equal(ReasonCodes.CommandNothingToReduce, Final(h, K1).ReasonCode);
                    return performed;
                }
                case LiveRow.CancelProtectingLeg:
                {
                    Prepare(h, state, E1, 1, Individual(40, 80));
                    Enter(h, state);
                    AssertState(h, state);
                    h.Deliver(Cancel(K1, E1, ChildRole.Stop));
                    var performed = Order(h, E1, 's')!.State == PlatformOrderState.Cancelled;
                    if (!performed) Assert.True(RefusedByConsent(h, K1));
                    return performed;
                }
                default:
                    throw new ArgumentOutOfRangeException(nameof(row));
            }
        }
    }
}
