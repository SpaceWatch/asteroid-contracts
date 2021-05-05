use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Order, StdResult, Storage};
use cosmwasm_storage::{
    singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton,
};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: CanonicalAddr,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Alert {
    pub key: String,
    pub blockchain: String,
    pub protocol: String,
    pub method: String,
    pub name: String,
    pub description: String,
    pub fields: Vec<AlertField>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AlertField {
    pub key: String,
    pub name: String,
    pub description: String,
    pub validation_regex: String,
}

static PREFIX_ALERT: &[u8] = b"alert";

pub fn store_alert<S: Storage>(storage: &mut S, alert: &Alert) -> StdResult<()> {
    Bucket::new(PREFIX_ALERT, storage).save(alert.key.as_bytes(), alert)
}

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;
pub fn read_alerts<S: Storage>(
    storage: &S,
    limit: Option<u32>,
    start_after: Option<CanonicalAddr>, // Kinda like a cursor for pagination
    order: Option<Order>,
) -> StdResult<Vec<Alert>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let (start, end, order) = match order {
        Some(Order::Ascending) => (calc_range_start(start_after), None, Order::Ascending),
        _ => (None, calc_range_end(start_after), Order::Descending),
    };

    let alerts_bucket: ReadonlyBucket<S, Alert> = ReadonlyBucket::new(PREFIX_ALERT, storage);
    alerts_bucket
        .range(start.as_deref(), end.as_deref(), order.into())
        .take(limit)
        .map(|item| {
            let (_k, v) = item?;
            Ok(Alert {
                key: v.key,
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
