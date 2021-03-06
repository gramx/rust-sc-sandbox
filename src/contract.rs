use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::TOTAL,
    state::TRANSACTIONS,
};

#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
    Uint128,
};

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
        //TODO: Add min-amount to prevent spam
        ExecuteMsg::Add { amount } => add(_deps, _info, amount),
    }
}

fn add(_deps: DepsMut, _info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    let sender = _info.sender.to_string();

    ////TODO: Old method, could be replaced
    let running_total = TOTAL.load(_deps.storage)? + amount;
    let _total = TOTAL.save(_deps.storage, &running_total);

    // update function for new or existing keys
    let update_tnx = |d: Option<Uint128>| -> StdResult<Uint128> {
        match d {
            Some(one) => Ok(one + amount),
            None => Ok(amount),
        }
    };
    TRANSACTIONS.update(_deps.storage, &sender, &update_tnx)?;

    Ok(Response::new()
        .add_attribute("action", "add")
        .add_attribute("total", running_total.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        //QueryMsg::Total {} => to_binary(&running_total(_deps)?),
        QueryMsg::Total {} => to_binary(&total_pool(_deps)?),
    }
}

/*
// Using a running total might use less gas? Not sure, needs testing.
fn running_total(_deps: Deps) -> StdResult<Uint128> {
    TOTAL.load(_deps.storage)
}
*/

// Gets a total by adding up everything. The running total method might be better for gas?
fn total_pool(_deps: Deps) -> StdResult<Uint128> {
    let ledger: StdResult<Vec<_>> = TRANSACTIONS
        .range(_deps.storage, None, None, Order::Ascending)
        .collect();
    let mut total = Uint128::from(0u128);
    for tnx in ledger.unwrap() {
        total += tnx.1;
    }
    Ok(total)
}

//TODO:
//  ENVIRO
//  - Fix git push not working in IDE
//  - Fix Code . and other things not working
//  X Downgrade node to fix SSL error (done, did not fix)
//
//  EASY
//  - Fing out how to get unit tests that call functions directly
//  - Find out what totalling method is cheaper, then clean code
//  - Figure out how to get random (lazy mode blockHeight.Hash or something)
//  - Add minimum buy-in to prevent spamming
//
//  MED
//  - Figure out how to get a Rust linter installed and on the pipeline
//  - Build winning logic (split tickets between clients by their buy-in ammount)
//  - Payout to winner 50%
//  - Make in-term payout for charity... maye MVP the owner can specify an address?
//
//  HARD
//  - Use real currency for "amount"
//  - Link into Angle Protocol for donations
//  - Figure out a true random
//  - Add logic for contract version migration (upgrades)

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn test_add_amount_first_person() {
        //Mock Data Setup
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("sender", &[]);
        let msg = InstantiateMsg {};

        //Start the contract
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        //Setup and add the amount
        let msg = ExecuteMsg::Add {
            amount: Uint128::new(12),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Value returned is as expected
        assert_eq!(
            Response::new()
                .add_attribute("action", "add")
                .add_attribute("total", "12"),
            res
        );

        //Value stored is as expected
        assert_eq!(Uint128::new(12), TOTAL.load(deps.as_ref().storage).unwrap());
    }
    #[test]
    fn test_add_amount_second_person() {
        //Mock Data Setup
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("sender", &[]);
        let msg = InstantiateMsg {};
        //Start the contract
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        //Setup and add the amount
        let msg = ExecuteMsg::Add {
            amount: Uint128::new(12),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Value returned is as expected
        assert_eq!(
            Response::new()
                .add_attribute("action", "add")
                .add_attribute("total", "12"),
            res
        );

        //Value stored is as expected
        assert_eq!(Uint128::new(12), TOTAL.load(deps.as_ref().storage).unwrap());

        //Setup and add second amount
        let msg = ExecuteMsg::Add {
            amount: Uint128::new(32),
        };
        let info = mock_info("sender", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        //Total value returned is as expected
        assert_eq!(
            Response::new()
                .add_attribute("action", "add")
                .add_attribute("total", "44"),
            res
        );
        //Total value stored is as expected
        assert_eq!(Uint128::new(44), TOTAL.load(deps.as_ref().storage).unwrap());
    }

    #[test]
    fn test_transactions_only_one() {
        //Mock Data Setup
        let addr1 = String::from("addr0001");
        let info = mock_info(addr1.as_ref(), &[]);
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};

        //Start the contract
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        //Setup and add the amount
        let msg = ExecuteMsg::Add {
            amount: Uint128::new(99),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Total is working correctly
        assert_eq!(
            Response::new()
                .add_attribute("action", "add")
                .add_attribute("total", "99"),
            _res
        );
        //Stored value lookup by address is working correctly
        assert_eq!(
            Uint128::new(99),
            TRANSACTIONS.load(deps.as_ref().storage, &addr1).unwrap()
        );
    }

    #[test]
    fn test_transactions_more_than_one() {
        //Mock Data Setup
        let addr1 = String::from("addr0001");
        let info = mock_info(addr1.as_ref(), &[]);
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};

        //Start the contract
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        //Setup and add the amount
        let msg = ExecuteMsg::Add {
            amount: Uint128::new(10),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Setup and add the second amount
        let msg = ExecuteMsg::Add {
            amount: Uint128::new(5),
        };
        let addr2 = String::from("addr0002");
        let info = mock_info(addr2.as_ref(), &[]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Total value returned is as expected (this should be in its own test, redundant here)
        assert_eq!(
            Response::new()
                .add_attribute("action", "add")
                .add_attribute("total", "15"),
            _res
        );
        //First value stored is as expected
        assert_eq!(
            Uint128::new(10),
            TRANSACTIONS.load(deps.as_ref().storage, &addr1).unwrap()
        );
        //Second value stored is as expected
        assert_eq!(
            Uint128::new(5),
            TRANSACTIONS.load(deps.as_ref().storage, &addr2).unwrap()
        );
    }

    #[test]
    fn test_transactions_duplicate_senders() {
        //Mock Data Setup
        let addr1 = String::from("addr0001");
        let info = mock_info(addr1.as_ref(), &[]);
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};

        //Start the contract
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        //Setup and add the amount
        let msg = ExecuteMsg::Add {
            amount: Uint128::new(10),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Setup and add the second amount
        let msg = ExecuteMsg::Add {
            amount: Uint128::new(5),
        };
        let addr2 = String::from("addr0002");
        let info = mock_info(addr2.as_ref(), &[]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Setup and add the first sender (a second time)
        let msg = ExecuteMsg::Add {
            amount: Uint128::new(100),
        };
        let info = mock_info(addr1.as_ref(), &[]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        //Total value returned is as expected (this should be in its own test, redundant here)
        assert_eq!(
            Response::new()
                .add_attribute("action", "add")
                .add_attribute("total", "115"),
            _res
        );
        //First value stored is as expected
        assert_eq!(
            Uint128::new(110),
            TRANSACTIONS.load(deps.as_ref().storage, &addr1).unwrap()
        );
        //Second value stored is as expected
        assert_eq!(
            Uint128::new(5),
            TRANSACTIONS.load(deps.as_ref().storage, &addr2).unwrap()
        );
    }

    /*
    #[test]
    fn test_new_total_logic_temp() {
        //Mock Data Setup
        let addr1 = String::from("addr0001");
        let info = mock_info(addr1.as_ref(), &[]);
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};

        //Start the contract
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        //Setup and add the amount
        let msg = ExecuteMsg::Add {amount: Uint128::new(10)};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    }*/
}
