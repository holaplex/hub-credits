#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod dataloaders;
pub mod db;
#[allow(clippy::pedantic)]
pub mod entities;
pub mod handlers;
pub mod mutations;
pub mod objects;
pub mod queries;

use async_graphql::{
    dataloader::DataLoader,
    extensions::{ApolloTracing, Logger},
    EmptySubscription, Schema,
};
use dataloaders::{CreditsLoader, TotalDeductionsLoader};
use db::Connection;
use hub_core::{
    anyhow::{Error, Result},
    clap,
    prelude::*,
    tokio,
    uuid::Uuid,
};
use mutations::Mutation;
use poem::{async_trait, FromRequest, Request, RequestBody};
use queries::Query;

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env, default_value_t = 3010)]
    pub port: u16,

    #[command(flatten)]
    pub db: db::DbArgs,
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

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

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub connection: Connection,
}

impl AppState {
    #[must_use]
    pub fn new(schema: AppSchema, connection: Connection) -> Self {
        Self { schema, connection }
    }
}

pub struct AppContext {
    pub db: Connection,
    pub user_id: Option<Uuid>,
    pub credits_loader: DataLoader<CreditsLoader>,
    pub total_deductions_loader: DataLoader<TotalDeductionsLoader>,
}

impl AppContext {
    #[must_use]
    pub fn new(db: Connection, user_id: Option<Uuid>) -> Self {
        let credits_loader = DataLoader::new(CreditsLoader::new(db.clone()), tokio::spawn);
        let total_deductions_loader =
            DataLoader::new(TotalDeductionsLoader::new(db.clone()), tokio::spawn);

        Self {
            db,
            user_id,
            credits_loader,
            total_deductions_loader,
        }
    }
}

/// Builds the GraphQL Schema, attaching the Database to the context
#[must_use]
pub fn build_schema() -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .enable_federation()
        .finish()
}
