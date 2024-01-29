use crate::msg::CollectionKind as CollectionKindMsg;
use cosmwasm_std::{Addr, Api, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CollectionKind {
    Single {
        image: String,
    },
    Collectible {
        minter: Addr,
        cover: String,
        public_key: Option<String>,
    },
}

pub fn to_raw(msg: CollectionKindMsg, api: &dyn Api) -> StdResult<CollectionKind> {
    let raw = match msg {
        CollectionKindMsg::Single { image } => CollectionKind::Single { image },
        CollectionKindMsg::Collectible {
            minter,
            public_key,
            cover,
        } => CollectionKind::Collectible {
            minter: api.addr_validate(minter.as_str())?,
            cover,
            public_key,
        },
    };
    Ok(raw)
}

pub fn to_msg(raw: CollectionKind) -> CollectionKindMsg {
    match raw {
        CollectionKind::Single { image } => CollectionKindMsg::Single { image },
        CollectionKind::Collectible {
            minter,
            public_key,
            cover,
        } => CollectionKindMsg::Collectible {
            minter: minter.to_string(),
            public_key,
            cover,
        },
    }
}
