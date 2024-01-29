use cosmwasm_std::Uint128;
use cw721_metadata_onchain::Metadata as Cw721Metadata;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CollectionKind {
    Single {
        image: String,
    },
    Collectible {
        minter: String,
        cover: String,
        public_key: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub nft_code_id: u64,
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub max_token_count: u32,
    pub mint_stages: Vec<MintStage>,
    pub collection_kind: CollectionKind,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ConfigureMintStageMsg {
    Config {
        name: Option<String>,
        start: Option<u64>,
        finish: Option<u64>,
        max_per_user: Option<u16>,
        price: Option<Uint128>,
        whitelist_enabled: Option<bool>,
    },
    Whitelist {
        whitelist: bool,
        candidates: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint {
        stage_id: u8,
        signature: Option<String>,
    },
    Reserve {
        stage_id: u8,
        signature: Option<String>,
    },
    MinterMint {
        token_id: u32,
        metadata: Cw721Metadata,
    },
    Configure {
        name: Option<String>,
        description: Option<String>,
        nft_address: Option<String>,
    },
    ConfigureMintStage {
        id: u8,
        config: ConfigureMintStageMsg,
    },
    WithdrawFunds {
        recipient: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    MintStages {
        limit: Option<u32>,
    },
    MintStage {
        stage_id: u8,
    },
    IsWhitelisted {
        stage_id: u8,
        address: String,
    },
    UnprocessedReservations {
        start_after: Option<u32>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub nft_address: Option<String>,
    pub name: String,
    pub description: String,
    pub collection_kind: CollectionKind,
    pub max_token_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub token_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IsWhitelistedResponse {
    pub whitelisted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintStagesResponse {
    pub mint_stages: Vec<MintStage>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintStage {
    pub id: u8,
    pub name: String,
    pub start: Option<u64>,
    pub finish: Option<u64>,
    pub max_per_user: Option<u16>,
    pub price: Option<Uint128>,
    pub whitelist_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UnprocessedReservationsResponse {
    pub reservations: Vec<u32>,
}
