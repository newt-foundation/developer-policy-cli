use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Address = String;

// hardcoded addresses for sepolia testnet
pub const USDC_ADDRESS: &str = "0xd1c01582bee80b35898cc3603b75dbb5851b4a85";
pub const WETH_ADDRESS: &str = "0xe42e3458283032c669c98e0d8f883a92fc64fe22";
pub const DOGE_ADDRESS: &str = "0xba2ae424d960c26247dd6c32edc70b295c744c43";
pub const ATOM_ADDRESS: &str = "0x0eb3a705fc54725037cc9e008bdede697f62f335";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub symbol: &'static str,
    pub address: Address,
    pub coingecko_id: &'static str,
}

// good enough as a static hashmap
pub fn get_token_map() -> HashMap<&'static str, TokenInfo> {
    [
        (
            "usd-coin",
            TokenInfo {
                symbol: "USDC",
                address: USDC_ADDRESS.to_string(),
                coingecko_id: "usd-coin",
            },
        ),
        (
            "weth",
            TokenInfo {
                symbol: "WETH",
                address: WETH_ADDRESS.to_string(),
                coingecko_id: "weth",
            },
        ),
        (
            "doge",
            TokenInfo {
                symbol: "DOGE",
                address: DOGE_ADDRESS.to_string(),
                coingecko_id: "doge",
            },
        ),
        (
            "cosmos",
            TokenInfo {
                symbol: "ATOM",
                address: ATOM_ADDRESS.to_string(),
                coingecko_id: "cosmos",
            },
        ),
    ]
    .into()
}

pub fn coingecko_to_address() -> HashMap<&'static str, Address> {
    get_token_map()
        .into_values()
        .map(|info| (info.coingecko_id, info.address))
        .collect()
}

pub fn address_to_coingecko() -> HashMap<Address, &'static str> {
    get_token_map()
        .into_values()
        .map(|info| (info.address, info.coingecko_id))
        .collect()
}
