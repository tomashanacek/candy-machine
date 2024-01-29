use crate::msg::{
    CollectionKind, ExecuteMsg, InstantiateMsg, MintStage, QueryMsg,
    UnprocessedReservationsResponse,
};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{
    attr, coins, from_binary, to_binary, CosmosMsg, Env, MessageInfo, Response, SubMsg, Uint128,
    VerificationError, WasmMsg,
};
use cw721_metadata_onchain::Metadata as Cw721Metadata;
use cw721_metadata_onchain::{ExecuteMsg as Cw721ExecuteMsg, Metadata, MintMsg};

use crate::contract::{execute, query};
use crate::error::ContractError;
use crate::testing::candy_machine;
use crate::testing::configure;
use crate::testing::instantiate;
use crate::testing::{
    mock_deps, MockDeps, TEST_BASE_DENOM, TEST_MINTER, TEST_NFT_ADDRESS, TEST_NFT_DESCRIPTION,
    TEST_NFT_IMAGE, TEST_NFT_NAME, TEST_PUBLIC_KEY, TEST_SIGNATURE, TEST_STAGE_ID, TEST_USER_1,
};

pub fn exec(
    deps: &mut MockDeps,
    env: Env,
    info: MessageInfo,
    token_id: u32,
    metadata: Cw721Metadata,
) -> Result<Response, ContractError> {
    execute(
        deps.as_mut(),
        env,
        info,
        ExecuteMsg::MinterMint { token_id, metadata },
    )
}

#[test]
fn success_mint_collectible() {
    let mut deps = mock_deps();
    let amount = 100 * TEST_BASE_DENOM;
    let mint_stage = MintStage {
        price: Some(Uint128::from(amount)),
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        collection_kind: CollectionKind::Collectible {
            minter: TEST_MINTER.to_string(),
            public_key: None,
            cover: TEST_NFT_IMAGE.to_string(),
        },
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    // reserve
    let info = mock_info(TEST_USER_1, &coins(amount, "uusd"));
    let res = candy_machine::exec(&mut deps, mock_env(), info).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "reserve"),
            attr("owner", TEST_USER_1.to_string()),
        ]
    );

    // check unprocessed reservations
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::UnprocessedReservations {
            start_after: None,
            limit: None,
        },
    )
    .unwrap();
    let res: UnprocessedReservationsResponse = from_binary(&res).unwrap();
    assert_eq!(vec![1], res.reservations);

    // mint
    let info = mock_info(TEST_MINTER, &[]);
    let metadata = Cw721Metadata {
        image: Some(TEST_NFT_IMAGE.to_string()),
        description: Some(TEST_NFT_DESCRIPTION.into()),
        name: Some(format!("{} #{}", TEST_NFT_NAME, 1)),
        attributes: None,
        ..Metadata::default()
    };
    let res = exec(&mut deps, mock_env(), info, 1, metadata.clone()).unwrap();

    let mint_msg = MintMsg {
        token_id: "1".to_string(),
        owner: TEST_USER_1.to_string(),
        token_uri: None,
        extension: Some(metadata),
    };
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: TEST_NFT_ADDRESS.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::Mint(mint_msg)).unwrap(),
            funds: vec![],
        }))]
    );
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "minter_mint"),
            attr("owner", TEST_USER_1.to_string()),
        ]
    );

    // check unprocessed reservations
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::UnprocessedReservations {
            start_after: None,
            limit: None,
        },
    )
    .unwrap();
    let res: UnprocessedReservationsResponse = from_binary(&res).unwrap();
    let expected: Vec<u32> = vec![];
    assert_eq!(expected, res.reservations);
}

#[test]
fn success_reservation_with_signature() {
    let mut deps = mock_deps();
    let mint_stage = MintStage {
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        collection_kind: CollectionKind::Collectible {
            minter: TEST_MINTER.to_string(),
            public_key: Some(TEST_PUBLIC_KEY.to_string()),
            cover: TEST_NFT_IMAGE.to_string(),
        },
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    // mint
    let info = mock_info(TEST_USER_1, &[]);
    let res = execute(
        deps.as_mut(),
        env,
        info,
        ExecuteMsg::Mint {
            stage_id: TEST_STAGE_ID,
            signature: Some(TEST_SIGNATURE.to_string()),
        },
    )
    .unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "reserve"),
            attr("owner", TEST_USER_1.to_string()),
        ]
    );

    // check unprocessed reservations
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::UnprocessedReservations {
            start_after: None,
            limit: None,
        },
    )
    .unwrap();
    let res: UnprocessedReservationsResponse = from_binary(&res).unwrap();
    assert_eq!(vec![1], res.reservations);
}

