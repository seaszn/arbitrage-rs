use crate::networks;

mod tracer;
mod arbitrum;
mod utils;
pub mod types;

pub async fn init(chain_id: u32) {
    if chain_id == networks::arbitrum {
        _ = arbitrum::init().await;
    }
}
