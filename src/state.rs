use cw_storage_plus::Map;
use cosmwasm_std::Uint128;
use cw_storage_plus::Item;

pub const ADDRESSES: Map<&str, Uint128> = Map::new("addresses");

pub const TOTAL: Item<Uint128> = Item::new("total");