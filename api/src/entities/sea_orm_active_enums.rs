//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use async_graphql::*;
use hub_core::credits::Blockchain as CoreBlockchain;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{credits, Actions};

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Enum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "action")]
pub enum Action {
    #[sea_orm(string_value = "create_drop")]
    CreateDrop,
    #[sea_orm(string_value = "create_wallet")]
    CreateWallet,
    #[sea_orm(string_value = "mint_edition")]
    MintEdition,
    #[sea_orm(string_value = "retry_drop")]
    RetryDrop,
    #[sea_orm(string_value = "retry_mint")]
    RetryMint,
    #[sea_orm(string_value = "transfer_asset")]
    TransferAsset,
    #[sea_orm(string_value = "create_collection")]
    CreateCollection,
    #[sea_orm(string_value = "retry_collection")]
    RetryCollection,
    #[sea_orm(string_value = "mint")]
    Mint,
    #[sea_orm(string_value = "mint_compressed")]
    MintCompressed,
    #[sea_orm(string_value = "update_mint")]
    UpdateMint,
    #[sea_orm(string_value = "switch_collection")]
    SwitchCollection,
}

impl From<Action> for credits::Action {
    fn from(v: Action) -> Self {
        match v {
            Action::CreateDrop => Self::CreateDrop,
            Action::MintEdition => Self::MintEdition,
            Action::RetryMint => Self::RetryMint,
            Action::TransferAsset => Self::TransferAsset,
            Action::CreateWallet => Self::CreateWallet,
            Action::RetryDrop => Self::RetryDrop,
            Action::CreateCollection => Self::CreateCollection,
            Action::RetryCollection => Self::RetryCollection,
            Action::Mint => Self::Mint,
            Action::MintCompressed => Self::MintCompressed,
            Action::UpdateMint => Self::UpdateMint,
            Action::SwitchCollection => Self::SwitchCollection,
        }
    }
}

impl From<Actions> for Action {
    fn from(v: Actions) -> Self {
        match v {
            Actions::CreateDrop => Self::CreateDrop,
            Actions::MintEdition => Self::MintEdition,
            Actions::RetryMint => Self::RetryMint,
            Actions::TransferAsset => Self::TransferAsset,
            Actions::CreateWallet => Self::CreateWallet,
            Actions::RetryDrop => Self::RetryDrop,
            Actions::CreateCollection => Self::CreateCollection,
            Actions::RetryCollection => Self::RetryCollection,
            Actions::Mint => Self::Mint,
            Actions::MintCompressed => Self::MintCompressed,
            Actions::UpdateMint => Self::UpdateMint,
            Actions::SwitchCollection => Self::SwitchCollection,
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Enum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "blockchain")]
pub enum Blockchain {
    #[sea_orm(string_value = "ethereum")]
    Ethereum,
    #[sea_orm(string_value = "polygon")]
    Polygon,
    #[sea_orm(string_value = "solana")]
    Solana,
}

impl From<Blockchain> for credits::Blockchain {
    fn from(v: Blockchain) -> Self {
        match v {
            Blockchain::Solana => Self::Solana,
            Blockchain::Polygon => Self::Polygon,
            Blockchain::Ethereum => Self::Ethereum,
        }
    }
}

impl TryFrom<CoreBlockchain> for Blockchain {
    type Error = Error;
    fn try_from(v: CoreBlockchain) -> Result<Self> {
        match v {
            CoreBlockchain::Solana => Ok(Self::Solana),
            CoreBlockchain::Polygon => Ok(Self::Polygon),
            CoreBlockchain::Ethereum => Ok(Self::Ethereum),
            CoreBlockchain::OffChain => Err(Error::new("blockchain unspecified")),
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Enum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "deduction_status")]
pub enum DeductionStatus {
    #[sea_orm(string_value = "confirmed")]
    Confirmed,
    #[sea_orm(string_value = "pending")]
    Pending,
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Enum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "deposit_reason")]
pub enum DepositReason {
    #[sea_orm(string_value = "gifted")]
    Gifted,
    #[sea_orm(string_value = "purchased")]
    Purchased,
}
