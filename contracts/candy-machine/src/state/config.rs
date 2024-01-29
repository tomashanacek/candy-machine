use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::collection_kind::CollectionKind;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub nft_address: Option<Addr>,
    pub name: String,
    pub description: String,
    pub max_token_count: u32,
    pub collection_kind: CollectionKind,
}

pub const CONFIG: Item<Config> = Item::new("config");
