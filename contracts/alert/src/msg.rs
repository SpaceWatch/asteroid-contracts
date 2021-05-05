use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ CanonicalAddr };
use crate::state::Alert;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateAlert {
      alert: Alert
    },
    SubscribeAlert {
      alert_key: String,
    },
    UnsubscribeAlert {
      alert_key: String,
    },
    // Reset { count: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAllAlerts {
      limit: Option<u32>,
      order_by: Option<OrderBy>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAllAlertsResponse {
  alerts: Vec<Alert>,
}




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
  Asc,
  Desc,
}