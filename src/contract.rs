use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::ADDRESSES,
    state::TOTAL,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Uint128;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let total_starter = Uint128::from(0u128);
    TOTAL.save(_deps.storage, &total_starter)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::Add { amount } => add(_deps, _info, amount)
    }
}

fn add(_deps: DepsMut, _info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    let sender = _info.sender.to_string();
    ADDRESSES.save(_deps.storage, &sender, &amount)?;
    let running_total = TOTAL.load(_deps.storage)? + &amount;
    let _total = TOTAL.save(_deps.storage, &running_total);
    Ok(Response::new()
        .add_attribute("action", "add")
        .add_attribute("total", running_total.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::Total {} => to_binary(&total(_deps)?),
    }
}

fn total(_deps: Deps) -> StdResult<Uint128> {
    TOTAL.load(_deps.storage)
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn test_add_amount_once() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("sender", &[]);

        let msg = InstantiateMsg {};

        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Add {
            amount: Uint128::new(12),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(
            Response::new()
                .add_attribute("action","add")
                .add_attribute("total", "12"),
            res
        );

        assert_eq!(
            Uint128::new(12), 
            ADDRESSES.load(
                deps.as_ref()
                .storage,
                "sender"
            ).unwrap()
        );
    }

    
    #[test]
    fn test_add_amount_twice() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("sender", &[]);

        let msg = InstantiateMsg {};

        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Add {
            amount: Uint128::new(12),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        assert_eq!(
            Response::new()
                .add_attribute("action","add")
                .add_attribute("total", "12"),
            res
        );

        assert_eq!(
            Uint128::new(12),
            ADDRESSES.load(
                deps.as_ref()
                .storage,
                "sender"
            ).unwrap()
        );

        /*
        assert_eq!(
            Response::new()
                .add_attribute("action","add")
                .add_attribute("total", "32"),
            res
        );

        assert_eq!(
            //Uint128::new(54),
            Uint128::new(32),
            ADDRESSES.load(
                deps.as_ref()
                .storage,
                "sender"
            ).unwrap()
        );
        */
    }

}
