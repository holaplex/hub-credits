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
                    .add_value(Alias::new("create_collection"))
                    .add_value(Alias::new("retry_collection"))
                    .add_value(Alias::new("mint"))
                    .add_value(Alias::new("mint_compressed"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
