use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::{prelude::*, FromQueryResult, QuerySelect};

use crate::{
    db::Connection,
    entities::{credit_deductions, prelude::CreditDeductions, sea_orm_active_enums::Action},
    objects::DeductionTotals,
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
    type Value = DeductionTotals;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let deductions = CreditDeductions::find()
            .select_only()
            .column(credit_deductions::Column::Organization)
            .column(credit_deductions::Column::Action)
            .column_as(credit_deductions::Column::Credits.sum(), "spent")
            .filter(
                credit_deductions::Column::Organization.is_in(keys.iter().map(ToOwned::to_owned)),
            )
            .group_by(credit_deductions::Column::Organization)
            .group_by(credit_deductions::Column::Action)
            .into_model::<TotalDeductions>()
            .all(self.db.get())
            .await?;

        Ok(deductions
            .into_iter()
            .map(|d| {
                (d.organization, DeductionTotals {
                    action: d.action,
                    spent: d.spent,
                })
            })
            .collect())
    }
}

#[derive(FromQueryResult)]
pub struct TotalDeductions {
    pub organization: Uuid,
    pub action: Action,
    pub spent: i64,
}
