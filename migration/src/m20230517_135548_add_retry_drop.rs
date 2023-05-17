use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::m20230424_213558_create_credits_deductions_table::Action;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_type(
                Type::alter()
                    .name(Action::Type)
                    .add_value(Alias::new("retry_drop"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
