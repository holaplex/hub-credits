pub use sea_orm_migration::prelude::*;

mod m20230418_193337_create_organization_credits_table;
mod m20230419_003253_create_credit_changes_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230418_193337_create_organization_credits_table::Migration),
            Box::new(m20230419_003253_create_credit_changes_table::Migration),
        ]
    }
}
