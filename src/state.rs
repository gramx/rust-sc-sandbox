use cw_storage_plus::Map;
use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Transaction {
    pub amount: Uint128,
    pub sender: Addr,
}

pub const TRANSACTIONS: Map<&str, Uint128> = Map::new("transaction");

pub const TOTAL: Item<Uint128> = Item::new("total");