use crate::msg::MintStage;
use base64;
use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,
};
use cw721_metadata_onchain::{ExecuteMsg as Cw721ExecuteMsg, Metadata, MintMsg};
use sha2::{Digest, Sha256};

use crate::error::ContractError;
use crate::state::collection_kind::CollectionKind;
use crate::state::config::{Config, CONFIG};
use crate::state::reservation;
use crate::state::stage::load as load_mint_stage;
use crate::state::state::{State, STATE};
use crate::state::user::{is_whitelisted, load as load_user, store as store_user};

pub fn minter_mint(
    deps: DepsMut,
    info: MessageInfo,
    token_id: u32,
    metadata: Metadata,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let minter = match config.collection_kind {
        CollectionKind::Single { image: _ } => Err(ContractError::InvalidCollectionKind {}),
        CollectionKind::Collectible {
            public_key: _,
            cover: _,
            minter,
        } => Ok(minter),
    }?;

    if info.sender != minter {
        return Err(ContractError::Unauthorized {});
    }

    let user_reservation =
        reservation::load(deps.storage, token_id).ok_or(ContractError::UnknownReservation {})?;
    reservation::remove_unprocessed(deps.storage, token_id);

    let mint_msg = MintMsg {
        token_id: token_id.to_string(),
        owner: user_reservation.user_address.to_string(),
        token_uri: None,
        extension: Some(metadata),
    };

    let resp = Response::default()
        .add_attribute("action", "minter_mint")
        .add_attribute("owner", user_reservation.user_address.to_string())
        .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.nft_address.unwrap().to_string(),
            msg: to_binary(&Cw721ExecuteMsg::Mint(mint_msg))?,
            funds: vec![],
        })]);

    Ok(resp)
}

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    stage_id: u8,
    signature: Option<String>,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let mint_stage: MintStage =
        load_mint_stage(deps.storage, stage_id).ok_or(ContractError::UnknownMintStage {})?;

    // check mint time frame
    let now = env.block.time.seconds();

    if let Some(start) = mint_stage.start {
        if now < start {
            return Err(ContractError::MintNotStarted { start: start });
        }
    }

    if let Some(finish) = mint_stage.finish {
        if finish < now {
            return Err(ContractError::MintFinished { finish: finish });
        }
    }
    // check if nft address is set
    config
        .nft_address
        .clone()
        .ok_or(ContractError::NftAddressNotDefined {})?;

    // check if there are nfts available to mint
    let mut state: State = STATE.load(deps.storage)?;
    if state.token_count >= config.max_token_count {
        return Err(ContractError::NoMoreNftsToMint {});
    }

    // check price
    if let Some(price) = mint_stage.price {
        let amount = info
            .funds
            .iter()
            .find(|c| c.denom == "uusd".to_string())
            .map(|c| c.amount)
            .unwrap_or_else(Uint128::zero);
        if amount.is_zero() {
            return Err(ContractError::NotAllowZeroAmount {});
        }
        if info.funds.len() > 1 {
            return Err(ContractError::NotAllowOtherDenoms {
                denom: "uusd".to_string(),
            });
        }
        if price != amount {
            return Err(ContractError::InvalidAmount { amount: price });
        }
    }

    // check if user is allowed to mint
    let sender = &deps.api.addr_canonicalize(info.sender.as_str())?;
    let user_minted_amount = load_user(deps.storage, sender);

    if mint_stage.whitelist_enabled && !is_whitelisted(deps.storage, stage_id, sender) {
        return Err(ContractError::NotAllowNonWhitelisted {
            address: info.sender.to_string(),
        });
    }

    if let Some(max_per_user) = mint_stage.max_per_user {
        if user_minted_amount >= max_per_user {
            return Err(ContractError::MaximumMintAmountPerUserExceeded {});
        }
    }

    // update user state and global state
    store_user(deps.storage, sender, user_minted_amount + 1)?;
    state.token_count += 1;
    STATE.save(deps.storage, &state)?;

    // mint
    match config.collection_kind.clone() {
        CollectionKind::Single { image } => {
            mint_single(info.sender.to_string(), state.token_count, config, image)
        }
        CollectionKind::Collectible {
            minter: _,
            cover: _,
            public_key,
        } => mint_collectible(
            deps,
            info.sender.clone(),
            state.token_count,
            public_key,
            signature,
        ),
    }
}

fn mint_collectible(
    deps: DepsMut,
    owner: Addr,
    token_count: u32,
    public_key: Option<String>,
    signature: Option<String>,
) -> Result<Response, ContractError> {
    if let Some(public_key_str) = public_key {
        let hash = Sha256::digest(owner.as_bytes());

        let public_key = base64::decode(&public_key_str)?;

        let signature_msg = signature.ok_or(ContractError::InvalidSignature {})?;
        let signature = base64::decode(&signature_msg)?;

        let result = deps
            .api
            .secp256k1_verify(&hash, &signature, public_key.as_ref())?;

        if !result {
            return Err(ContractError::InvalidSignature {});
        }
    }

    reservation::store(
        deps.storage,
        token_count,
        &reservation::Reservation {
            user_address: owner.clone(),
            token_id: token_count,
        },
    )?;
    reservation::store_unprocessed(deps.storage, token_count, &owner)?;

    Ok(Response::default()
        .add_attribute("action", "reserve")
        .add_attribute("owner", owner.as_str()))
}

fn mint_single(
    owner: String,
    token_count: u32,
    config: Config,
    image: String,
) -> Result<Response, ContractError> {
    let mint_msg = MintMsg {
        token_id: token_count.to_string(),
        owner: owner.clone(),
        token_uri: None,
        extension: Some(Metadata {
            image: Some(image),
            description: Some(config.description),
            name: Some(format!("{} #{}", config.name, token_count)),
            attributes: None,
            ..Metadata::default()
        }),
    };

    let resp = Response::default()
        .add_attribute("action", "mint")
        .add_attribute("owner", owner)
        .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.nft_address.unwrap().to_string(),
            msg: to_binary(&Cw721ExecuteMsg::Mint(mint_msg))?,
            funds: vec![],
        })]);

    Ok(resp)
}