#[test]
fn fail_unauthorized() {
    let mut deps = mock_deps();
    let amount = 100 * TEST_BASE_DENOM;
    let mint_stage = MintStage {
        price: Some(Uint128::from(amount)),
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        collection_kind: CollectionKind::Collectible {
            minter: TEST_MINTER.to_string(),
            public_key: None,
            cover: TEST_NFT_IMAGE.to_string(),
        },
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    // mint
    let info = mock_info(TEST_USER_1, &[]);
    let metadata = Cw721Metadata {
        image: Some(TEST_NFT_IMAGE.to_string()),
        description: Some(TEST_NFT_DESCRIPTION.into()),
        name: Some(format!("{} #{}", TEST_NFT_NAME, 1)),
        attributes: None,
        ..Metadata::default()
    };
    let res = exec(&mut deps, mock_env(), info, 1, metadata.clone());

    match res.unwrap_err() {
        ContractError::Unauthorized {} => {}
        e => panic!("unexpected error: {:?}", e),
    }
}

#[test]
fn fail_unknown_reservation() {
    let mut deps = mock_deps();
    let amount = 100 * TEST_BASE_DENOM;
    let mint_stage = MintStage {
        price: Some(Uint128::from(amount)),
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        collection_kind: CollectionKind::Collectible {
            minter: TEST_MINTER.to_string(),
            public_key: None,
            cover: TEST_NFT_IMAGE.to_string(),
        },
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    // mint
    let info = mock_info(TEST_MINTER, &[]);
    let metadata = Cw721Metadata {
        image: Some(TEST_NFT_IMAGE.to_string()),
        description: Some(TEST_NFT_DESCRIPTION.into()),
        name: Some(format!("{} #{}", TEST_NFT_NAME, 1)),
        attributes: None,
        ..Metadata::default()
    };
    let res = exec(&mut deps, mock_env(), info, 1, metadata.clone());

    match res.unwrap_err() {
        ContractError::UnknownReservation {} => {}
        e => panic!("unexpected error: {:?}", e),
    }
}

#[test]
fn fail_invalid_collection_kind() {
    let mut deps = mock_deps();
    let env = mock_env();

    instantiate::default(&mut deps);
    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    // mint
    let info = mock_info(TEST_USER_1, &[]);
    let metadata = Cw721Metadata {
        image: Some(TEST_NFT_IMAGE.to_string()),
        description: Some(TEST_NFT_DESCRIPTION.into()),
        name: Some(format!("{} #{}", TEST_NFT_NAME, 1)),
        attributes: None,
        ..Metadata::default()
    };
    let res = exec(&mut deps, mock_env(), info, 1, metadata.clone());

    match res.unwrap_err() {
        ContractError::InvalidCollectionKind {} => {}
        e => panic!("unexpected error: {:?}", e),
    }
}

#[test]
fn fail_signature_not_provided() {
    let mut deps = mock_deps();
    let mint_stage = MintStage {
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        collection_kind: CollectionKind::Collectible {
            minter: TEST_MINTER.to_string(),
            public_key: Some(TEST_PUBLIC_KEY.to_string()),
            cover: TEST_NFT_IMAGE.to_string(),
        },
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    // mint
    let info = mock_info(TEST_USER_1, &[]);
    let res = candy_machine::exec(&mut deps, mock_env(), info);

    match res.unwrap_err() {
        ContractError::InvalidSignature {} => {}
        e => panic!("unexpected error: {:?}", e),
    }
}

#[test]
fn fail_invalid_signature() {
    let mut deps = mock_deps();
    let mint_stage = MintStage {
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        collection_kind: CollectionKind::Collectible {
            minter: TEST_MINTER.to_string(),
            public_key: Some(TEST_PUBLIC_KEY.to_string()),
            cover: TEST_NFT_IMAGE.to_string(),
        },
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    // mint
    let info = mock_info(TEST_USER_1, &[]);
    let res = execute(
        deps.as_mut(),
        env,
        info,
        ExecuteMsg::Mint {
            stage_id: TEST_STAGE_ID,
            signature: Some("z7bviv/gnfbVsg9XPXGXJGMoDyxgJNKmT+q0X0pW6iQykM9vSV2oFScOydG3Wk2aQz+jY8gKpbPpWhvkUjglsg==".to_string()),
        },
    );

    match res.unwrap_err() {
        ContractError::InvalidSignature {} => {}
        e => panic!("unexpected error: {:?}", e),
    }
}

#[test]
fn fail_invalid_signature_format() {
    let mut deps = mock_deps();
    let mint_stage = MintStage {
        ..instantiate::default_mint_stage()
    };

    let init_msg = InstantiateMsg {
        mint_stages: vec![mint_stage],
        collection_kind: CollectionKind::Collectible {
            minter: TEST_MINTER.to_string(),
            public_key: Some(TEST_PUBLIC_KEY.to_string()),
            cover: TEST_NFT_IMAGE.to_string(),
        },
        ..instantiate::default_msg()
    };

    let env = mock_env();

    instantiate::exec(&mut deps, init_msg);

    configure::set_nft_address(&mut deps, env.clone()).unwrap();

    // mint
    let info = mock_info(TEST_USER_1, &[]);
    let res = execute(
        deps.as_mut(),
        env,
        info,
        ExecuteMsg::Mint {
            stage_id: TEST_STAGE_ID,
            signature: Some("".to_string()),
        },
    );

    match res.unwrap_err() {
        ContractError::CryptoVerify(VerificationError::InvalidSignatureFormat {}) => {}
        e => panic!("unexpected error: {:?}", e),
    }
}
