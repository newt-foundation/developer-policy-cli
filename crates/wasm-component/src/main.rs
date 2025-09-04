mod bindings;
mod wasi_fetcher;

use shared::{print_log, tokens::address_to_coingecko};
use wasi_fetcher::TradingAgent;

fn main() {
    // single CLI arg containing a JSON array of addresses
    let addresses: Vec<&str> = vec![
        "0xe42e3458283032c669c98e0d8f883a92fc64fe22",
        "0xba2ae424d960c26247dd6c32edc70b295c744c43",
        "0xe68249a2A7A19082c85E1D809B154fD17483D7CF",
    ];
    let addresses: Vec<String> = addresses.into_iter().map(|a| a.to_lowercase()).collect();

    // map addresses -> coin ids
    let addr_to_id = address_to_coingecko();
    let coin_ids: Vec<&str> = addresses
        .iter()
        .filter_map(|a| addr_to_id.get(a.as_str()).copied())
        .collect();

    // check if debug mode is enabled
    let arg = std::env::args()
        .nth(1)
        .unwrap_or_default();
    std::env::set_var("ENV", arg);
    
    print_log("Analyzing market and computing trading signals for profitable trades");
    
    let agent = TradingAgent::new(None);
    match agent.compute_trading_signal(&coin_ids) {
        Ok(trading_signal) => {
            print_log("Trading signals computed successfully");
            print_log(&format!("Result:\n{}", trading_signal.display_pretty()));
            if std::env::var("ENV").unwrap_or_default() != "development" {
                println!("{}", serde_json::to_string_pretty(&trading_signal).unwrap());
            }
        },
        Err(e) => {
            eprintln!("[Trading Agent] Failed to compute trading signals: {:#}", e);
        }
    }
}
