use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::models::{Alert, AlertField, OrderBy};
use crate::msg::{GetAlertsResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{
    read_alerts, read_config, read_subscriptions_for_address, store_alert, store_config, Config,
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
        HandleMsg::SubscribeAlert { alert_key } => Ok(HandleResponse::default()),
        HandleMsg::UnsubscribeAlert { alert_key } => Ok(HandleResponse::default()),
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

// pub fn try_reset<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: Env,
//     count: i32,
// ) -> StdResult<HandleResponse> {
//     let api = &deps.api;

//     // if api.canonical_address(&env.message.sender)? != config.owner {
//     //   store_config(&mut deps.storage)
//     // }

//     Ok(HandleResponse::default())
// }

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
        QueryMsg::GetSubscriptions {
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
) -> StdResult<GetAlertsResponse> {
    let start_after = if let Some(start_after) = start_after {
        Some(deps.api.canonical_address(&start_after)?)
    } else {
        None
    };
    let sender_addr = deps.api.canonical_address(&subscriber_addr)?;
    let alerts: Vec<Subscription> =
        read_subscriptions_for_address(&deps.storage, sender_addr, start_after, limit, order_by)?;

    Ok(GetAlertsResponse { alerts })
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
