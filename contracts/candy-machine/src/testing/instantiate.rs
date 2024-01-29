use crate::msg::{
    CollectionKind as CollectionKindMsg, InstantiateMsg, MintStage, MintStagesResponse, QueryMsg,
    StateResponse,
};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, to_binary, Api, ContractResult, Env, MessageInfo, Reply, ReplyOn, Response,
    SubMsg, SubMsgExecutionResponse, WasmMsg,
};
use cw2::{get_contract_version, ContractVersion};
use cw721_metadata_onchain::InstantiateMsg as NftInstantiateMsg;

use crate::contract::{instantiate, query, reply, CONTRACT_NAME, CONTRACT_VERSION};
use crate::state::collection_kind::CollectionKind;
use crate::state::config::{Config, CONFIG};
use crate::testing::{
    mock_deps, MockDeps, TEST_NFT_DESCRIPTION, TEST_NFT_IMAGE, TEST_NFT_NAME, TEST_NFT_SYMBOL,
    TEST_OWNER,
};

pub fn exec(deps: &mut MockDeps, msg: InstantiateMsg) -> (Env, MessageInfo, Response) {
    let env = mock_env();
    let info = mock_info(TEST_OWNER, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    (env, info, res)
}

pub fn default(deps: &mut MockDeps) -> (Env, MessageInfo, Response) {
    exec(deps, default_msg())
}

pub fn default_mint_stage() -> MintStage {
    let default_blocktime = mock_env().block.time.seconds();

    MintStage {
        id: 1,
        name: "Public".to_string(),
        start: Some(default_blocktime),
        finish: Some(default_blocktime + 100),
        max_per_user: Some(1),
        whitelist_enabled: false,
        price: None,
    }
}

pub fn default_msg() -> InstantiateMsg {
    InstantiateMsg {
        name: TEST_NFT_NAME.to_string(),
        symbol: TEST_NFT_SYMBOL.to_string(),
        description: TEST_NFT_DESCRIPTION.to_string(),
        collection_kind: CollectionKindMsg::Single {
            image: TEST_NFT_IMAGE.to_string(),
        },
        max_token_count: 5,
        nft_code_id: 10u64,
        mint_stages: vec![default_mint_stage()],
    }
}

#[test]
fn proper_initialization() {
    let mut deps = mock_deps();

    let (_env, _info, res) = default(&mut deps);

    // check contract version
    assert_eq!(
        get_contract_version(deps.as_ref().storage).unwrap(),
        ContractVersion {
            contract: CONTRACT_NAME.to_string(),
            version: CONTRACT_VERSION.to_string()
        }
    );

    // check submessages
    assert_eq!(
        res.messages,
        vec![SubMsg {
            msg: WasmMsg::Instantiate {
                code_id: 10u64,
                msg: to_binary(&NftInstantiateMsg {
                    name: TEST_NFT_NAME.to_string(),
                    symbol: TEST_NFT_SYMBOL.to_string(),
                    minter: MOCK_CONTRACT_ADDR.to_string(),
                })
                .unwrap(),
                funds: vec![],
                label: "".to_string(),
                admin: None,
            }
            .into(),
            gas_limit: None,
            id: 1,
            reply_on: ReplyOn::Success,
        }]
    );

    // store nft token address
    let reply_msg = Reply {
        id: 1,
        result: ContractResult::Ok(SubMsgExecutionResponse {
            events: vec![],
            data: Some(vec![10, 6, 110, 102, 116, 48, 48, 48].into()),
        }),
    };

    let _res = reply(deps.as_mut(), mock_env(), reply_msg).unwrap();

    // check config
    assert_eq!(
        CONFIG.load(deps.as_ref().storage).unwrap(),
        Config {
            owner: deps.api.addr_validate(TEST_OWNER).unwrap(),
            name: TEST_NFT_NAME.to_string(),
            description: TEST_NFT_DESCRIPTION.to_string(),
            collection_kind: CollectionKind::Single {
                image: TEST_NFT_IMAGE.to_string()
            },
            max_token_count: 5,
            nft_address: Some(deps.api.addr_validate("nft000").unwrap()),
        }
    );

    // check state
    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();
    let value: StateResponse = from_binary(&res).unwrap();
    assert_eq!(value, StateResponse { token_count: 0 });

    // check mint stages
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::MintStages { limit: None },
    )
    .unwrap();
    let value: MintStagesResponse = from_binary(&res).unwrap();
    assert_eq!(
        value,
        MintStagesResponse {
            mint_stages: vec![default_mint_stage()]
        }
    );
}
