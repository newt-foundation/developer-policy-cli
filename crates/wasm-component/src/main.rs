mod bindings;
mod wasi_fetcher;

use serde_json::json;
use shared::tokens::address_to_coingecko;
use wasi_fetcher::TradingAgent;

fn main() {
    // single CLI arg containing a JSON array of addresses
    let addresses: Vec<&str> = vec![
        "0xe42e3458283032c669c98e0d8f883a92fc64fe22",
        "0xba2ae424d960c26247dd6c32edc70b295c744c43",
        "0x0eb3a705fc54725037cc9e008bdede697f62f335",
    ];
    let addresses: Vec<String> = addresses.into_iter().map(|a| a.to_lowercase()).collect();

    // map addresses -> coin ids
    let addr_to_id = address_to_coingecko();
    let coin_ids: Vec<&str> = addresses
        .iter()
        .filter_map(|a| addr_to_id.get(a.as_str()).copied())
        .collect();

    println!("[Trading Agent] Analyzing market and computing trading signals for profitable trades");
    
    let agent = TradingAgent::new(None);
    match agent.compute_trading_signal(&coin_ids) {
        Ok(price_data) => {
            let out = json!(&price_data);
            println!("[Trading Agent] Trading signals computed successfully");
            println!("Result: {}", serde_json::to_string_pretty(&out).unwrap());
        },
        Err(e) => {
            eprintln!("[Trading Agent] Failed to compute trading signals: {:#}", e);
        }
    }
}
