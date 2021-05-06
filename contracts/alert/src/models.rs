use cosmwasm_std::Order;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Alert {
    pub alert_key: String,
    pub blockchain: String,
    pub protocol: String,
    pub method: String,
    pub name: String,
    pub description: String,
    pub fields: Vec<AlertField>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AlertField {
    pub field_key: String,
    pub name: String,
    pub description: String,
    pub validation_regex: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Subscription {
    pub alert_key: String,
    pub field_values: Vec<SubscriptionFieldValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SubscriptionFieldValue {
    pub field_key: String,
    pub value: String,
}

/**
 * OrderBy implementation copied from https://github.com/Mirror-Protocol/mirror-contracts/blob/0211924a22296423e0fbc2762b69123f686a231f/packages/mirror_protocol/src/common.rs
 */
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    Asc,
    Desc,
}

// Converts our custom OrderBy to cosmwasm_std::Order;
impl Into<Order> for OrderBy {
    fn into(self) -> Order {
        if self == OrderBy::Asc {
            Order::Ascending
        } else {
            Order::Descending
        }
    }
}
