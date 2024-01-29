use crate::msg::{CollectionKind, ConfigResponse, ExecuteMsg, QueryMsg};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{attr, from_binary, Env, Response};

use crate::contract::{execute, query};
use crate::error::ContractError;
use crate::testing::instantiate;
use crate::testing::{
    mock_deps, MockDeps, TEST_NFT_ADDRESS, TEST_NFT_DESCRIPTION, TEST_NFT_IMAGE, TEST_OWNER,
    TEST_USER_1,
};

pub fn set_nft_address_msg() -> ExecuteMsg {
    ExecuteMsg::Configure {
        description: None,
        name: None,
        nft_address: Some(TEST_NFT_ADDRESS.to_string()),
    }
}

pub fn set_nft_address(deps: &mut MockDeps, env: Env) -> Result<Response, ContractError> {
    let info = mock_info(TEST_OWNER, &[]);
    execute(deps.as_mut(), env, info, set_nft_address_msg())
}

#[test]
fn fail_configure_unauthorized() {
    let mut deps = mock_deps();
    instantiate::default(&mut deps);

    let new_nft_name = "test".to_string();

    let info = mock_info(TEST_USER_1, &[]);
    let res = execute(
        deps.as_mut(),
        mock_env(),
        info,
        ExecuteMsg::Configure {
            description: None,
            name: Some(new_nft_name),
            nft_address: None,
        },
    );

    match res.unwrap_err() {
        ContractError::Unauthorized {} => {}
        e => panic!("unexpected error: {:?}", e),
    }
}

#[test]
fn success_configure() {
    // instantiate
    let mut deps = mock_deps();
    instantiate::default(&mut deps);

    let new_nft_name = "test".to_string();

    let info = mock_info(TEST_OWNER, &[]);
    let res = execute(
        deps.as_mut(),
        mock_env(),
        info,
        ExecuteMsg::Configure {
            description: None,
            nft_address: Some(new_nft_name.clone()),
            name: Some(new_nft_name.clone()),
        },
    )
    .unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "configure"),
            attr("sender", TEST_OWNER.to_string()),
        ]
    );

    // check config
    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let value: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(
        value,
        ConfigResponse {
            name: new_nft_name.clone(),
            description: TEST_NFT_DESCRIPTION.to_string(),
            collection_kind: CollectionKind::Single {
                image: TEST_NFT_IMAGE.to_string()
            },
            max_token_count: 5,
            nft_address: Some(new_nft_name)
        }
    )
}
