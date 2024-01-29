use crate::msg::MintStage;
use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::{Map, U8Key};

pub const STAGE: Map<U8Key, MintStage> = Map::new("stage");

pub fn store(storage: &mut dyn Storage, stage_id: u8, stage: &MintStage) -> StdResult<()> {
    STAGE.save(storage, U8Key::from(stage_id), &stage)
}

pub fn remove(storage: &mut dyn Storage, stage_id: u8) {
    STAGE.remove(storage, U8Key::from(stage_id))
}

pub fn load(storage: &dyn Storage, stage_id: u8) -> Option<MintStage> {
    STAGE.may_load(storage, U8Key::from(stage_id)).ok()?
}
