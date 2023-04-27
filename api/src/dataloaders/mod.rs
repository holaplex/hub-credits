pub mod credits;
pub mod deposits;
pub mod total_deductions;

pub use credits::Loader as CreditsLoader;
pub use deposits::Loader as DepositsLoader;
pub use total_deductions::Loader as TotalDeductionsLoader;
