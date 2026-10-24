//! A position's cost basis and the realized gross P&L of each fill, on AVERAGE COST (S13; trades
//! keep FIFO for their own figures). Proposal `/tmp/allwin-review/s13-realized-pnl-per-fill.md`.
//!
//! A fill that reduces the position realizes `closed × (price − avg_before) × direction ×
//! point_value`. A fill that flips it closes the old side and opens the rest at its price. When
//! the cost basis is unknown (an amend/remove without the platform's average price), nothing can be
//! realized: the caller treats the account as stale (fail closed).
//!
//! Esquadro (2026-10-24): the PLATFORM's cost basis wins whenever a report carries it
//! (`avg_price_after`); the computed one is used only without it. A computed basis more than one
//! tick away from the platform's is logged and counted (`Reconciled::diverged`), never a sync
//! reason.

use rust_decimal::{Decimal, RoundingStrategy};

/// Money per fill: exact product, then 4 decimals, half AWAY from zero, like PostgreSQL's
/// `round(numeric, 4)` (Esquadro; data model rev. 16.23 §5.6). The only place the rule lives.
pub fn round_money(v: Decimal) -> Decimal {
    v.round_dp_with_strategy(4, RoundingStrategy::MidpointAwayFromZero)
}

/// `NUMERIC(22,10)`: average prices keep 10 decimals, half to even.
fn round_price(v: Decimal) -> Decimal {
    v.round_dp_with_strategy(10, RoundingStrategy::MidpointNearestEven)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "buy" => Some(Self::Buy),
            "sell" => Some(Self::Sell),
            _ => None,
        }
    }
}

/// One account's position in one contract. `qty` is signed (long > 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub qty: i64,
    pub avg_price: Decimal,
    /// False after an amend/remove without the platform's average price.
    pub basis_known: bool,
}

