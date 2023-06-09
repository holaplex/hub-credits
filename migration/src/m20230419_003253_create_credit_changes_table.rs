use sea_orm_migration::prelude::*;

use crate::m20230418_193337_create_organization_credits_table::OrganizationCredits;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credit_changes_initiated_by_idx")
                    .table(CreditChanges::Table)
                    .col(CreditChanges::InitiatedBy)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credit_changes_credits_idx")
                    .table(CreditChanges::Table)
                    .col(CreditChanges::Credits)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credit_changes_code_idx")
                    .table(CreditChanges::Table)
                    .col(CreditChanges::Code)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credit_changes_created_at_idx")
                    .table(CreditChanges::Table)
                    .col(CreditChanges::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CreditChanges::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum CreditChanges {
    Table,
    Id,
    Organization,
    InitiatedBy,
    Credits,
    Code,
    CreatedAt,
}
