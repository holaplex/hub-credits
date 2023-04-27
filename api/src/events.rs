use hub_core::{prelude::*, uuid::Uuid};
use sea_orm::{ActiveModelTrait, Set};

use crate::{
    db::Connection,
    entities::{credit_deposits, organization_credits, sea_orm_active_enums::DepositReason},
    proto::{organization_events, Organization, OrganizationEventKey},
    Services,
};
/// This function matches each event type and processes it.
///
/// # Errors
/// This function fails if it is unable to process any event
#[allow(clippy::too_many_lines)]
pub async fn process(msg: Services, db: Connection, gift_amount: u64) -> Result<()> {
    // match topics
    match msg {
        Services::Organizations(key, e) => match e.event {
            Some(organization_events::Event::OrganizationCreated(org)) => {
                deposit_gifted_credits(db, key, org, gift_amount).await
            },
            Some(_) | None => Ok(()),
        },
    }
}

/// Deposits a given `gift_amount` of credits by creating a `credit_deposit` record.
/// Creates a new `organization_credits` record with the gifted credit balance and inserts it into the database
/// If any of these operations fail, the function returns an error.
async fn deposit_gifted_credits(
    db: Connection,
    key: OrganizationEventKey,
    org: Organization,
    gift_amount: u64,
) -> Result<()> {
    let org: Uuid = Uuid::from_str(&org.id)?;
    let initiated_by = Uuid::from_str(&key.user_id)?;

    let deposit = credit_deposits::ActiveModel {
        organization: Set(org),
        initiated_by: Set(initiated_by),
        credits: Set(gift_amount.try_into()?),
        cost: Set(0.0),
        reason: Set(DepositReason::Gifted),
        ..Default::default()
    };

    deposit.insert(db.get()).await?;

    let org_credits = organization_credits::ActiveModel {
        id: Set(org),
        balance: Set(gift_amount.try_into()?),
        updated_at: Set(None),
        ..Default::default()
    };

    org_credits.insert(db.get()).await?;

    Ok(())
}
