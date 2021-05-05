use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str;

use crate::models::OrderBy;
use cosmwasm_std::{Api, CanonicalAddr, Decimal, Extern, Querier, StdResult, Storage};
use cosmwasm_storage::{
    singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton,
};

use crate::models::Alert;

pub static PREFIX_CONFIG: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
}

pub fn store_config<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    singleton(storage, PREFIX_CONFIG).save(config)
}

pub fn read_config<S: Storage>(storage: &S) -> StdResult<Config> {
    singleton_read(storage, PREFIX_CONFIG).load()
}

static PREFIX_ALERT: &[u8] = b"alert";

pub fn store_alert<S: Storage>(storage: &mut S, alert: &Alert) -> StdResult<()> {
    Bucket::new(PREFIX_ALERT, storage).save(alert.key.as_bytes(), alert)
}

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;
pub fn read_alerts<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,

    start_after: Option<CanonicalAddr>, // Kinda like a cursor for pagination
    limit: Option<u32>,
    order: Option<OrderBy>,
) -> StdResult<Vec<Alert>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let (start, end, order) = match order {
        Some(OrderBy::Asc) => (calc_range_start(start_after), None, OrderBy::Asc),
        _ => (None, calc_range_end(start_after), OrderBy::Desc),
    };

    let alerts_bucket: ReadonlyBucket<S, Alert> = ReadonlyBucket::new(PREFIX_ALERT, &deps.storage);
    alerts_bucket
        .range(start.as_deref(), end.as_deref(), order.into())
        .take(limit)
        .map(|item| {
            let (k, v) = item?;
            Ok(Alert {
                key: String::from_utf8(k).unwrap(),
                blockchain: v.blockchain,
                protocol: v.protocol,
                method: v.method,
                name: v.name,
                description: v.description,
                fields: v.fields,
            })
        })
        .collect()
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_start(start_after: Option<CanonicalAddr>) -> Option<Vec<u8>> {
    start_after.map(|idx| {
        let mut v = idx.as_slice().to_vec();
        v.push(1);
        v
    })
}

// this will set the first key after the provided key in Desc
fn calc_range_end(start_after: Option<CanonicalAddr>) -> Option<Vec<u8>> {
    start_after.map(|idx| idx.as_slice().to_vec())
}
