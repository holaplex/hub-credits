use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::{
    db::Connection,
    entities::{credit_deposits, credit_deposits::Model as CreditDeposit, prelude::CreditDeposits},
};

#[derive(Debug, Clone)]
pub struct Loader {
    pub db: Connection,
}

impl Loader {
    #[must_use]
    pub fn new(db: Connection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for Loader {
    type Error = FieldError;
    type Value = Vec<CreditDeposit>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let deposits = CreditDeposits::find()
            .filter(credit_deposits::Column::Organization.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        Ok(deposits
            .into_iter()
            .fold(HashMap::new(), |mut acc, deposit| {
                acc.entry(deposit.organization)
                    .or_insert_with(Vec::new)
                    .push(deposit);

                acc
            }))
    }
}
