use crate::msg::{ConfigureMintStageMsg, MintStage};
use cosmwasm_std::{attr, Addr, DepsMut, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::state::config::{Config, CONFIG};
use crate::state::stage::{load as load_mint_stage, store as store_mint_stage};
use crate::state::user;

pub fn configure(
    deps: DepsMut,
    info: MessageInfo,
    name: Option<String>,
    description: Option<String>,
    nft_address: Option<String>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(name) = name {
        config.name = name;
    }

    if let Some(description) = description {
        config.description = description;
    }

    if let Some(nft_address) = nft_address {
        let validated_nft_address: Addr = deps.api.addr_validate(&nft_address)?;
        config.nft_address = Some(validated_nft_address);
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "configure"),
        attr("sender", info.sender.to_string()),
    ]))
}

pub fn configure_mint_stage(
    deps: DepsMut,
    info: MessageInfo,
    stage_id: u8,
    msg: ConfigureMintStageMsg,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    match msg {
        ConfigureMintStageMsg::Config {
            name,
            start,
            finish,
            price,
            max_per_user,
            whitelist_enabled,
        } => update_mint_stage(
            deps,
            info,
            stage_id,
            name,
            start,
            finish,
            price,
            max_per_user,
            whitelist_enabled,
        ),
        ConfigureMintStageMsg::Whitelist {
            whitelist,
            candidates,
        } => update_whitelist(deps, info, stage_id, whitelist, candidates),
    }
}

pub fn update_whitelist(
    deps: DepsMut,
    info: MessageInfo,
    stage_id: u8,
    whitelist: bool,
    candidates: Vec<String>,
) -> Result<Response, ContractError> {
    let api = deps.api;
    let storage = deps.storage;

    load_mint_stage(storage, stage_id).ok_or(ContractError::UnknownMintStage {})?;

    candidates
        .iter()
        .map(|x| api.addr_canonicalize(x.as_str()).unwrap())
        .for_each(|candidate| match whitelist {
            true => user::register_whitelist(storage, stage_id, &candidate).unwrap(),
            false => user::unregister_whitelist(storage, stage_id, &candidate).unwrap(),
        });

    Ok(Response::new().add_attributes(vec![
        attr("action", "whitelist_user"),
        attr("sender", info.sender.to_string()),
    ]))
}

pub fn update_mint_stage(
    deps: DepsMut,
    info: MessageInfo,
    stage_id: u8,
    name: Option<String>,
    start: Option<u64>,
    finish: Option<u64>,
    price: Option<Uint128>,
    max_per_user: Option<u16>,
    whitelist_enabled: Option<bool>,
) -> Result<Response, ContractError> {
    let mut stage: MintStage =
        load_mint_stage(deps.storage, stage_id).ok_or(ContractError::UnknownMintStage {})?;

    if let Some(name) = name {
        stage.name = name;
    }

    if let Some(start) = start {
        stage.start = Some(start);
    }

    if let Some(finish) = finish {
        stage.finish = Some(finish);
    }

    if let Some(price) = price {
        stage.price = Some(price);
    }

    if let Some(max_per_user) = max_per_user {
        stage.max_per_user = Some(max_per_user);
    }

    if let Some(whitelist_enabled) = whitelist_enabled {
        stage.whitelist_enabled = whitelist_enabled;
    }

    store_mint_stage(deps.storage, stage_id, &stage)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "configure_mint_stage"),
        attr("sender", info.sender.to_string()),
    ]))
}
