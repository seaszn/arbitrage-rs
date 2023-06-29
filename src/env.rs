use dotenv::dotenv;
use crate::networks;

pub fn init_environment() {
    dotenv().ok();

    let config: Configuration = get_environment_config();
    let network: networks::Network = networks::get_network(config.chain_id);

    println!("{}", network.exchanges.len())
}

struct Configuration {
    pub chain_id: i32,
    pub rpc_endpoint: String,
    pub bundle_executor_address: String,
    pub private_key: String,
    pub min_route_length: i32,
    pub max_route_length: i32,
    pub min_market_reserves: f32,
}

fn get_environment_config() -> Configuration {
    // Network chain id
    let chain_id: i32 = std::env::var("CHAIN_ID")
        .expect("CHAIN_ID must be set.")
        .parse()
        .unwrap();

    // Json-Rpc enpoint
    let rpc_endpoint: String = std::env::var("RPC_ENDPOINT").expect("RPC_ENPOINT must be set.");

    // Bundle Executor address
    let bundle_executor_address: String =
        std::env::var("BUNDLE_EXECUTOR").expect("BUNLDE_EXECUTOR must be set.");

    // Private key
    let private_key: String = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set.");

    // Minimum route length
    let min_route_length: i32 = std::env::var("MIN_ROUTE_LENGTH")
        .expect("MIN_ROUTE_LENGTH must be set.")
        .parse()
        .unwrap();

    // Maximum route length
    let max_route_length: i32 = std::env::var("MAX_ROUTE_LENGTH")
        .expect("MAX_ROUTE_LENGTH must be set.")
        .parse()
        .unwrap();

    // Maximum route length
    let min_market_reserves: f32 = std::env::var("MIN_MARKET_RESERVES")
        .expect("MIN_MARKET_RESERVES must be set.")
        .parse()
        .unwrap();

    let config: Configuration = Configuration {
        chain_id,
        rpc_endpoint,
        bundle_executor_address,
        private_key,
        min_route_length,
        max_route_length,
        min_market_reserves,
    };

    return config;
}
