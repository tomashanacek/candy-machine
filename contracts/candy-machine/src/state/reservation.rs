use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Map, U32Key};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Reservation {
    pub token_id: u32,
    pub user_address: Addr,
}

pub const RESERVATION: Map<U32Key, Reservation> = Map::new("reservation");
pub const UNPROCESSED: Map<U32Key, Addr> = Map::new("unprocessed");

pub fn store(storage: &mut dyn Storage, token_id: u32, reservation: &Reservation) -> StdResult<()> {
    RESERVATION.save(storage, U32Key::from(token_id), &reservation)
}

pub fn remove(storage: &mut dyn Storage, token_id: u32) {
    RESERVATION.remove(storage, U32Key::from(token_id))
}

pub fn load(storage: &dyn Storage, token_id: u32) -> Option<Reservation> {
    RESERVATION.may_load(storage, U32Key::from(token_id)).ok()?
}

pub fn store_unprocessed(
    storage: &mut dyn Storage,
    token_id: u32,
    user_address: &Addr,
) -> StdResult<()> {
    UNPROCESSED.save(storage, U32Key::from(token_id), &user_address)
}

pub fn remove_unprocessed(storage: &mut dyn Storage, token_id: u32) {
    UNPROCESSED.remove(storage, U32Key::from(token_id))
}
