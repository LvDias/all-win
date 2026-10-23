//! `catalog.opt_in_rule_version` (global, no RLS; append-only).

use async_trait::async_trait;
use catalog_application::CatalogError;
use catalog_application::rule_version::{RuleChange, RuleVersion, RuleVersionRepository};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgRuleVersionRepository {
    pool: PgPool,
}

impl PgRuleVersionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

struct Row {
    id: Uuid,
    kind: String,
    firm_stance: String,
    is_generic: bool,
    firm_id: Option<Uuid>,
    firm_platform: Option<String>,
    version_number: i32,
    published_at: chrono::DateTime<chrono::Utc>,
    quotes: serde_json::Value,
    research: Option<serde_json::Value>,
    change_cause: Option<String>,
    change_reason: Option<serde_json::Value>,
    previous_id: Option<Uuid>,
    previous_firm_stance: Option<String>,
}

macro_rules! select_rule {
    ($tail:literal, $($arg:expr),*) => {
        sqlx::query_as!(
            Row,
            r#"SELECT v.id, v.kind, v.firm_stance, v.is_generic, v.firm_id, v.firm_platform,
                      v.version_number, v.published_at, v.quotes, v.research, v.change_cause,
                      v.change_reason, v.previous_id, p.firm_stance AS "previous_firm_stance?"
                 FROM catalog.opt_in_rule_version v
                 LEFT JOIN catalog.opt_in_rule_version p ON p.id = v.previous_id "# + $tail,
            $($arg),*
        )
    };
}

fn to_rule(r: Row) -> RuleVersion {
    let change = match (r.previous_id, r.change_cause, r.previous_firm_stance) {
        (Some(previous), Some(cause), Some(previous_stance)) => Some(RuleChange {
            previous_rule_version_id: previous,
            previous_firm_stance: previous_stance,
            cause,
            reason: r.change_reason,
        }),
        _ => None,
    };
    RuleVersion {
        id: r.id,
        kind: r.kind,
        firm_stance: r.firm_stance,
        firm_platform: r.firm_platform,
        is_generic: r.is_generic,
        firm_id: r.firm_id,
        version_number: r.version_number,
        published_at: r.published_at,
        quotes: r.quotes,
        research: r.research,
        change,
    }
}

fn catalog_error(e: sqlx::Error) -> CatalogError {
    CatalogError(e.to_string())
}

#[async_trait]
impl RuleVersionRepository for PgRuleVersionRepository {
    async fn by_id(&self, id: Uuid) -> Result<Option<RuleVersion>, CatalogError> {
        select_rule!("WHERE v.id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map(|r| r.map(to_rule))
            .map_err(catalog_error)
    }

    async fn current_generic(&self, kind: &str) -> Result<Option<RuleVersion>, CatalogError> {
        select_rule!(
            "WHERE v.is_generic AND v.kind = $1 ORDER BY v.version_number DESC LIMIT 1",
            kind
        )
        .fetch_optional(&self.pool)
        .await
        .map(|r| r.map(to_rule))
        .map_err(catalog_error)
    }
}
