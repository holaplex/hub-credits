use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::{
    db::Connection,
    entities::{
        organization_credits,
        prelude::{CreditDeposits, OrganizationCredits},
    },
    objects::Credits,
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
    type Value = Credits;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let deposits = OrganizationCredits::find()
            .find_with_related(CreditDeposits)
            .filter(organization_credits::Column::Id.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        Ok(deposits
            .into_iter()
            .map(|(org_credits, credit_deposits)| {
                (org_credits.id, Credits {
                    id: org_credits.id,
                    balance: org_credits.balance,
                    deposits: credit_deposits,
                })
            })
            .collect())
    }
}
