use std::{io::Error, vec};

use ethers::types::U256;

use crate::{
    env::{RuntimeCache, RuntimeConfig},
    exchanges::types::Protocol,
    handlers::types::BalanceChange,
    networks::Network,
    types::{market::Market, OrgValue, OrganizedList, Reserves, TransactionLog},
};

use self::types::Exchange;

mod stable_swap;
pub mod types;
mod uniswap_v2;

pub async fn get_exchange_markets(
    network: &'static Network,
    runtime_cache: &RuntimeCache,
    runtime_config: &'static RuntimeConfig,
) -> Result<Vec<Market>, Error> {
    let mut result: Vec<Market> = vec![];

    for exchange in &network.exchanges {
        if exchange.protocol == Protocol::UniswapV2 {
            if let Ok(mut response) =
                uniswap_v2::get_markets(exchange, network, runtime_cache, runtime_config).await
            {
                result.append(&mut response);
            };
        } else if exchange.protocol == Protocol::StableSwap {
            // return stable_swap::get_markets(exchange);
        }
    }

    return Ok(result);
}

#[inline(always)]
pub fn parse_balance_changes(
    logs: Vec<TransactionLog>,
    runtime_cache: &'static RuntimeCache,
) -> Vec<BalanceChange> {
    let mut result: Vec<BalanceChange> = vec![];

    // Uniswap V2
    result.append(&mut uniswap_v2::parse_balance_changes(
        logs.iter()
            .filter(|x| x.protocol == Protocol::UniswapV2)
            .collect(),
        runtime_cache,
    ));

    return result;
}

#[inline(always)]
pub async fn get_market_reserves(
    markets: &'static OrganizedList<Market>,
    runtime_cache: &'static RuntimeCache,
    runtime_config: &'static RuntimeConfig,
) -> OrganizedList<Reserves> {
    let filtered_markets: Vec<&'static OrgValue<Market>> = markets.filter(|x| {
        x.value.protocol == Protocol::UniswapV2 || x.value.protocol == Protocol::StableSwap
    });

    // Uniswap V2
    let uniswap_v2_markets =
        uniswap_v2::get_market_reserves(filtered_markets, runtime_cache, runtime_config).await;

    return uniswap_v2_markets;
}

#[inline(always)]
pub fn calculate_amount_out(
    reserves: &Reserves,
    input_amount: &U256,
    market: &Market
) -> U256 {
    let protocol = &market.protocol;

    if protocol == &Protocol::UniswapV2 || (protocol == &Protocol::StableSwap && market.stable == false) {
        return uniswap_v2::calculate_amount_out(market, reserves, input_amount);
    }

    return U256::zero();
}
