use async_graphql::SimpleObject;
use hub_core::uuid::Uuid;

use crate::entities::{credit_deposits::Model as CreditDeposit, sea_orm_active_enums::Action};

#[derive(SimpleObject, Debug, Clone)]
pub struct Credits {
    pub id: Uuid,
    pub balance: i64,
    pub deposits: Vec<CreditDeposit>,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct DeductionTotals {
    pub action: Action,
    pub spent: i64,
}
