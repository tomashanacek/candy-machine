use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::OwnedDeps;

use crate::testing::mock_querier::{mock_dependencies, CustomMockWasmQuerier};

mod candy_machine;
mod configure;
mod configure_mint_stage;
mod instantiate;
mod mint_collectible;
mod mock_querier;
mod withdraw_funds;

const TEST_OWNER: &str = "wasm1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v";
const TEST_USER_1: &str = "terra1e8ryd9ezefuucd4mje33zdms9m2s90m57878v9";
const TEST_NFT_ADDRESS: &str = "wasm1e8ryd9ezefuucd4mje33zdms9m2s90m57878v1";
const TEST_MINTER: &str = "wasm1e8ryd9ezefuucd4mje33zdms9m2s90m57878v2";
const TEST_NFT_NAME: &str = "Test";
const TEST_NFT_SYMBOL: &str = "TEST";
const TEST_NFT_DESCRIPTION: &str = "TEST NFT";
const TEST_NFT_IMAGE: &str = "ipfs://QmerYDeXcVhwPsrYc6pGJmzt1RZUgrfsCdfz9DUrkyDRed";
const TEST_STAGE_ID: u8 = 1;
const TEST_BASE_DENOM: u128 = 1_000_000u128;
const TEST_PUBLIC_KEY: &str = "A/+Q17DEAXBGd5DGQwnqQAKIwkBRPZuy2Qi+J/oDpxxI";
const TEST_SIGNATURE: &str =
    "gwTKfDMZ1DVDz0CLox1vbSg3E1vuM9uNiALEdc4yw5wEIPXVQJfzR9WhvNC84X0U3xMZx5YIwjH5tzEHe5UDYA==";

type MockDeps = OwnedDeps<MockStorage, MockApi, CustomMockWasmQuerier>;

fn mock_deps() -> MockDeps {
    mock_dependencies(&[])
}
