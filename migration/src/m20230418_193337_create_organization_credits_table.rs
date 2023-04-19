use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrganizationCredits::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganizationCredits::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("default gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(OrganizationCredits::Balance)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationCredits::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .col(ColumnDef::new(OrganizationCredits::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("organization_credits_created_at_idx")
                    .table(OrganizationCredits::Table)
                    .col(OrganizationCredits::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("organization_credits_updated_at_idx")
                    .table(OrganizationCredits::Table)
                    .col(OrganizationCredits::UpdatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("organization_credits_balance_idx")
                    .table(OrganizationCredits::Table)
                    .col(OrganizationCredits::Balance)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationCredits::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum OrganizationCredits {
    Table,
    Id,
    Balance,
    CreatedAt,
    UpdatedAt,
}
