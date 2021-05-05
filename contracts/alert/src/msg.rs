use crate::models::{Alert, AlertField, OrderBy};
use cosmwasm_std::{CanonicalAddr, HumanAddr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateAlert {
        blockchain: String,
        protocol: String,
        method: String,
        name: String,
        description: String,
        fields: Vec<AlertField>,
    },
    SubscribeAlert {
        alert_key: String,
    },
    UnsubscribeAlert {
        alert_key: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAlerts {
        start_after: Option<HumanAddr>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAlertsResponse {
    pub alerts: Vec<Alert>,
}
