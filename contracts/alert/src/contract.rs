use crate::models::{Alert, AlertField, OrderBy, Subscription, SubscriptionFieldValue};
use crate::msg::{
    GetAlertsResponse, GetSubscriptionsForAddressResponse, HandleMsg, InitMsg, QueryMsg,
};
use crate::state::{
    read_alert, read_alerts, read_config, read_subscription_for_address,
    read_subscriptions_for_address, remove_subscription_for_address, store_alert, store_config,
    store_subscription_for_address, Config,
};
use cosmwasm_std::{
    to_binary, Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, StdError, StdResult, Storage,
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let config = Config {
        owner: deps.api.canonical_address(&env.message.sender)?,
    };

    store_config(&mut deps.storage, &config)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreateAlert {
            blockchain,
            protocol,
            method,
            name,
            description,
            fields,
        } => try_create_alert(
            deps,
            env,
            blockchain,
            protocol,
            method,
            name,
            description,
            fields,
        ),
        HandleMsg::SubscribeAlert {
            alert_key,
            field_values,
        } => try_subscribe_alert(deps, env, alert_key, field_values),
        HandleMsg::UnsubscribeAlert { alert_key } => try_unsubscribe_alert(deps, env, alert_key),
    }
}

pub fn try_create_alert<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    blockchain: String,
    protocol: String,
    method: String,
    name: String,
    description: String,
    fields: Vec<AlertField>,
) -> StdResult<HandleResponse> {
    let config: Config = read_config(&deps.storage)?;

    // Only owner can create alert for now
    // TODO: Have some form of whitelisted addresses
    if deps.api.canonical_address(&env.message.sender)? != config.owner {
        return Err(StdError::unauthorized());
    }

    // Concatenate `Blockchain.Protocol.method` to use as alert_key
    let alert_key: String = format!("{}.{}.{}", blockchain, protocol, method);
    let alert: Alert = Alert {
        alert_key,
        blockchain,
        protocol,
        method,
        name,
        description,
        fields,
    };
    store_alert(&mut deps.storage, &alert)?;

    Ok(HandleResponse::default())
}

pub fn try_subscribe_alert<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    alert_key: String,
    field_values: Vec<SubscriptionFieldValue>,
) -> StdResult<HandleResponse> {
    // This throws an error if the alert doesen't exist
    // TODO: Better error message
    let alert: Alert = read_alert(&deps.storage, &alert_key)?;
    let alert_fields: Vec<AlertField> = alert.fields;
    for alert_field in alert_fields {
        let valid_field_value: Option<&SubscriptionFieldValue> = field_values
            .iter()
            .find(|field_value| field_value.field_key == alert_field.field_key);
        // Throw error if a required value for a field is missing
        if !valid_field_value.is_some() {
            return Err(StdError::generic_err(format!(
                "Missing field {}",
                alert_field.field_key
            )));
        }
        // Throw error if a field value does not satisfy regex expression
        // if !Regex::new(&alert_field.validation_regex)
        //     .unwrap()
        //     .is_match(&valid_field_value.unwrap().value)
        // {
        //     return Err(StdError::generic_err(format!(
        //         "Invalid field {}",
        //         alert_field.field_key
        //     )));
        // }
    }

    let canonical_subscriber_addr: CanonicalAddr =
        deps.api.canonical_address(&env.message.sender)?;
    let subscription: Subscription = Subscription {
        alert_key,
        field_values,
    };
    store_subscription_for_address(&mut deps.storage, canonical_subscriber_addr, subscription)?;

    Ok(HandleResponse::default())
}

pub fn try_unsubscribe_alert<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    alert_key: String,
) -> StdResult<HandleResponse> {
    let canonical_subscriber_addr: CanonicalAddr =
        deps.api.canonical_address(&env.message.sender)?;

    // This throws an alert if an alert with this key doesn't exist
    // TODO: Better error message
    read_alert(&deps.storage, &alert_key)?;
    // This throws an error if user is not subscribed for this alert
    // TODO: Better error message
    read_subscription_for_address(&deps.storage, &canonical_subscriber_addr, &alert_key)?;

    remove_subscription_for_address(&mut deps.storage, canonical_subscriber_addr, alert_key);

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAlerts {
            start_after,
            limit,
            order_by,
        } => to_binary(&handle_query_alerts(deps, start_after, limit, order_by)),
        QueryMsg::GetSubscriptionsForAddress {
            subscriber_addr,
            start_after,
            limit,
            order_by,
        } => to_binary(&handle_query_subscriptions_for_address(
            deps,
            subscriber_addr,
            start_after,
            limit,
            order_by,
        )),
    }
}

fn handle_query_alerts<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    start_after: Option<HumanAddr>,
    limit: Option<u32>,
    order_by: Option<OrderBy>,
) -> StdResult<GetAlertsResponse> {
    let start_after = if let Some(start_after) = start_after {
        Some(deps.api.canonical_address(&start_after)?)
    } else {
        None
    };

    let alerts: Vec<Alert> = read_alerts(&deps.storage, start_after, limit, order_by)?;

    Ok(GetAlertsResponse { alerts })
}

fn handle_query_subscriptions_for_address<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    subscriber_addr: HumanAddr,
    start_after: Option<HumanAddr>,
    limit: Option<u32>,
    order_by: Option<OrderBy>,
) -> StdResult<GetSubscriptionsForAddressResponse> {
    let start_after = if let Some(start_after) = start_after {
        Some(deps.api.canonical_address(&start_after)?)
    } else {
        None
    };
    let sender_addr = deps.api.canonical_address(&subscriber_addr)?;
    let subscriptions: Vec<Subscription> =
        read_subscriptions_for_address(&deps.storage, sender_addr, start_after, limit, order_by)?;

    Ok(GetSubscriptionsForAddressResponse { subscriptions })
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env};
    // use cosmwasm_std::{coins, from_binary, StdError};

    // #[test]
    // fn proper_initialization() {
    //     let mut deps = mock_dependencies(20, &[]);

    //     let msg = InitMsg { count: 17 };
    //     let env = mock_env("creator", &coins(1000, "earth"));

    //     // we can just call .unwrap() to assert this was a success
    //     let res = init(&mut deps, env, msg).unwrap();
    //     assert_eq!(0, res.messages.len());

    //     // it worked, let's query the state
    //     let res = query(&deps, QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(17, value.count);
    // }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies(20, &coins(2, "token"));

    //     let msg = InitMsg { count: 17 };
    //     let env = mock_env("creator", &coins(2, "token"));
    //     let _res = init(&mut deps, env, msg).unwrap();

    //     // beneficiary can release it
    //     let env = mock_env("anyone", &coins(2, "token"));
    //     let msg = HandleMsg::Increment {};
    //     let _res = handle(&mut deps, env, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(&deps, QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies(20, &coins(2, "token"));

    //     let msg = InitMsg { count: 17 };
    //     let env = mock_env("creator", &coins(2, "token"));
    //     let _res = init(&mut deps, env, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_env = mock_env("anyone", &coins(2, "token"));
    //     let msg = HandleMsg::Reset { count: 5 };
    //     let res = handle(&mut deps, unauth_env, msg);
    //     match res {
    //         Err(StdError::Unauthorized { .. }) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_env = mock_env("creator", &coins(2, "token"));
    //     let msg = HandleMsg::Reset { count: 5 };
    //     let _res = handle(&mut deps, auth_env, msg).unwrap();

    //     // should now be 5
    //     let res = query(&deps, QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
