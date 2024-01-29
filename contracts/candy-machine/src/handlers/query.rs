use crate::msg::{
    ConfigResponse, IsWhitelistedResponse, MintStage, MintStagesResponse, StateResponse,
    UnprocessedReservationsResponse,
};
use cosmwasm_std::{Addr, Deps, Order, StdResult};
use cw_storage_plus::Bound;
use std::convert::TryInto;

use crate::error::ContractError;
use crate::state::collection_kind;
use crate::state::config::{Config, CONFIG};
use crate::state::reservation::UNPROCESSED;
use crate::state::stage::{load as load_mint_stage, STAGE};
use crate::state::state::{State, STATE};
use crate::state::user;

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;

fn address_to_string(address: Option<Addr>) -> Option<String> {
    if let Some(address) = address {
        Some(address.to_string())
    } else {
        None
    }
}

pub fn query_config(deps: Deps) -> Result<ConfigResponse, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        nft_address: address_to_string(config.nft_address),
        name: config.name,
        description: config.description,
        collection_kind: collection_kind::to_msg(config.collection_kind),
        max_token_count: config.max_token_count,
    })
}

pub fn query_state(deps: Deps) -> Result<StateResponse, ContractError> {
    let state: State = STATE.load(deps.storage)?;
    Ok(StateResponse {
        token_count: state.token_count,
    })
}

pub fn query_mint_stages(
    deps: Deps,
    limit: Option<u32>,
) -> Result<MintStagesResponse, ContractError> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let mint_stages: StdResult<Vec<MintStage>> = STAGE
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect();

    Ok(MintStagesResponse {
        mint_stages: mint_stages?,
    })
}

pub fn query_mint_stage(deps: Deps, stage_id: u8) -> Result<MintStage, ContractError> {
    let mint_stage =
        load_mint_stage(deps.storage, stage_id).ok_or(ContractError::UnknownMintStage {})?;

    Ok(mint_stage)
}

pub fn is_whitelisted(
    deps: Deps,
    stage_id: u8,
    address: String,
) -> Result<IsWhitelistedResponse, ContractError> {
    let user_addr = deps.api.addr_canonicalize(address.as_str())?;
    let whitelisted = user::is_whitelisted(deps.storage, stage_id, &user_addr);

    Ok(IsWhitelistedResponse { whitelisted })
}

pub fn query_unprocessed_reservations(
    deps: Deps,
    start_after: Option<u32>,
    limit: Option<u32>,
) -> Result<UnprocessedReservationsResponse, ContractError> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive_int);

    let reservations: Vec<u32> = UNPROCESSED
        .keys(deps.storage, start, None, Order::Descending)
        .map(|key| u32::from_be_bytes(key.try_into().unwrap()))
        .take(limit)
        .collect();

    Ok(UnprocessedReservationsResponse { reservations })
}
