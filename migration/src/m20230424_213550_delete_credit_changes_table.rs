use sea_orm_migration::prelude::*;

use crate::{
    m20230418_193337_create_organization_credits_table::OrganizationCredits,
    m20230419_003253_create_credit_changes_table::CreditChanges,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CreditChanges::Table).to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CreditChanges::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CreditChanges::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("default gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(CreditChanges::Organization)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CreditChanges::InitiatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(CreditChanges::Credits)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CreditChanges::Code).integer().not_null())
                    .col(
                        ColumnDef::new(CreditChanges::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-credit_changes_org")
                            .from(CreditChanges::Table, CreditChanges::Organization)
                            .to(OrganizationCredits::Table, OrganizationCredits::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}
