//! Appends accounts events to `app.outbox` inside the caller's user-scoped transaction
//! (ALIGN-2): the event commits or rolls back with the change.

use accounts_application::AccountsEvent;
use shared_kernel::UserId;
use sqlx::PgConnection;
use uuid::Uuid;

pub(crate) async fn insert(
    conn: &mut PgConnection,
    user_id: UserId,
    event: &AccountsEvent,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload)
           VALUES ($1, $2, $3, $4, $5)"#,
        Uuid::now_v7(),
        user_id.as_uuid(),
        event.event_type(),
        event.at(),
        event.payload(),
    )
    .execute(conn)
    .await?;
    Ok(())
}
