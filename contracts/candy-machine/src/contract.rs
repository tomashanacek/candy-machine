#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdError,
    StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw721_metadata_onchain::InstantiateMsg as NftInstantiateMsg;
use protobuf::Message;
use serde::Serialize;

use crate::error::ContractError;
use crate::handlers::configure;
use crate::handlers::mint;
use crate::handlers::query;
use crate::handlers::withdraw;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::response::MsgInstantiateContractResponse;
use crate::state::collection_kind;
use crate::state::config::{Config, CONFIG};
use crate::state::stage;
use crate::state::state::{State, STATE};

// version info for migration info
pub const CONTRACT_NAME: &str = "crates.io:candy-machine";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: info.sender.clone(),
        name: msg.name.clone(),
        description: msg.description,
        collection_kind: collection_kind::to_raw(msg.collection_kind, deps.api)?,
        nft_address: None,
        max_token_count: msg.max_token_count,
    };
    CONFIG.save(deps.storage, &config)?;

    let state = State { token_count: 0 };
    STATE.save(deps.storage, &state)?;

    msg.mint_stages
        .iter()
        .try_for_each(|mint_stage| stage::store(deps.storage, mint_stage.id, &mint_stage))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_submessage(SubMsg {
            msg: WasmMsg::Instantiate {
                admin: None,
                code_id: msg.nft_code_id,
                msg: to_binary(&NftInstantiateMsg {
                    name: msg.name,
                    symbol: msg.symbol,
                    minter: env.contract.address.to_string(),
                })?,
                funds: vec![],
                label: "".to_string(),
            }
            .into(),
            gas_limit: None,
            id: INSTANTIATE_REPLY_ID,
            reply_on: ReplyOn::Success,
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let data = msg.result.unwrap().data.unwrap();
    let res: MsgInstantiateContractResponse =
        Message::parse_from_bytes(data.as_slice()).map_err(|_| {
            StdError::parse_err("MsgInstantiateContractResponse", "failed to parse data")
        })?;
    let nft_token = res.get_contract_address();

    let api = deps.api;
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.nft_address = Some(api.addr_validate(nft_token)?);
        Ok(config)
    })?;

    Ok(Response::new().add_attribute("nft_token_addr", nft_token))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {
            stage_id,
            signature,
        } => mint::mint(deps, env, info, stage_id, signature),
        ExecuteMsg::Reserve {
            stage_id,
            signature,
        } => mint::mint(deps, env, info, stage_id, signature),
        ExecuteMsg::MinterMint { token_id, metadata } => {
            mint::minter_mint(deps, info, token_id, metadata)
        }
        ExecuteMsg::ConfigureMintStage { id, config } => {
            configure::configure_mint_stage(deps, info, id, config)
        }
        ExecuteMsg::WithdrawFunds { recipient } => {
            withdraw::withdraw_funds(deps, env, info, recipient)
        }
        ExecuteMsg::Configure {
            name,
            description,
            nft_address,
        } => configure::configure(deps, info, name, description, nft_address),
    }
}

pub fn result_to_binary<T>(res: Result<T, ContractError>) -> Result<Binary, ContractError>
where
    T: Serialize,
{
    match res {
        Ok(data) => Ok(to_binary(&data)?),
        Err(error) => Err(error),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Config {} => result_to_binary(query::query_config(deps)),
        QueryMsg::State {} => result_to_binary(query::query_state(deps)),
        QueryMsg::MintStages { limit } => result_to_binary(query::query_mint_stages(deps, limit)),
        QueryMsg::MintStage { stage_id } => {
            result_to_binary(query::query_mint_stage(deps, stage_id))
        }
        QueryMsg::IsWhitelisted { stage_id, address } => {
            result_to_binary(query::is_whitelisted(deps, stage_id, address))
        }
        QueryMsg::UnprocessedReservations { start_after, limit } => result_to_binary(
            query::query_unprocessed_reservations(deps, start_after, limit),
        ),
    }
}
