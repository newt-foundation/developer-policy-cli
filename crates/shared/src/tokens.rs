use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Address = String;

// hardcoded addresses for mainnet
pub const USDC_ADDRESS: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
pub const WETH_ADDRESS: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
pub const DOGE_ADDRESS: &str = "0xba2ae424d960c26247dd6c32edc70b295c744c43";
pub const DOT_ADDRESS: &str = "0x8d010bf9c26881788b4e6bf5fd1bdc358c8f90b8";
pub const VIRTUAL_ADDRESS: &str = "0x44ff8620b8ca30902395a7bd3f2407e1a091bf73";
pub const DEXE_ADDRESS: &str = "0xde4ee8057785a7e8e800db58f9784845a5c2cbd6";

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
        ( // mockerc20
            "weth",
            TokenInfo {
                symbol: "WETH",
                address: "0xe42e3458283032c669c98e0d8f883a92fc64fe22".to_string(),
                coingecko_id: "weth",
            },
        ),
        ( // mockerc20
            "cosmos",
            TokenInfo {
                symbol: "ATOM",
                address: "0xe68249a2a7a19082c85e1d809b154fd17483d7cf".to_string(),
                coingecko_id: "cosmos",
            },
        ),
        (
            "doge",
            TokenInfo {
                symbol: "DOGE",
                address: DOGE_ADDRESS.to_string(),
                coingecko_id: "dogecoin",
            },
        ),
        (
            "polkadot",
            TokenInfo {
                symbol: "DOT",
                address: DOT_ADDRESS.to_string(),
                coingecko_id: "polkadot",
            },
        ),
        (
            "virtual-protocol",
            TokenInfo {
                symbol: "VIRTUAL",
                address: VIRTUAL_ADDRESS.to_string(),
                coingecko_id: "virtual-protocol",
            },
        ),
        (
            "dexe",
            TokenInfo {
                symbol: "DEXE",
                address: DEXE_ADDRESS.to_string(),
                coingecko_id: "dexe",
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
