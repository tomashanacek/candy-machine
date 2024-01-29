use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use candy_machine::msg::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, IsWhitelistedResponse, MintStage,
    MintStagesResponse, QueryMsg, StateResponse,
};
use candy_machine::state::config::Config;
use candy_machine::state::state::State;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(MintStage), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(StateResponse), &out_dir);
    export_schema(&schema_for!(IsWhitelistedResponse), &out_dir);
    export_schema(&schema_for!(MintStagesResponse), &out_dir);
}
