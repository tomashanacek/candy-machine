use crate::msg::{ExecuteMsg, InstantiateMsg, MintStage, QueryMsg, StateResponse};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{
    attr, coins, from_binary, to_binary, CosmosMsg, Env, MessageInfo, Response, SubMsg, Timestamp,
    Uint128, WasmMsg,
};
use cw721_metadata_onchain::{ExecuteMsg as Cw721ExecuteMsg, Metadata, MintMsg};

use crate::contract::{execute, query};
use crate::error::ContractError;
use crate::testing::configure;
use crate::testing::configure_mint_stage;
use crate::testing::instantiate;
use crate::testing::{
    mock_deps, MockDeps, TEST_BASE_DENOM, TEST_NFT_ADDRESS, TEST_NFT_DESCRIPTION, TEST_NFT_IMAGE,
    TEST_NFT_NAME, TEST_STAGE_ID, TEST_USER_1,
};

pub fn exec(deps: &mut MockDeps, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    execute(
        deps.as_mut(),
        env,
        info,
        ExecuteMsg::Mint {
            stage_id: TEST_STAGE_ID,
            signature: None,
        },
    )
}

#[test]
fn success_mint_public_free() {
    let mut deps = mock_deps();
    let env = mock_env();

    instantiate::default(&mut deps);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    let info = mock_info(TEST_USER_1, &[]);
    let res = exec(&mut deps, mock_env(), info).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "mint"),
            attr("owner", TEST_USER_1.to_string()),
        ]
    );

    let mint_msg = MintMsg {
        token_id: "1".to_string(),
        owner: TEST_USER_1.to_string(),
        token_uri: None,
        extension: Some(Metadata {
            image: Some(TEST_NFT_IMAGE.to_string()),
            description: Some(TEST_NFT_DESCRIPTION.into()),
            name: Some(format!("{} #{}", TEST_NFT_NAME, 1)),
            attributes: None,
            ..Metadata::default()
        }),
    };
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: TEST_NFT_ADDRESS.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::Mint(mint_msg)).unwrap(),
            funds: vec![],
        }))]
    );

    // should increase counter by 1
    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();
    let state: StateResponse = from_binary(&res).unwrap();
    assert_eq!(1, state.token_count);
}

#[test]
fn success_mint_public_with_price() {
    let mut deps = mock_deps();
    let amount = 100 * TEST_BASE_DENOM;
    let mint_stage = MintStage {
        price: Some(Uint128::from(amount)),
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    let info = mock_info(TEST_USER_1, &coins(amount, "uusd"));
    let res = exec(&mut deps, mock_env(), info).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "mint"),
            attr("owner", TEST_USER_1.to_string()),
        ]
    );

    let mint_msg = MintMsg {
        token_id: "1".to_string(),
        owner: TEST_USER_1.to_string(),
        token_uri: None,
        extension: Some(Metadata {
            image: Some(TEST_NFT_IMAGE.to_string()),
            description: Some(TEST_NFT_DESCRIPTION.into()),
            name: Some(format!("{} #{}", TEST_NFT_NAME, 1)),
            attributes: None,
            ..Metadata::default()
        }),
    };
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: TEST_NFT_ADDRESS.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::Mint(mint_msg)).unwrap(),
            funds: vec![],
        }))]
    );

    // should increase counter by 1
    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();
    let state: StateResponse = from_binary(&res).unwrap();
    assert_eq!(1, state.token_count);
}

#[test]
fn success_mint_private_free() {
    let mut deps = mock_deps();
    let mint_stage = MintStage {
        whitelist_enabled: true,
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();
    configure_mint_stage::update_whitelist(&mut deps, env.clone()).unwrap();

    let info = mock_info(TEST_USER_1, &[]);
    let res = exec(&mut deps, mock_env(), info).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "mint"),
            attr("owner", TEST_USER_1.to_string()),
        ]
    );

    let mint_msg = MintMsg {
        token_id: "1".to_string(),
        owner: TEST_USER_1.to_string(),
        token_uri: None,
        extension: Some(Metadata {
            image: Some(TEST_NFT_IMAGE.to_string()),
            description: Some(TEST_NFT_DESCRIPTION.into()),
            name: Some(format!("{} #{}", TEST_NFT_NAME, 1)),
            attributes: None,
            ..Metadata::default()
        }),
    };
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: TEST_NFT_ADDRESS.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::Mint(mint_msg)).unwrap(),
            funds: vec![],
        }))]
    );

    // should increase counter by 1
    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();
    let state: StateResponse = from_binary(&res).unwrap();
    assert_eq!(1, state.token_count);
}

