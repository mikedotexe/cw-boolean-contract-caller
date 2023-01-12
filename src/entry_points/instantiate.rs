use crate::msgs::instantiate_msg::InstantiateMsg;
use crate::{CONTRACT_NAME, CONTRACT_VERSION};
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use crate::errors::ContractError;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _first_param: Option<InstantiateMsg>,
) -> Result<Response, ContractError> {
    // Set the contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Return a thumbs up
    Ok(Response::default())
}
