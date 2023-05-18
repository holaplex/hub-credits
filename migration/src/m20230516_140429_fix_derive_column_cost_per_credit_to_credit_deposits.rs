use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(CreditDeposits::Table)
                    .drop_column(CreditDeposits::PerCreditCost)
                    .to_owned(),
            )
            .await?;

        let sql = r#"
        ALTER TABLE credit_deposits
        ADD COLUMN per_credit_cost double precision
        GENERATED ALWAYS AS (CASE WHEN cost = 0 THEN 0 ELSE cost/credits END)
        STORED NOT NULL;
    "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(Iden)]
pub enum CreditDeposits {
    Table,
    PerCreditCost,
}
