use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};

pub static PREFIX_USER: &[u8] = b"user";
pub static PREFIX_WHITELIST: &[u8] = b"whitelist";
pub static PREFIX_USER_INDEX: &[u8] = b"index-user";

pub fn store(storage: &mut dyn Storage, owner: &CanonicalAddr, user: u16) -> StdResult<()> {
    let mut user_bucket: Bucket<u16> = bucket(storage, PREFIX_USER);
    user_bucket.save(owner.as_slice(), &user)
}

pub fn remove(storage: &mut dyn Storage, owner: &CanonicalAddr) {
    let mut user_bucket: Bucket<u16> = bucket(storage, PREFIX_USER);
    user_bucket.remove(owner.as_slice())
}

pub fn load(storage: &dyn Storage, owner: &CanonicalAddr) -> u16 {
    let user_bucket: ReadonlyBucket<u16> = bucket_read(storage, PREFIX_USER);
    user_bucket.load(owner.as_slice()).unwrap_or_default()
}

pub fn register_whitelist(
    storage: &mut dyn Storage,
    stage_id: u8,
    owner: &CanonicalAddr,
) -> StdResult<()> {
    save_whitelist(storage, stage_id, owner, true)
}

pub fn unregister_whitelist(
    storage: &mut dyn Storage,
    stage_id: u8,
    owner: &CanonicalAddr,
) -> StdResult<()> {
    save_whitelist(storage, stage_id, owner, false)
}

fn save_whitelist(
    storage: &mut dyn Storage,
    stage_id: u8,
    owner: &CanonicalAddr,
    whitelisted: bool,
) -> StdResult<()> {
    Bucket::<bool>::multilevel(storage, &[PREFIX_USER_INDEX, PREFIX_WHITELIST, &[stage_id]])
        .save(owner.as_slice(), &whitelisted)
}

pub fn is_whitelisted(storage: &dyn Storage, stage_id: u8, owner: &CanonicalAddr) -> bool {
    ReadonlyBucket::<bool>::multilevel(storage, &[PREFIX_USER_INDEX, PREFIX_WHITELIST, &[stage_id]])
        .load(owner.as_slice())
        .unwrap_or_default()
}
