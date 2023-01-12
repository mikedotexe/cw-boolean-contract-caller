use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response, to_binary, WasmMsg, SubMsg};
use cosmwasm_schema::cw_serde;
use crate::errors::ContractError;
use cw_croncat_core::msg::{ExecuteMsg as CroncatExecuteMsg, TaskRequest};
use cw_croncat_core::types::{Action, Interval};
use crate::REPLY_CRONCAT_TASK_CREATION;

// Let's say we don't have a package/crate but know the structure
#[cw_serde]
pub enum BooleanContractExecuteMsg {
    Toggle {},
}

pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, croncat_manager_address: String, boolean_address: String) -> Result<Response, ContractError> {
    // This will validate the addresses and throw an error (see "?") if invalid
    let croncat_manager_address = deps.api.addr_validate(&croncat_manager_address)?;
    let boolean_address = deps.api.addr_validate(&boolean_address)?;

    let croncat_task = TaskRequest {
        interval: Interval::Block(1),
        boundary: None,
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: boolean_address.clone().into_string(),
                msg: to_binary(
                    &BooleanContractExecuteMsg::Toggle {},
                )?,
                funds: vec![],
            }),
            gas_limit: Some(150_000), // fine tune gas here
        }],
        queries: None,
        transforms: None,
        cw20_coins: vec![],
    };

    let create_task_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: croncat_manager_address.clone().into_string(),
        msg: to_binary(
            &CroncatExecuteMsg::CreateTask {
                task: croncat_task,
            },
        )?,
        funds: info.funds,
    });

    let sub_message = SubMsg::reply_on_error(create_task_msg, REPLY_CRONCAT_TASK_CREATION);

    Ok(Response::new()
        .add_attribute("croncat_manager_address", croncat_manager_address)
        .add_attribute("boolean_address", boolean_address)
        .add_submessage(sub_message)
    )
}
