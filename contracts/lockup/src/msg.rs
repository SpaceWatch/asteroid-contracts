use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ CanonicalAddr };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub owner_addr: CanonicalAddr,
    pub anchor_contract_addr: CanonicalAddr, // Which contract specifically?
    pub lockup_token: CanonicalAddr, // The token to lockup
    pub lockup_amount: i32, // The token to lockup
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Increment {},
    Reset { count: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
