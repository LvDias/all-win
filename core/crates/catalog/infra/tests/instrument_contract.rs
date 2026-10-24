//! The rollover calendar (ADR-0003 P5): the contract month active at an instant.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use catalog_application::instrument::ContractReader;
use catalog_infra::PgInstrumentRepository;
use chrono::{TimeZone, Utc};

#[tokio::test]
async fn the_latest_rollover_at_or_before_the_instant_wins() {
    let Some(db) = support::db().await else {
        return;
    };
    let mnq = "01a166e7-a8d0-700f-bade-6d8493132056";
    let at = |m: u32, d: u32| Utc.with_ymd_and_hms(2026, m, d, 13, 30, 0).unwrap();
    for (month, from) in [("2026-09-01", at(6, 11)), ("2026-12-01", at(9, 10))] {
        sqlx::query(
            "INSERT INTO catalog.instrument_contract (instrument_id, contract_month, active_from)
             VALUES ($1::uuid, $2::date, $3) ON CONFLICT DO NOTHING",
        )
        .bind(mnq)
        .bind(month)
        .bind(from)
        .execute(&db.admin)
        .await
        .unwrap();
    }
    let repo = PgInstrumentRepository::new(db.app.clone());
    let active = |t| {
        let repo = &repo;
        async move { repo.active_contract("MNQ", t).await.unwrap() }
    };
    assert_eq!(active(at(6, 1)).await, None, "before the first row");
    assert_eq!(active(at(6, 11)).await.as_deref(), Some("2026-09"));
    assert_eq!(active(at(9, 9)).await.as_deref(), Some("2026-09"));
    assert_eq!(active(at(9, 10)).await.as_deref(), Some("2026-12"));
    assert_eq!(repo.active_contract("ZZZ", at(10, 1)).await.unwrap(), None);
}
