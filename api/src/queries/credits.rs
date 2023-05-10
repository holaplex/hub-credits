use std::collections::HashMap;

use async_graphql::{Context, Object, Result, SimpleObject};
use hub_core::credits::CreditsClient;

use crate::{
    entities::sea_orm_active_enums::{Action, Blockchain},
    Actions,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "CreditsQuery")]
impl Query {
    /// Returns a list of `ActionCost` which represents the cost of each action on different blockchains.
    ///
    /// # Errors
    /// This function fails if it fails to get `CreditsClient` or if blockchain enum conversion fails.
    async fn credit_sheet(&self, ctx: &Context<'_>) -> Result<Vec<ActionCost>> {
        let credits = ctx.data::<CreditsClient<Actions>>()?;

        let sheet = credits.credit_sheet();

        let mut action_map: HashMap<Actions, Vec<BlockchainCost>> = HashMap::new();
        for ((action, blockchain), value) in sheet {
            action_map
                .entry(*action)
                .or_insert_with(Vec::new)
                .push(BlockchainCost {
                    blockchain: (*blockchain).try_into()?,
                    credits: *value,
                });
        }

        let res = action_map
            .into_iter()
            .map(|(action, blockchains)| ActionCost {
                action: action.into(),
                blockchains,
            })
            .collect();

        Ok(res)
    }
}

/// Represents the cost of performing a certain action on different blockchains
#[derive(Debug, Clone, SimpleObject)]
pub struct ActionCost {
    /// enum that represents the type of action being performed.
    pub action: Action,
    /// a vector of BlockchainCost structs that represents the cost of performing the action on each blockchain.
    pub blockchains: Vec<BlockchainCost>,
}

/// Represents the cost of performing an action on a specific blockchain
#[derive(Debug, Clone, SimpleObject)]
pub struct BlockchainCost {
    /// enum that represents the blockchain on which the action is being performed.
    pub blockchain: Blockchain,
    /// represents the cost in credits for performing the action on the blockchain.
    pub credits: u64,
}
