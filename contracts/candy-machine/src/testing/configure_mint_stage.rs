use crate::msg::{ConfigureMintStageMsg, ExecuteMsg, IsWhitelistedResponse, MintStage, QueryMsg};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{attr, from_binary, Env, Response};

use crate::contract::{execute, query};
use crate::error::ContractError;
use crate::testing::instantiate;
use crate::testing::{mock_deps, MockDeps, TEST_OWNER, TEST_STAGE_ID, TEST_USER_1};

pub fn update_whitelist_msg() -> ExecuteMsg {
    ExecuteMsg::ConfigureMintStage {
        id: TEST_STAGE_ID,
        config: ConfigureMintStageMsg::Whitelist {
            whitelist: true,
            candidates: vec![TEST_USER_1.to_string()],
        },
    }
}

pub fn update_whitelist(deps: &mut MockDeps, env: Env) -> Result<Response, ContractError> {
    let info = mock_info(TEST_OWNER, &[]);
    execute(deps.as_mut(), env, info, update_whitelist_msg())
}

#[test]
fn success_update_whitelist() {
    // instantiate
    let mut deps = mock_deps();
    instantiate::default(&mut deps);

    let res = update_whitelist(&mut deps, mock_env()).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "whitelist_user"),
            attr("sender", TEST_OWNER.to_string()),
        ]
    );

    // check config
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::IsWhitelisted {
            stage_id: TEST_STAGE_ID,
            address: TEST_USER_1.to_string(),
        },
    )
    .unwrap();
    let value: IsWhitelistedResponse = from_binary(&res).unwrap();
    assert_eq!(value, IsWhitelistedResponse { whitelisted: true })
}

#[test]
fn success_update_mint_stage() {
    // instantiate
    let mut deps = mock_deps();
    instantiate::default(&mut deps);

    let mint_stage = instantiate::default_mint_stage();

    let info = mock_info(TEST_OWNER, &[]);
    let res = execute(
        deps.as_mut(),
        mock_env(),
        info,
        ExecuteMsg::ConfigureMintStage {
            id: TEST_STAGE_ID,
            config: ConfigureMintStageMsg::Config {
                name: None,
                start: None,
                finish: None,
                max_per_user: Some(10),
                price: None,
                whitelist_enabled: None,
            },
        },
    )
    .unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "configure_mint_stage"),
            attr("sender", TEST_OWNER.to_string()),
        ]
    );

    // check config
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::MintStage {
            stage_id: TEST_STAGE_ID,
        },
    )
    .unwrap();
    let value: MintStage = from_binary(&res).unwrap();
    assert_eq!(
        value,
        MintStage {
            id: mint_stage.id,
            name: mint_stage.name,
            start: mint_stage.start,
            finish: mint_stage.finish,
            max_per_user: Some(10),
            price: mint_stage.price,
            whitelist_enabled: mint_stage.whitelist_enabled
        }
    )
}

#[test]
fn fail_configure_unauthorized() {
    let mut deps = mock_deps();
    instantiate::default(&mut deps);

    let info = mock_info(TEST_USER_1, &[]);
    let res = execute(deps.as_mut(), mock_env(), info, update_whitelist_msg());

    match res.unwrap_err() {
        ContractError::Unauthorized {} => {}
        e => panic!("unexpected error: {:?}", e),
    }
}

#[test]
fn fail_unknown_mint_stage() {
    let mut deps = mock_deps();
    instantiate::default(&mut deps);

    let info = mock_info(TEST_OWNER, &[]);
    let res = execute(
        deps.as_mut(),
        mock_env(),
        info,
        ExecuteMsg::ConfigureMintStage {
            id: 2,
            config: ConfigureMintStageMsg::Whitelist {
                whitelist: true,
                candidates: vec![TEST_USER_1.to_string()],
            },
        },
    );

    match res.unwrap_err() {
        ContractError::UnknownMintStage {} => {}
        e => panic!("unexpected error: {:?}", e),
    }
}
