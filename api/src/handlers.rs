use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use hub_core::{anyhow, uuid::Uuid};
use organization_credits::Model as OrganizationCreditsModel;
use poem::{
    handler,
    web::{Data, Html, Json, Path},
    IntoResponse, Result,
};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::{
    db::Connection,
    entities::{organization_credits, prelude::OrganizationCredits},
    AppContext, AppState, UserID,
};

#[handler]
pub fn health() {}

#[handler]
pub fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[handler]
pub async fn graphql_handler(
    Data(state): Data<&AppState>,
    user_id: UserID,
    req: GraphQLRequest,
) -> Result<GraphQLResponse> {
    let UserID(user_id) = user_id;

    let context = AppContext::new(state.connection.clone(), user_id);

    Ok(state
        .schema
        .execute(req.0.data(context).data(state.credits.clone()))
        .await
        .into())
}

#[handler]
pub async fn get_organization(
    organization: Path<Uuid>,
    Data(conn): Data<&Connection>,
) -> Result<Json<Option<GetOrganizationResponse>>> {
    let Path(organization) = organization;

    let oc = OrganizationCredits::find_by_id(organization)
        .one(conn.get())
        .await
        .map_err(anyhow::Error::from)?;

    Ok(Json(oc.map(Into::into)))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetOrganizationResponse {
    pub id: Uuid,
    pub balance: i64,
    pub pending_balance: i64,
}

impl From<OrganizationCreditsModel> for GetOrganizationResponse {
    fn from(
        OrganizationCreditsModel {
            id,
            balance,
            pending_balance,
            ..
        }: OrganizationCreditsModel,
    ) -> Self {
        Self {
            id,
            balance,
            pending_balance,
        }
    }
}
