//! The real `/adapter/v1` server on loopback, with fakes for the other modules' ports.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use gateway_application::consume::ConsumeCommandsRequested;
use gateway_application::deliver::DeliverCommands;
use gateway_application::disconnect::HandleDisconnect;
use gateway_application::frames::HandleFrame;
use gateway_application::hello::HandleHello;
use gateway_application::intake::HandleExecutionReport;
use gateway_application::ports::*;
use gateway_application::session::{SessionSettings, Sessions};
use gateway_application::snapshot::HandlePositionSnapshot;
use gateway_enterprise::ids::{AccountId, ConnectionId, VenueAccountId};
use gateway_infra::{GatewayRuntime, PgGatewayStore, ProtocolSchema, router};
use shared_kernel::{Money, SystemClock, UserId};
use uuid::Uuid;

use super::Db;

pub const TOKEN: &str = "awd_AbCdEf123456_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHI";

pub struct Tokens(AdapterPrincipal);

#[async_trait]
impl DeviceTokens for Tokens {
    async fn authenticate(
        &self,
        presented: &PresentedToken,
    ) -> Result<Option<AdapterPrincipal>, PortError> {
        Ok((presented.expose() == TOKEN).then(|| self.0.clone()))
    }
    async fn still_valid(&self, _key_id: &str) -> Result<bool, PortError> {
        Ok(true)
    }
}

/// Binds the first venue it is asked about to the test account.
pub struct Bindings {
    account: AccountId,
    venue: Mutex<Option<VenueAccountId>>,
}

#[async_trait]
impl AccountBindings for Bindings {
    async fn accounts_for_venues(
        &self,
        _u: UserId,
        venues: &[VenueAccountId],
    ) -> Result<HashMap<VenueAccountId, BoundAccount>, PortError> {
        let mut v = self.venue.lock().unwrap();
        let bound = *v.get_or_insert(venues[0]);
        Ok(venues
            .iter()
            .filter(|x| **x == bound)
            .map(|x| {
                (
                    *x,
                    BoundAccount {
                        account_id: self.account,
                        display_name: "Conta A".into(),
                    },
                )
            })
            .collect())
    }
    async fn binding_of(
        &self,
        _u: UserId,
        account: AccountId,
    ) -> Result<Option<VenueAccountId>, PortError> {
        Ok((account == self.account)
            .then(|| *self.venue.lock().unwrap())
            .flatten())
    }
    async fn account_name(
        &self,
        _u: UserId,
        account: AccountId,
    ) -> Result<Option<String>, PortError> {
        Ok((account == self.account).then(|| "Conta A".into()))
    }
}

pub struct NoRisk;

#[async_trait]
impl RiskLimits for NoRisk {
    async fn current_limits(
        &self,
        _u: UserId,
        _a: &[AccountId],
    ) -> Result<HashMap<AccountId, DerivedLimits>, PortError> {
        Ok(HashMap::new())
    }
    async fn commission_rate(&self, _u: UserId, _a: AccountId) -> Result<Option<Money>, PortError> {
        Ok(None)
    }
}

pub struct Server {
    pub url: String,
    pub runtime: Arc<GatewayRuntime>,
    pub schema: Arc<ProtocolSchema>,
    pub consume: ConsumeCommandsRequested,
    pub user: UserId,
    pub account: AccountId,
    pub connection: ConnectionId,
}

pub async fn start(db: &Db) -> Server {
    start_with(db, SessionSettings::default()).await
}

pub async fn start_with(db: &Db, settings: SessionSettings) -> Server {
    let user = db.user().await;
    let connection = ConnectionId::from_uuid(Uuid::now_v7());
    let account = AccountId::from_uuid(Uuid::now_v7());
    let store: Arc<dyn GatewayStore> = Arc::new(PgGatewayStore::new(db.app.clone()));
    let bindings = Arc::new(Bindings {
        account,
        venue: Mutex::new(None),
    });
    let risk = Arc::new(NoRisk);
    let schema = Arc::new(ProtocolSchema::load().unwrap());
    let sessions = Sessions::default();
    let principal = AdapterPrincipal {
        user_id: user,
        connection_id: connection,
        key_id: "AbCdEf123456".into(),
        locale: "en".into(),
    };
    let intake = HandleExecutionReport::new(store.clone(), bindings.clone(), risk.clone());
    let snapshot = HandlePositionSnapshot::new(store.clone(), bindings.clone(), sessions.clone());
    let runtime = Arc::new(GatewayRuntime::new(
        sessions.clone(),
        Arc::new(Tokens(principal)),
        schema.clone(),
        HandleHello::new(
            store.clone(),
            bindings.clone(),
            risk.clone(),
            settings.clone(),
        ),
        HandleFrame::new(schema.clone(), store.clone(), intake, snapshot),
        DeliverCommands::new(
            store.clone(),
            bindings.clone(),
            settings.clone(),
            chrono::Duration::seconds(30),
        ),
        HandleDisconnect::new(store.clone(), sessions.clone()),
        settings,
        Arc::new(SystemClock),
    ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = router(runtime.clone());
    tokio::spawn(async move {
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .unwrap()
    });
    Server {
        url: format!("ws://{addr}/adapter/v1"),
        runtime,
        schema,
        consume: ConsumeCommandsRequested::new(store.clone(), bindings.clone()),
        user,
        account,
        connection,
    }
}
