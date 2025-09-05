mod bindings;
mod wasi_fetcher;

use bindings::Guest;
use shared::tokens::address_to_coingecko;
use wasi_fetcher::TradingAgent;

struct Component;

impl Guest for Component {
    fn run(input: String) -> Result<String, String> {
        execute_trading_logic(&input)
    }
}

fn execute_trading_logic(_input: &str) -> Result<String, String> {
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

    let agent = TradingAgent::new(None);
    let trading_signal = agent
        .compute_trading_signal(&coin_ids)
        .map_err(|e| format!("Failed to compute trading signals: {}", e))?;

    serde_json::to_string_pretty(&trading_signal)
        .map_err(|e| format!("Failed to serialize result: {}", e))
}

bindings::export!(Component with_types_in bindings);

fn main() {
    let input = std::env::args().nth(1).unwrap_or_default();
    match execute_trading_logic(&input) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
