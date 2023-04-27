use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use hub_core::uuid::Uuid;
use organization_credits::Model as OrganizationCreditsModel;

use crate::{
    entities::{
        credit_deposits::Model as CreditDeposit, organization_credits, sea_orm_active_enums::Action,
    },
    AppContext,
};

#[derive(SimpleObject, Debug, Clone)]
#[graphql(complex)]
pub struct Credits {
    pub id: Uuid,
    pub balance: i64,
}

impl From<OrganizationCreditsModel> for Credits {
    fn from(OrganizationCreditsModel { id, balance, .. }: OrganizationCreditsModel) -> Self {
        Self { id, balance }
    }
}

#[ComplexObject]
impl Credits {
    async fn deposits(&self, ctx: &Context<'_>) -> Result<Option<Vec<CreditDeposit>>> {
        let AppContext {
            deposits_loader, ..
        } = ctx.data::<AppContext>()?;

        deposits_loader.load_one(self.id).await
    }
}
#[derive(SimpleObject, Debug, Clone)]
pub struct DeductionTotals {
    pub action: Action,
    pub spent: i64,
}
