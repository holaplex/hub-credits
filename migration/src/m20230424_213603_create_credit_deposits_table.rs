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
                    .as_enum(Reason::Type)
                    .values([Reason::Gifted, Reason::Purchased])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CreditDeposits::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CreditDeposits::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("default gen_random_uuid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(CreditDeposits::Organization)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditDeposits::InitiatedBy)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditDeposits::Credits)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CreditDeposits::Cost).double().not_null())
                    .col(
                        ColumnDef::new(CreditDeposits::Reason)
                            .custom(Reason::Type)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditDeposits::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-credit_deposits_org")
                            .from(CreditDeposits::Table, CreditDeposits::Organization)
                            .to(OrganizationCredits::Table, OrganizationCredits::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CreditDeposits::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(Reason::Type).to_owned())
            .await
    }
}

#[derive(Iden)]
enum CreditDeposits {
    Table,
    Id,
    Organization,
    InitiatedBy,
    Credits,
    Cost,
    Reason,
    CreatedAt,
}

enum Reason {
    Type,
    Gifted,
    Purchased,
}

impl Iden for Reason {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Type => "deposit_reason",
            Self::Gifted => "gifted",
            Self::Purchased => "purchased",
        })
        .unwrap();
    }
}
