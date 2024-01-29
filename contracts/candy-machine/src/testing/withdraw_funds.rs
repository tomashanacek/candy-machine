use crate::msg::ExecuteMsg;
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{attr, coins};

use crate::contract::execute;
use crate::error::ContractError;
use crate::testing::instantiate;
use crate::testing::{mock_deps, TEST_OWNER, TEST_USER_1};

#[test]
fn success() {
    let mut deps = mock_deps();
    instantiate::default(&mut deps);

    let amount = 1000;

    deps.querier
        .update_balance(MOCK_CONTRACT_ADDR, coins(amount, "uusd"));

    let info = mock_info(TEST_OWNER, &[]);
    let res = execute(
        deps.as_mut(),
        mock_env(),
        info,
        ExecuteMsg::WithdrawFunds {
            recipient: TEST_OWNER.to_string(),
        },
    )
    .unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "withdraw_funds"),
            attr("recipient", TEST_OWNER.to_string()),
        ]
    );
}

#[test]
fn fail_unauthorized() {
    let mut deps = mock_deps();
    instantiate::default(&mut deps);

    let info = mock_info(TEST_USER_1, &[]);
    let res = execute(
        deps.as_mut(),
        mock_env(),
        info,
        ExecuteMsg::WithdrawFunds {
            recipient: TEST_OWNER.to_string(),
        },
    );

    match res.unwrap_err() {
        ContractError::Unauthorized {} => {}
        e => panic!("unexpected error: {:?}", e),
    }
}
