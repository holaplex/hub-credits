//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

use super::sea_orm_active_enums::{Action, Blockchain, DeductionStatus};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "credit_deductions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub organization: Uuid,
    pub initiated_by: Uuid,
    pub credits: i64,
    pub action: Action,
    pub blockchain: Blockchain,
    pub created_at: DateTimeWithTimeZone,
    pub status: DeductionStatus,
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
