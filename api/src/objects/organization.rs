use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use hub_core::uuid::Uuid;

use super::credits::{Credits, DeductionTotals};
use crate::AppContext;

// Define a GraphQL object for an organization
#[derive(SimpleObject, Debug, Clone)]
#[graphql(complex)]
pub struct Organization {
    // Define an external GraphQL field for the ID of the organization
    #[graphql(external)]
    pub id: Uuid,
}

#[ComplexObject]
impl Organization {
    /// Define an asynchronous function to load the credits for the organization
    /// Returns `Credits` object
    /// #Errors
    /// returns error if credits_loader is not found in the context or if the loader fails to load the credits

    pub async fn credits(&self, ctx: &Context<'_>) -> Result<Option<Credits>> {
        let AppContext { credits_loader, .. } = ctx.data::<AppContext>()?;

        credits_loader.load_one(self.id).await
    }
    /// Define an asynchronous function to load the total credits deducted grouped by each action
    /// Returns `DeductionTotals` object
    /// #Errors
    /// returns error if total_deductions_loader is not found in the context or if the loader fails to load the total deductions

    pub async fn deduction_totals(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<DeductionTotals>> {
        let AppContext {
            total_deductions_loader,
            ..
        } = ctx.data::<AppContext>()?;

        total_deductions_loader.load_one(id).await
    }
}
