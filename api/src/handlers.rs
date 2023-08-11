use std::str::FromStr;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use hub_core::{anyhow, prelude::*, uuid::Uuid};
use organization_credits::Model as OrganizationCreditsModel;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Html, Json, Path},
    Body, Error, IntoResponse, Result,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use stripe::{CheckoutSession, EventObject, EventType, Webhook};

use crate::{
    db::Connection,
    entities::{
        credit_deposits, organization_credits, prelude::OrganizationCredits,
        sea_orm_active_enums::DepositReason,
    },
    stripe::Stripe,
    AppContext, AppState, StripeSignature, UserID,
};

const CREDITS_PACK_SIZE: i64 = 100;

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

#[handler]
pub async fn stripe_webhook(
    Data(conn): Data<&Connection>,
    Data(stripe): Data<&Stripe>,
    stripe_signature: StripeSignature,
    body: Body,
) -> Result<()> {
    let payload = body.into_string().await?;

    if let Ok(event) = Webhook::construct_event(
        &payload,
        &stripe_signature.content(),
        &stripe.webhook_secret,
    ) {
        match event.event_type {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    handle_checkout_session_completed(&stripe.client, conn, session).await?;
                }
            },
            _ => {
                info!(
                    "Unknown event encountered in webhook: {:?}",
                    event.event_type
                );
            },
        }
    } else {
        return Err(Error::from_string(
            "failed to construct webhook event, recheck your payload and signature",
            StatusCode::BAD_REQUEST,
        ));
    }

    Ok(())
}

async fn handle_checkout_session_completed(
    client: &stripe::Client,
    db: &Connection,
    session: stripe::CheckoutSession,
) -> Result<()> {
    let conn = db.get();

    let session = CheckoutSession::retrieve(client, &session.id, &["line_items"])
        .await
        .map_err(anyhow::Error::from)?;

    let line_item = session.line_items.data.get(0).ok_or_else(|| {
        Error::from_string(
            "no line items on stripe checkout session",
            StatusCode::NOT_FOUND,
        )
    })?;

    let credits: i64 = line_item
        .quantity
        .ok_or_else(|| {
            Error::from_string("no quantity on stripe line item", StatusCode::NOT_FOUND)
        })?
        .try_into()
        .map_err(anyhow::Error::from)?;

    let stripe::CheckoutSession { amount_total, .. } = session;

    let amount_total = amount_total.ok_or_else(|| {
        Error::from_string(
            "no amount total on stripe checkout session",
            StatusCode::NOT_FOUND,
        )
    })?;

    let organization_id = session
        .metadata
        .get("organization_id")
        .map(|id| Uuid::from_str(id))
        .expect("Missing organization_id in metadata")
        .map_err(|_| Error::from_status(StatusCode::NOT_FOUND))?;
    let user_id = session
        .metadata
        .get("user_id")
        .map(|id| Uuid::from_str(id))
        .expect("Missing user_id in metadata")
        .map_err(|_| Error::from_status(StatusCode::NOT_FOUND))?;

    let organization_credits = OrganizationCredits::find_by_id(organization_id)
        .one(conn)
        .await
        .map_err(anyhow::Error::from)?
        .ok_or_else(|| {
            Error::from_string("organization credits not found", StatusCode::NOT_FOUND)
        })?;

    let credits = credits * CREDITS_PACK_SIZE;

    let credit_balance = organization_credits.balance + credits;
    let pending_credit_balance = organization_credits.pending_balance + credits;

    let mut organization_credits: organization_credits::ActiveModel = organization_credits.into();

    organization_credits.balance = Set(credit_balance);
    organization_credits.pending_balance = Set(pending_credit_balance);

    organization_credits
        .update(conn)
        .await
        .map_err(anyhow::Error::from)?;

    #[allow(clippy::cast_precision_loss)]
    let cost = amount_total as f64 / 100.0;

    let credit_deposit = credit_deposits::ActiveModel {
        organization: Set(organization_id),
        initiated_by: Set(user_id),
        credits: Set(credits),
        reason: Set(DepositReason::Purchased),
        cost: Set(cost),
        ..Default::default()
    };

    credit_deposit
        .insert(conn)
        .await
        .map_err(anyhow::Error::from)?;
    Ok(())
}
