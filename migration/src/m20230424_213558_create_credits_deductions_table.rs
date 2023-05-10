use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::m20230418_193337_create_organization_credits_table::OrganizationCredits;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Action::Type)
                    .values([
                        Action::CreateDrop,
                        Action::MintEdition,
                        Action::TransferAsset,
                        Action::RetryMint,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Blockchain::Type)
                    .values([
                        Blockchain::Solana,
                        Blockchain::Ethereum,
                        Blockchain::Polygon,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CreditDeductions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CreditDeductions::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("default gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(CreditDeductions::Organization)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditDeductions::InitiatedBy)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditDeductions::Credits)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditDeductions::Action)
                            .custom(Action::Type)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditDeductions::Blockchain)
                            .custom(Blockchain::Type)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditDeductions::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-credit_deductions_org")
                            .from(CreditDeductions::Table, CreditDeductions::Organization)
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
                    .name("credit_deductions_initiated_by_idx")
                    .table(CreditDeductions::Table)
                    .col(CreditDeductions::InitiatedBy)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credit_deductions_credits_idx")
                    .table(CreditDeductions::Table)
                    .col(CreditDeductions::Credits)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credit_deductions_action_idx")
                    .table(CreditDeductions::Table)
                    .col(CreditDeductions::Action)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credit_deductions_created_at_idx")
                    .table(CreditDeductions::Table)
                    .col(CreditDeductions::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CreditDeductions::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(Blockchain::Type).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(Action::Type).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum CreditDeductions {
    Table,
    Id,
    Organization,
    InitiatedBy,
    Credits,
    Action,
    Blockchain,
    CreatedAt,
}

enum Action {
    Type,
    CreateDrop,
    MintEdition,
    TransferAsset,
    RetryMint,
}

impl Iden for Action {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Type => "action",
            Self::CreateDrop => "create_drop",
            Self::MintEdition => "mint_edition",
            Self::TransferAsset => "transfer_asset",
            Self::RetryMint => "retry_mint",
        })
        .unwrap();
    }
}

enum Blockchain {
    Type,
    Solana,
    Polygon,
    Ethereum,
}

impl Iden for Blockchain {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Type => "blockchain",
            Self::Solana => "solana",
            Self::Polygon => "polygon",
            Self::Ethereum => "ethereum",
        })
        .unwrap();
    }
}
