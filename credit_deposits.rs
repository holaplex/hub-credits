//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

use super::sea_orm_active_enums::DepositReason;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "credit_deposits")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub organization: Uuid,
    pub initiated_by: Uuid,
    pub credits: i64,
    #[sea_orm(column_type = "Double")]
    pub cost: f64,
    pub reason: DepositReason,
    pub created_at: DateTime,
    #[sea_orm(column_type = "Double")]
    pub per_credit_cost: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization_credits::Entity",
        from = "Column::Organization",
        to = "super::organization_credits::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    OrganizationCredits,
}

impl Related<super::organization_credits::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrganizationCredits.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
