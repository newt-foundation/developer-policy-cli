mod bindings;
mod wasi_fetcher;

use shared::tokens::address_to_coingecko;
use wasi_fetcher::WasiPriceFetcher;

fn main() {
    // single CLI arg containing a JSON array of addresses
    let arg = std::env::args()
        .nth(1)
        .expect("pass JSON array of addresses");
    let addresses: Vec<String> =
        serde_json::from_str(&arg).expect("invalid JSON array of addresses");
    let addresses: Vec<String> = addresses.into_iter().map(|a| a.to_lowercase()).collect();

    // map addresses -> coin ids
    let addr_to_id = address_to_coingecko();
    let coin_ids: Vec<&str> = addresses
        .iter()
        .filter_map(|a| addr_to_id.get(a.as_str()).copied())
        .collect();

    let fetcher = WasiPriceFetcher::new(None);
    match fetcher.get_price_data(&coin_ids) {
        Ok(price_data) => match serde_json::to_string_pretty(&price_data) {
            Ok(out) => println!("{out}"),
            Err(e) => eprintln!("Failed to serialize output: {e}"),
        },
        Err(e) => {
            eprintln!("Error: {:#}", e.wrap_err("failed to fetch price data"));
        }
    }
}
