#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod dataloaders;
pub mod db;
pub mod entities;
pub mod events;
pub mod handlers;
pub mod mutations;
pub mod objects;
pub mod queries;
pub mod stripe;

use async_graphql::{
    dataloader::DataLoader,
    extensions::{ApolloTracing, Logger},
    EmptyMutation, EmptySubscription, Schema,
};
use dataloaders::{CreditsLoader, TotalDeductionsLoader};
use db::Connection;
use hub_core::{
    anyhow::{Error, Result},
    clap,
    consumer::RecvError,
    credits::{Action, CreditsClient},
    prelude::*,
    tokio,
    uuid::Uuid,
};
use poem::{async_trait, http::StatusCode, FromRequest, Request, RequestBody};
use queries::Query;

use crate::{
    dataloaders::DepositsLoader,
    stripe::{Stripe, StripeArgs},
};
impl hub_core::producer::Message for credits::CreditsEvent {
    type Key = credits::CreditsEventKey;
}

#[allow(clippy::pedantic)]
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/organization.proto.rs"));
    include!(concat!(env!("OUT_DIR"), "/credits_mpsc.rs"));
}

#[allow(clippy::pedantic)]
pub mod credits {
    include!(concat!(env!("OUT_DIR"), "/credits.rs"));
}

#[derive(Debug)]
pub enum Services {
    Organizations(proto::OrganizationEventKey, proto::OrganizationEvents),
    CreditsMpsc(credits::CreditsEventKey, proto::CreditsMpscEvent),
}

impl hub_core::consumer::MessageGroup for Services {
    const REQUESTED_TOPICS: &'static [&'static str] = &["hub-orgs", "credits_mpsc"];

    fn from_message<M: hub_core::consumer::Message>(msg: &M) -> Result<Self, RecvError> {
        let topic = msg.topic();
        let key = msg.key().ok_or(RecvError::MissingKey)?;
        let val = msg.payload().ok_or(RecvError::MissingPayload)?;
        info!(topic, ?key, ?val);

        match topic {
            "hub-orgs" => {
                let key = proto::OrganizationEventKey::decode(key)?;
                let val = proto::OrganizationEvents::decode(val)?;

                Ok(Services::Organizations(key, val))
            },
            "credits_mpsc" => {
                let key = credits::CreditsEventKey::decode(key)?;
                let val = proto::CreditsMpscEvent::decode(val)?;

                Ok(Services::CreditsMpsc(key, val))
            },
            t => Err(RecvError::BadTopic(t.into())),
        }
    }
}

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env, default_value_t = 3010)]
    pub port: u16,

    #[command(flatten)]
    pub db: db::DbArgs,

    #[arg(short, long, env)]
    pub gift_amount: u64,

    #[command(flatten)]
    pub stripe: StripeArgs,
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(Debug, Clone, Copy)]
pub struct UserID(Option<Uuid>);

impl TryFrom<&str> for UserID {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let id = Uuid::from_str(value)?;

        Ok(Self(Some(id)))
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for UserID {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let id = req
            .headers()
            .get("X-USER-ID")
            .and_then(|value| value.to_str().ok())
            .map_or(Ok(Self(None)), Self::try_from)?;

        Ok(id)
    }
}

#[derive(Debug, Clone)]
pub struct StripeSignature(String);

impl StripeSignature {
    #[must_use]
    pub fn content(&self) -> String {
        self.0.clone()
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for StripeSignature {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let stripe_signature = req
            .headers()
            .get("Stripe-Signature")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                poem::Error::from_string("missing stripe signature", StatusCode::BAD_REQUEST)
            })?;

        Ok(StripeSignature(stripe_signature.to_string()))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter, strum::AsRefStr)]
pub enum Actions {
    CreateDrop,
    MintEdition,
    RetryMint,
    TransferAsset,
}

impl From<Actions> for Action {
    fn from(value: Actions) -> Self {
        match value {
            Actions::CreateDrop => Action::CreateDrop,
            Actions::MintEdition => Action::MintEdition,
            Actions::RetryMint => Action::RetryMint,
            Actions::TransferAsset => Action::TransferAsset,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub connection: Connection,
    pub credits: CreditsClient<Actions>,
    pub stripe: Stripe,
}

impl AppState {
    #[must_use]
    pub fn new(
        schema: AppSchema,
        connection: Connection,
        credits: CreditsClient<Actions>,
        stripe: Stripe,
    ) -> Self {
        Self {
            schema,
            connection,
            credits,
            stripe,
        }
    }
}

pub struct AppContext {
    pub db: Connection,
    pub user_id: Option<Uuid>,
    pub credits_loader: DataLoader<CreditsLoader>,
    pub total_deductions_loader: DataLoader<TotalDeductionsLoader>,
    pub deposits_loader: DataLoader<DepositsLoader>,
}

impl AppContext {
    #[must_use]
    pub fn new(db: Connection, user_id: Option<Uuid>) -> Self {
        let credits_loader = DataLoader::new(CreditsLoader::new(db.clone()), tokio::spawn);
        let total_deductions_loader =
            DataLoader::new(TotalDeductionsLoader::new(db.clone()), tokio::spawn);
        let deposits_loader = DataLoader::new(DepositsLoader::new(db.clone()), tokio::spawn);

        Self {
            db,
            user_id,
            credits_loader,
            total_deductions_loader,
            deposits_loader,
        }
    }
}

/// Builds the GraphQL Schema, attaching the Database to the context
#[must_use]
pub fn build_schema() -> AppSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .enable_federation()
        .finish()
}
