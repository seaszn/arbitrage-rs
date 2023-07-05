use std::{sync::Arc, vec};

use crate::{
    env::types::{RuntimeClient, UniswapQueryContract},
    exchanges::types::Protocol,
    handlers::types::swap::Swap,
    networks::Network,
    types::{market::Market, TransactionLog},
};

use self::types::Exchange;

mod stable_swap;
pub mod types;
mod uniswap_v2;

pub async fn get_exchange_markets(
    exchange: &Exchange,
    network: Arc<Network>,
    client: RuntimeClient,
    uniswap_query: UniswapQueryContract,
) -> Vec<Arc<Market>> {
    println!("Loading markets from factory {}", exchange.factory_address);

    if exchange.protocol == Protocol::UniswapV2 {
        return uniswap_v2::get_markets(exchange, network, client, uniswap_query).await;
    } else if exchange.protocol == Protocol::StableSwap {
        // return stable_swap::get_markets(exchange);
    }

    return vec![];
}

pub fn parse_balance_changes(logs: Vec<TransactionLog>) -> Vec<Swap> {
    let mut result: Vec<Swap> = vec![];

    // Uniswap V2
    if logs
        .clone()
        .into_iter()
        .any(|x| x.protocol == Protocol::UniswapV2)
    {
        result.append(&mut uniswap_v2::parse_balance_changes(
            logs.clone()
                .into_iter()
                .filter(|x| x.protocol == Protocol::UniswapV2)
                .collect(),
        ));
    }

    // // Stable swap
    // if logs
    //     .clone()
    //     .into_iter()
    //     .any(|x| x.protocol == Protocol::UniswapV2)
    // {
    //     result.append(&mut uniswap_v2::parse_balance_changes(
    //         logs.clone()
    //             .into_iter()
    //             .filter(|x| x.protocol == Protocol::UniswapV2)
    //             .collect(),
    //     ));
    // }

    return result;
}
