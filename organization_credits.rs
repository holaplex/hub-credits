//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "organization_credits")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub balance: i64,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::credit_deductions::Entity")]
    CreditDeductions,
    #[sea_orm(has_many = "super::credit_deposits::Entity")]
    CreditDeposits,
}

impl Related<super::credit_deductions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CreditDeductions.def()
    }
}

impl Related<super::credit_deposits::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CreditDeposits.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}