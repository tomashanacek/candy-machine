use crate::error::ContractError;
use crate::state::config::{Config, CONFIG};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn withdraw_funds(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&recipient)?;

    let config: Config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::new()
        .add_attribute("action", "withdraw_funds")
        .add_attribute("recipient", &recipient))
}