impl Position {
    pub const FLAT: Self = Self {
        qty: 0,
        avg_price: Decimal::ZERO,
        basis_known: true,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FillEffect {
    /// `None` when the cost basis is unknown.
    pub realized_gross: Option<Decimal>,
    pub closed_qty: i64,
    pub after: Position,
}

pub fn apply_fill(
    before: Position,
    side: Side,
    qty: i64,
    price: Decimal,
    point_value: Decimal,
) -> FillEffect {
    let signed = match side {
        Side::Buy => qty,
        Side::Sell => -qty,
    };
    let after_qty = before.qty + signed;
    let same_direction = before.qty == 0 || (before.qty > 0) == (signed > 0);
    if same_direction {
        let held = Decimal::from(before.qty.abs());
        let added = Decimal::from(qty);
        let avg = if before.qty == 0 {
            price
        } else {
            round_price((held * before.avg_price + added * price) / (held + added))
        };
        return FillEffect {
            realized_gross: Some(Decimal::ZERO),
            closed_qty: 0,
            after: Position {
                qty: after_qty,
                avg_price: avg,
                basis_known: before.basis_known || before.qty == 0,
            },
        };
    }
    let closed = before.qty.abs().min(qty);
    let direction = if before.qty > 0 {
        Decimal::ONE
    } else {
        -Decimal::ONE
    };
    let realized = before.basis_known.then(|| {
        round_money(Decimal::from(closed) * (price - before.avg_price) * direction * point_value)
    });
    let after = if after_qty == 0 {
        Position::FLAT
    } else if (after_qty > 0) == (before.qty > 0) {
        // Partly closed: the rest keeps its average price.
        Position {
            qty: after_qty,
            ..before
        }
    } else {
        // Flipped: the remainder opened at this fill's price.
        Position {
            qty: after_qty,
            avg_price: price,
            basis_known: true,
        }
    };
    FillEffect {
        realized_gross: realized,
        closed_qty: closed,
        after,
    }
}

/// The position before a fill (data model rev. 16.23, M1). With the report's `position_after`
/// the platform wins: `before = position_after − signed qty`. The stored row is used only if its
/// quantity matches; otherwise the average before is known only when the fill reduces without
/// zeroing or flipping (on average cost a reduction keeps the average, so before = after). Without
/// `position_after` and without a stored row the position before is unknown (`None`), never flat.
pub fn position_before(
    stored: Option<Position>,
    side: Side,
    qty: i64,
    position_after: Option<i64>,
    avg_price_after: Option<Decimal>,
) -> Option<Position> {
    let Some(after) = position_after else {
        return stored;
    };
    let signed = match side {
        Side::Buy => qty,
        Side::Sell => -qty,
    };
    let before = after - signed;
    if let Some(p) = stored
        && p.qty == before
    {
        return Some(p);
    }
    if before == 0 {
        return Some(Position::FLAT);
    }
    let reduces_only = after != 0 && (after > 0) == (before > 0) && after.abs() < before.abs();
    Some(match avg_price_after {
        Some(avg) if reduces_only => Position {
            qty: before,
            avg_price: avg,
            basis_known: true,
        },
        _ => Position {
            qty: before,
            avg_price: Decimal::ZERO,
            basis_known: false,
        },
    })
}

/// The position after a report: the platform's `position_after` / `avg_price_after` when present,
/// else the computed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reconciled {
    pub position: Position,
    /// The computed basis was more than `tick` away from the platform's (log + metric).
    pub diverged: bool,
}

pub fn reconcile(
    computed: Position,
    platform_qty: Option<i64>,
    platform_avg: Option<Decimal>,
    tick: Decimal,
) -> Reconciled {
    let qty = platform_qty.unwrap_or(computed.qty);
    let Some(avg) = platform_avg else {
        return Reconciled {
            position: Position { qty, ..computed },
            diverged: false,
        };
    };
    let diverged = computed.basis_known
        && computed.qty != 0
        && qty != 0
        && (computed.avg_price - avg).abs() > tick;
    let position = if qty == 0 {
        Position::FLAT
    } else {
        Position {
            qty,
            avg_price: round_price(avg),
            basis_known: true,
        }
    };
    Reconciled { position, diverged }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    // MNQ: 2 USD per point.
    const PV: &str = "2";

    fn long(qty: i64, avg: &str) -> Position {
        Position {
            qty,
            avg_price: d(avg),
            basis_known: true,
        }
    }

    #[test]
    fn opening_and_adding_average_the_price() {
        let e = apply_fill(Position::FLAT, Side::Buy, 2, d("21000.25"), d(PV));
        assert_eq!(
            (e.realized_gross, e.after),
            (Some(d("0")), long(2, "21000.25"))
        );
        let e = apply_fill(e.after, Side::Buy, 1, d("21003.25"), d(PV));
        assert_eq!(e.after, long(3, "21001.25"));
    }

    #[test]
    fn reducing_realizes_on_the_average_and_keeps_it() {
        let e = apply_fill(long(3, "21001.25"), Side::Sell, 2, d("21011.25"), d(PV));
        // 2 × 10 points × 2 USD.
        assert_eq!(e.realized_gross, Some(d("40")));
        assert_eq!((e.closed_qty, e.after), (2, long(1, "21001.25")));
        let e = apply_fill(e.after, Side::Sell, 1, d("20991.25"), d(PV));
        assert_eq!(
            (e.realized_gross, e.after),
            (Some(d("-20")), Position::FLAT)
        );
    }

    #[test]
    fn a_flip_closes_the_old_side_and_opens_the_rest_at_the_fill_price() {
        let e = apply_fill(long(1, "21000"), Side::Sell, 3, d("21005"), d(PV));
        assert_eq!(e.realized_gross, Some(d("10")));
        assert_eq!(
            e.after,
            Position {
                qty: -2,
                avg_price: d("21005"),
                basis_known: true
            }
        );
        // Short covered lower: profit.
        let e = apply_fill(e.after, Side::Buy, 2, d("21000"), d(PV));
        assert_eq!((e.realized_gross, e.after), (Some(d("20")), Position::FLAT));
    }

    #[test]
    fn money_rounds_half_away_from_zero_at_four_decimals() {
        // 1/128 point × 31.25 USD = 0.244140625 → 0.2441.
        let e = apply_fill(long(1, "100"), Side::Sell, 1, d("100.0078125"), d("31.25"));
        assert_eq!(e.realized_gross, Some(d("0.2441")));
        assert_eq!(round_money(d("0.00005")), d("0.0001"));
        assert_eq!(round_money(d("-0.00005")), d("-0.0001"));
        assert_eq!(round_money(d("0.00015")), d("0.0002"));
    }

    #[test]
    fn an_unknown_basis_realizes_nothing_until_flat_or_reanchored() {
        let unknown = Position {
            basis_known: false,
            ..long(2, "0")
        };
        let e = apply_fill(unknown, Side::Sell, 1, d("21000"), d(PV));
        assert_eq!(e.realized_gross, None);
        assert!(!e.after.basis_known);
        let e = apply_fill(e.after, Side::Sell, 1, d("21000"), d(PV));
        assert_eq!((e.realized_gross, e.after), (None, Position::FLAT));
    }

    #[test]
    fn the_platform_basis_wins_and_a_divergence_over_a_tick_is_flagged() {
        let tick = d("0.25");
        let computed = long(2, "21000.10");
        let r = reconcile(computed, Some(2), Some(d("21000.25")), tick);
        assert_eq!((r.position, r.diverged), (long(2, "21000.25"), false));
        let r = reconcile(computed, Some(2), Some(d("21001.00")), tick);
        assert_eq!((r.position, r.diverged), (long(2, "21001.00"), true));
        // Without the platform's price the computed basis stays.
        let r = reconcile(computed, Some(2), None, tick);
        assert_eq!((r.position, r.diverged), (computed, false));
        // An unknown basis is re-anchored by the platform's price.
        let unknown = Position {
            basis_known: false,
            ..long(1, "0")
        };
        let r = reconcile(unknown, Some(1), Some(d("21000")), tick);
        assert_eq!((r.position, r.diverged), (long(1, "21000"), false));
        assert_eq!(
            reconcile(computed, Some(0), Some(d("0")), tick).position,
            Position::FLAT
        );
    }

    #[test]
    fn position_before_follows_the_platform_and_never_guesses() {
        let stored = long(3, "21000");
        // Stored quantity matches the platform's before: the stored basis is used.
        assert_eq!(
            position_before(Some(stored), Side::Sell, 1, Some(2), Some(d("21000"))),
            Some(stored)
        );
        // Stored row diverged (or missing); a pure reduction: before's average = after's.
        assert_eq!(
            position_before(
                Some(long(5, "1")),
                Side::Sell,
                1,
                Some(2),
                Some(d("21000.5"))
            ),
            Some(long(3, "21000.5"))
        );
        assert_eq!(
            position_before(None, Side::Sell, 1, Some(2), Some(d("21000.5"))),
            Some(long(3, "21000.5"))
        );
        // A flip or a close to flat cannot tell the old average: unknown.
        let unknown = |qty| {
            Some(Position {
                qty,
                avg_price: d("0"),
                basis_known: false,
            })
        };
        assert_eq!(
            position_before(None, Side::Sell, 3, Some(-2), Some(d("21000"))),
            unknown(1)
        );
        assert_eq!(
            position_before(None, Side::Sell, 2, Some(0), None),
            unknown(2)
        );
        // Opening from flat is known.
        assert_eq!(
            position_before(None, Side::Buy, 2, Some(2), Some(d("21000"))),
            Some(Position::FLAT)
        );
        // No position_after and no stored row: unknown, never flat.
        assert_eq!(position_before(None, Side::Buy, 1, None, None), None);
        assert_eq!(
            position_before(Some(stored), Side::Buy, 1, None, None),
            Some(stored)
        );
    }

    #[test]
    fn a_fill_to_flat_with_a_matching_row_uses_the_row_basis() {
        // 16.23: the platform's avg_price_after is null or 0 when flat; the stored row wins.
        let stored = long(2, "21000");
        let before = position_before(Some(stored), Side::Sell, 2, Some(0), None).unwrap();
        assert_eq!(before, stored);
        let e = apply_fill(before, Side::Sell, 2, d("21010"), d(PV));
        assert_eq!(e.realized_gross, Some(d("40")));
        let r = reconcile(e.after, Some(0), Some(d("0")), d("0.25"));
        assert_eq!((r.position, r.diverged), (Position::FLAT, false));
    }
}
