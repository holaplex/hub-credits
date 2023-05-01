use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(DeductionStatus::Type)
                    .values([DeductionStatus::Pending, DeductionStatus::Confirmed])
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(CreditDeductions::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(CreditDeductions::Status)
                            .custom(DeductionStatus::Type)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(CreditDeductions::Table)
                    .drop_column(CreditDeductions::Status)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum CreditDeductions {
    Table,
    Status,
}

pub enum DeductionStatus {
    Type,
    Pending,
    Confirmed,
}

impl Iden for DeductionStatus {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Type => "deduction_status",
            Self::Pending => "pending",
            Self::Confirmed => "confirmed",
        })
        .unwrap();
    }
}
