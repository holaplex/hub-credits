pub use sea_orm_migration::prelude::*;

mod m20230418_193337_create_organization_credits_table;
mod m20230419_003253_create_credit_changes_table;
mod m20230424_213550_delete_credit_changes_table;
mod m20230424_213558_create_credits_deductions_table;
mod m20230424_213603_create_credit_deposits_table;
mod m20230424_222452_add_derive_column_cost_per_credit_to_credit_deposits;
mod m20230501_140743_add_pending_balance_column_to_org_credits;
mod m20230501_140859_drop_default_value_from_credit_deductions_id;
mod m20230501_142306_add_status_column_to_credit_deductions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230418_193337_create_organization_credits_table::Migration),
            Box::new(m20230419_003253_create_credit_changes_table::Migration),
            Box::new(m20230424_213550_delete_credit_changes_table::Migration),
            Box::new(m20230424_213558_create_credits_deductions_table::Migration),
            Box::new(m20230424_213603_create_credit_deposits_table::Migration),
            Box::new(
                m20230424_222452_add_derive_column_cost_per_credit_to_credit_deposits::Migration,
            ),
            Box::new(m20230501_140743_add_pending_balance_column_to_org_credits::Migration),
            Box::new(m20230501_140859_drop_default_value_from_credit_deductions_id::Migration),
            Box::new(m20230501_142306_add_status_column_to_credit_deductions::Migration),
        ]
    }
}
