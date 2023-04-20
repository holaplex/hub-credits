//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use async_graphql::{Enum, Result};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "credit_changes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub organization: Uuid,
    pub initiated_by: Uuid,
    pub credits: i64,
    pub code: Code,
    pub created_at: DateTime,
}

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Code {
    #[graphql(name = "CREDIT_DEPOSIT")]
    #[sea_orm(num_value = 1)]
    CreditDeposit,
    #[graphql(name = "SOLANA_CREATE_DROP")]
    #[sea_orm(num_value = 2)]
    SolanaCreateDrop,
    #[graphql(name = "POLYGON_CREATE_DROP")]
    #[sea_orm(num_value = 3)]
    PolygonCreateDrop,
    #[graphql(name = "SOLANA_MINT_EDITION")]
    #[sea_orm(num_value = 4)]
    SolanaMintEdition,
    #[graphql(name = "POLYGON_MINT_EDITION")]
    #[sea_orm(num_value = 5)]
    PolygonMintEdition,
    #[graphql(name = "SOLANA_TRANSFER_ASSET")]
    #[sea_orm(num_value = 6)]
    SolanaTransferAsset,
    #[graphql(name = "POLYGON_TRANSFER_ASSET")]
    #[sea_orm(num_value = 7)]
    PolygonTransferAsset,
    #[graphql(name = "SOLANA_RETRY_MINT")]
    #[sea_orm(num_value = 8)]
    SolanaRetryMint,
    #[graphql(name = "POLYGON_RETRY_MINT")]
    #[sea_orm(num_value = 9)]
    PolygonRetryMint,
}

impl From<Code> for i32 {
    fn from(value: Code) -> Self {
        match value {
            Code::CreditDeposit => 1,
            Code::SolanaCreateDrop => 2,
            Code::PolygonCreateDrop => 3,
            Code::SolanaMintEdition => 4,
            Code::PolygonMintEdition => 5,
            Code::SolanaTransferAsset => 6,
            Code::PolygonTransferAsset => 7,
            Code::SolanaRetryMint => 8,
            Code::PolygonRetryMint => 9,
        }
    }
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
