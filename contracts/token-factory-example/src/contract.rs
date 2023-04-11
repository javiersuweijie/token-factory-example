#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, CosmosMsg, Uint128, Reply, StdError};
use cw0::parse_instantiate_response_data;
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ CountResponse, CustomExecuteMsg, DenomUnit, ExecuteMsg, InstantiateMsg, Metadata, QueryMsg, TokenExecuteMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:token-factory-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CREATE_REPLY_ID:u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    return Ok(Response::default());
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<CustomExecuteMsg>, ContractError> {
    match msg {
        ExecuteMsg::CreateToken { denom, symbol, name} => create_token(deps, env, info,denom, name,symbol),
        ExecuteMsg::MintToken { denom, amount, recipient } => mint_token(deps, info, denom, amount, recipient),
        ExecuteMsg::BurnToken { .. } => burn_token(deps, info),
    }
}

fn create_token(deps: DepsMut, env: Env, info: MessageInfo, denom: String, name: String, symbol: String) -> Result<Response<CustomExecuteMsg>, ContractError> {
    let tf_denom = format!("factory/{}/{}", env.contract.address, denom);
    let create_token_msg = TokenExecuteMsg::CreateDenom {
        subdenom: denom.clone(),
        metadata: Metadata {
            description: name.clone(),
            denom_units: vec![DenomUnit {
                denom: tf_denom.to_string(),
                exponent: 0,
                aliases: vec![],
            }],
            base: tf_denom.to_string(),
            display: tf_denom.to_string(),
            name,
            symbol,
        },
    };
    let submsg = SubMsg::reply_on_success(CosmosMsg::Custom(CustomExecuteMsg::Token(create_token_msg)), CREATE_REPLY_ID);
   Ok(Response::default().add_submessage(submsg))
}

fn mint_token(dep: DepsMut, info: MessageInfo, denom: String, amount: Uint128, recipient: String) -> Result<Response<CustomExecuteMsg>, ContractError> {
    let mint_msg = TokenExecuteMsg::MintTokens {
        denom,
        amount,
        mint_to_address: recipient
    };
    let submsg = SubMsg::new(CosmosMsg::Custom(CustomExecuteMsg::Token(mint_msg)));
    Ok(Response::default().add_submessage(submsg))
}

fn burn_token(dep: DepsMut, info: MessageInfo) -> Result<Response<CustomExecuteMsg>, ContractError> {
    let coins_to_burn = info.funds;
    let mut burn_msgs = vec![];
    for coin in coins_to_burn {
        let denom = coin.denom;
        let amount = coin.amount;
        let burn_msg = TokenExecuteMsg::BurnTokens {
            denom,
            amount,
            burn_from_address: "".to_string()
        };
        burn_msgs.push(SubMsg::new(CosmosMsg::Custom(CustomExecuteMsg::Token(burn_msg))));
    }
    Ok(Response::default().add_submessages(burn_msgs))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        CREATE_REPLY_ID => {
            let response= reply.result.unwrap();
            // This is a hack to get the denom from the response data. This is not a good way to do this.
            // It works because the response data is a protobuf encoded string that contains the denom in the first slot (similar to the contract instantiation response)
            let denom =  parse_instantiate_response_data(response.data.unwrap().as_slice()).map_err(|_| ContractError::Std(StdError::generic_err("parse error".to_string())))?;
            Ok(Response::new().add_attribute("denom", denom.contract_address))
        }
        _ => Err(ContractError::InvalidReplyId(reply.id))
    }
}
