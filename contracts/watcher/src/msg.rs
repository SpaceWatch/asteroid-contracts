use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ CanonicalAddr };
use crate::state::Watcher;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateWatcher {
      watcher: Watcher
    },
    SubscribeWatcher {
      watcher_key: String,
    },
    UnsubscribeWatcher {
      watcher_key: String,
    },
    // Reset { count: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAllWatchers {
      limit: Option<u32>,
      order_by: Option<OrderBy>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAllWatchersResponse {
  watchers: Vec<Watcher>,
}




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
  Asc,
  Desc,
}