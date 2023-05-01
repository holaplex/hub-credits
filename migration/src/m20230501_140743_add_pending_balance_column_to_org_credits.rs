use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OrganizationCredits::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(OrganizationCredits::PendingBalance)
                            .big_integer()
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
                    .table(OrganizationCredits::Table)
                    .drop_column(OrganizationCredits::PendingBalance)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum OrganizationCredits {
    Table,
    PendingBalance,
}
