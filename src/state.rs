use cw_storage_plus::{Map, Item};
use cosmwasm_std::Uint128;


pub const TRANSACTIONS: Map<&str, Uint128> = Map::new("transaction");

pub const TOTAL: Item<Uint128> = Item::new("total");