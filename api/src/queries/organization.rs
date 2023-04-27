use async_graphql::{Context, Object, Result};
use hub_core::uuid::Uuid;

use crate::objects::Organization;

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "OrganizationQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    #[graphql(entity)]
    async fn find_organization_by_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] id: Uuid,
    ) -> Result<Organization> {
        Ok(Organization { id })
    }
}
