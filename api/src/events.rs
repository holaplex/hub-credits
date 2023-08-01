use std::ops::Sub;

use hub_core::{prelude::*, producer::Producer, uuid::Uuid};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::{
    credits::{credits_event, Action, Blockchain, Credits, CreditsEvent, CreditsEventKey},
    db::Connection,
    entities::{
        credit_deductions, credit_deposits, organization_credits,
        sea_orm_active_enums::{self, DeductionStatus, DepositReason},
    },
    proto::{credits_mpsc_event, organization_events, Organization, OrganizationEventKey},
    Services,
};
/// This function matches each event type and processes it.
///
/// # Errors
/// This function fails if it is unable to process any event
#[allow(clippy::too_many_lines)]
pub async fn process(
    msg: Services,
    db: Connection,
    producer: Producer<CreditsEvent>,
    gift_amount: u64,
) -> Result<()> {
    // match topics
    match msg {
        Services::Organizations(key, e) => match e.event {
            Some(organization_events::Event::OrganizationCreated(org)) => {
                deposit_gifted_credits(db, key, org, gift_amount).await
            },
            Some(_) | None => Ok(()),
        },
        Services::CreditsMpsc(key, e) => match e.event {
            Some(credits_mpsc_event::Event::PendingDeduction(c)) => {
                deduct_credits(db, key, c).await
            },
            Some(credits_mpsc_event::Event::ConfirmDeduction(_)) => {
                confirm_credit_deduction(db, producer, key).await
            },
            None => Ok(()),
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

    let org_credits = organization_credits::ActiveModel {
        id: Set(org),
        balance: Set(gift_amount.try_into()?),
        pending_balance: Set(gift_amount.try_into()?),
        updated_at: Set(None),
        ..Default::default()
    };

    org_credits.insert(db.get()).await?;

    let deposit = credit_deposits::ActiveModel {
        organization: Set(org),
        initiated_by: Set(initiated_by),
        credits: Set(gift_amount.try_into()?),
        cost: Set(0.0),
        reason: Set(DepositReason::Gifted),
        ..Default::default()
    };

    deposit.insert(db.get()).await?;

    Ok(())
}

async fn deduct_credits(db: Connection, key: CreditsEventKey, c: Credits) -> Result<()> {
    let CreditsEventKey { id, user_id } = key.clone();

    let Credits {
        credits,
        action,
        blockchain,
        organization,
    } = c;

    let org_id = Uuid::from_str(&organization)?;
    let id = Uuid::from_str(&id)?;
    let user_id = Uuid::from_str(&user_id)?;

    let org_credits_model = organization_credits::Entity::find_by_id(org_id)
        .one(db.get())
        .await?
        .ok_or_else(|| anyhow!("No organization found with id {}", org_id))?;
    let mut org_credits: organization_credits::ActiveModel = org_credits_model.clone().into();

    org_credits.pending_balance = Set(org_credits_model.pending_balance.sub(credits));

    let action = Action::from_i32(action).ok_or_else(|| anyhow!("Invalid action: {}", action))?;
    let blockchain = Blockchain::from_i32(blockchain)
        .ok_or_else(|| anyhow!("Unsupported blockchain: {}", blockchain))?;

    let credit_deductions = credit_deductions::ActiveModel {
        id: Set(id),
        organization: Set(org_id),
        initiated_by: Set(user_id),
        credits: Set(credits),
        action: Set(action.try_into()?),
        blockchain: Set(blockchain.try_into()?),
        status: Set(DeductionStatus::Pending),
        ..Default::default()
    };

    credit_deductions.insert(db.get()).await?;
    org_credits.update(db.get()).await?;

    Ok(())
}

async fn confirm_credit_deduction(
    db: Connection,
    producer: Producer<CreditsEvent>,
    key: CreditsEventKey,
) -> Result<()> {
    let CreditsEventKey { id, .. } = key.clone();

    let id = Uuid::from_str(&id)?;

    let credit_deductions_model = credit_deductions::Entity::find_by_id(id)
        .one(db.get())
        .await?
        .ok_or_else(|| anyhow!("No credit deduction found with id {}", id))?;

    let mut credit_deductions: credit_deductions::ActiveModel =
        credit_deductions_model.clone().into();
    credit_deductions.status = Set(DeductionStatus::Confirmed);

    let org_credits_model =
        organization_credits::Entity::find_by_id(credit_deductions_model.organization)
            .one(db.get())
            .await?
            .ok_or_else(|| {
                anyhow!(
                    "No organization credits found for {}",
                    credit_deductions_model.organization
                )
            })?;

    let mut org_credits: organization_credits::ActiveModel = org_credits_model.clone().into();

    org_credits.balance = Set(org_credits_model
        .balance
        .sub(credit_deductions_model.credits));

    org_credits.update(db.get()).await?;
    credit_deductions.update(db.get()).await?;

    let blockchain: Blockchain = credit_deductions_model.blockchain.into();
    let action: Action = credit_deductions_model.action.into();

    let event = CreditsEvent {
        event: Some(credits_event::Event::CreditsDeducted(Credits {
            credits: credit_deductions_model.credits,
            blockchain: blockchain.into(),
            action: action.into(),
            organization: credit_deductions_model.organization.to_string(),
        })),
    };

    producer.send(Some(&event), Some(&key)).await?;
    Ok(())
}

impl TryFrom<Action> for sea_orm_active_enums::Action {
    type Error = Error;

    fn try_from(v: Action) -> Result<Self> {
        match v {
            Action::Unspecified => Err(anyhow!("Invalid enum variant")),
            Action::CreateDrop => Ok(Self::CreateDrop),
            Action::MintEdition => Ok(Self::MintEdition),
            Action::TransferAsset => Ok(Self::TransferAsset),
            Action::RetryMint => Ok(Self::RetryMint),
            Action::CreateWallet => Ok(Self::CreateWallet),
            Action::RetryDrop => Ok(Self::RetryDrop),
            Action::CreateCollection => Ok(Self::CreateCollection),
            Action::RetryCollection => Ok(Self::RetryCollection),
            Action::Mint => Ok(Self::Mint),
            Action::MintCompressed => Ok(Self::MintCompressed),
        }
    }
}

impl TryFrom<Blockchain> for sea_orm_active_enums::Blockchain {
    type Error = Error;

    fn try_from(v: Blockchain) -> Result<Self> {
        match v {
            Blockchain::Unspecified => Err(anyhow!("Invalid enum variant")),
            Blockchain::Solana => Ok(Self::Solana),
            Blockchain::Polygon => Ok(Self::Polygon),
            Blockchain::Ethereum => Ok(Self::Ethereum),
        }
    }
}
