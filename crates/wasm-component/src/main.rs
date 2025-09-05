mod bindings;
mod wasi_fetcher;
mod config;

use dotenvy::dotenv;
use bindings::Guest;
use shared::{print_log, tokens::address_to_coingecko};
use wasi_fetcher::TradingAgent;

struct Component;

impl Guest for Component {
    fn run(input: String) -> Result<String, String> {
        dotenv().unwrap_or_default();
        execute_trading_logic(&input)
    }
}

fn execute_trading_logic(input: &str) -> Result<String, String> {
    std::env::set_var("ENV", input);
    if input == "development" {
        println!("");
    }
    print_log(&format!("{} mode", input));

    let addresses = [
        "0xe42e3458283032c669c98e0d8f883a92fc64fe22",
        "0xba2ae424d960c26247dd6c32edc70b295c744c43",
        "0xe68249a2A7A19082c85E1D809B154fD17483D7CF",
    ];

    let addr_to_id = address_to_coingecko();
    let coin_ids: Vec<&str> = addresses
        .iter()
        .map(|a| a.to_lowercase())
        .filter_map(|a| addr_to_id.get(a.as_str()).copied())
        .collect();
      
    print_log("Analyzing market and computing trading signals for profitable trades");

    let api_key = match std::env::var("COINGECKO_API_KEY") {
        Ok(key) => Some(key),
        Err(_) => None,
    };
    let agent = TradingAgent::new(api_key);
    let trading_signal = agent
        .compute_trading_signal(&coin_ids)
        .map_err(|e| format!("Failed to compute trading signals: {}", e))?;

    print_log("Trading signals computed successfully");
    print_log(&trading_signal.display_pretty());
    
    if std::env::var("ENV").unwrap_or_default() == "development" {
        let profitable_trades = agent.filter_profitable_trades(&trading_signal)
            .map_err(|e| format!("Failed to filter profitable trades: {}", e))?;
        print_log("Profitable trades filtered successfully");
        print_log(&serde_json::to_string_pretty(&profitable_trades).unwrap());
    }

    Ok(serde_json::to_string(&trading_signal).unwrap())
}

bindings::export!(Component with_types_in bindings);

fn main() {
    let input = std::env::args().nth(1).unwrap_or_default();
    std::env::set_var("ENV", input.clone());
    dotenv().unwrap_or_default();
    match execute_trading_logic(&input) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
