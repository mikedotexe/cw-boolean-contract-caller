mod make_croncat_task;

use crate::errors::ContractError;
use crate::msgs::execute_msg::ExecuteMsg;
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MakeCroncatTask { croncat_manager_address, boolean_address } => make_croncat_task::execute(deps, env, info, croncat_manager_address, boolean_address),
    }
}