#[test]
fn success_mint_private_with_price() {
    let mut deps = mock_deps();

    let amount = 100 * TEST_BASE_DENOM;

    let mint_stage = MintStage {
        whitelist_enabled: true,
        price: Some(Uint128::from(amount)),
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();
    configure_mint_stage::update_whitelist(&mut deps, env.clone()).unwrap();

    let info = mock_info(TEST_USER_1, &coins(amount, "uusd"));
    let res = exec(&mut deps, mock_env(), info).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "mint"),
            attr("owner", TEST_USER_1.to_string()),
        ]
    );

    let mint_msg = MintMsg {
        token_id: "1".to_string(),
        owner: TEST_USER_1.to_string(),
        token_uri: None,
        extension: Some(Metadata {
            image: Some(TEST_NFT_IMAGE.to_string()),
            description: Some(TEST_NFT_DESCRIPTION.into()),
            name: Some(format!("{} #{}", TEST_NFT_NAME, 1)),
            attributes: None,
            ..Metadata::default()
        }),
    };
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: TEST_NFT_ADDRESS.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::Mint(mint_msg)).unwrap(),
            funds: vec![],
        }))]
    );

    // should increase counter by 1
    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();
    let state: StateResponse = from_binary(&res).unwrap();
    assert_eq!(1, state.token_count);
}

#[test]
fn fail_swap_not_started() {
    let mut deps = mock_deps();
    instantiate::default(&mut deps);
    let default_msg = instantiate::default_mint_stage();

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(default_msg.start.unwrap() - 1);
    match exec(&mut deps, env, mock_info(TEST_USER_1, &[])) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::MintNotStarted { start }) => {
            assert_eq!(start, default_msg.start.unwrap())
        }
        Err(e) => panic!("Unexpected error {:?}", e),
    }
}

#[test]
fn fail_swap_finished() {
    let mut deps = mock_deps();
    instantiate::default(&mut deps);
    let default_msg = instantiate::default_mint_stage();

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(default_msg.finish.unwrap() + 1);
    match exec(&mut deps, env, mock_info(TEST_USER_1, &[])) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::MintFinished { finish }) => {
            assert_eq!(finish, default_msg.finish.unwrap())
        }
        Err(e) => panic!("Unexpected error {:?}", e),
    }
}

#[test]
fn fail_nft_address_not_defined() {
    let mut deps = mock_deps();

    let env = mock_env();

    instantiate::default(&mut deps);

    match exec(&mut deps, env, mock_info(TEST_USER_1, &[])) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::NftAddressNotDefined {}) => (),
        Err(e) => panic!("Unexpected error {:?}", e),
    }
}

#[test]
fn fail_no_more_nfts_to_mint() {
    let mut deps = mock_deps();
    let init_msg = InstantiateMsg {
        max_token_count: 0,
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    match exec(&mut deps, env, mock_info(TEST_USER_1, &[])) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::NoMoreNftsToMint {}) => (),
        Err(e) => panic!("Unexpected error {:?}", e),
    }
}

#[test]
fn fail_maximum_mint_amount_per_user_exceeded() {
    let mut deps = mock_deps();
    let mint_stage = MintStage {
        max_per_user: Some(0),
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    match exec(&mut deps, env, mock_info(TEST_USER_1, &[])) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::MaximumMintAmountPerUserExceeded {}) => (),
        Err(e) => panic!("Unexpected error {:?}", e),
    }
}

#[test]
fn fail_zero_amount_not_allowed() {
    let mut deps = mock_deps();
    let amount = 100 * TEST_BASE_DENOM;
    let mint_stage = MintStage {
        price: Some(Uint128::from(amount)),
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    match exec(&mut deps, env, mock_info(TEST_USER_1, &[])) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::NotAllowZeroAmount {}) => (),
        Err(e) => panic!("Unexpected error {:?}", e),
    }
}

#[test]
fn fail_invalid_amount() {
    let mut deps = mock_deps();
    let amount = 100 * TEST_BASE_DENOM;
    let mint_stage = MintStage {
        price: Some(Uint128::from(amount)),
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    match exec(&mut deps, env, mock_info(TEST_USER_1, &coins(100, "uusd"))) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::InvalidAmount { amount: _ }) => (),
        Err(e) => panic!("Unexpected error {:?}", e),
    }
}

#[test]
fn fail_not_whitelisted() {
    let mut deps = mock_deps();
    let mint_stage = MintStage {
        whitelist_enabled: true,
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    match exec(&mut deps, env, mock_info(TEST_USER_1, &coins(100, "uusd"))) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::NotAllowNonWhitelisted { address: _ }) => (),
        Err(e) => panic!("Unexpected error {:?}", e),
    }
}
