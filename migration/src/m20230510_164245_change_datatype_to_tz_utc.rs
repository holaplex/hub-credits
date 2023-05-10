use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, Statement},
};

use crate::{
    m20230418_193337_create_organization_credits_table::OrganizationCredits,
    m20230424_213558_create_credits_deductions_table::CreditDeductions,
    m20230424_213603_create_credit_deposits_table::CreditDeposits,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"alter database credits set timezone to 'utc' ;"#.to_string(),
        );

        db.execute(stmt).await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganizationCredits::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganizationCredits::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(CreditDeductions::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(CreditDeposits::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
