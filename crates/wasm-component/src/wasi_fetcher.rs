use crate::bindings::newton::provider::http::{fetch, HttpRequest};
use shared::price::TradingSignal;
use shared::print_log;
use shared::strategy::{calculate_200dma, filter_profitable_trades, ProfitTrade};
use shared::tokens::coingecko_to_address;
use std::collections::HashMap;
use tinyjson::JsonValue;

type Result<T> = std::result::Result<T, String>;

pub struct TradingAgent {
    api_key: Option<String>,
}

impl TradingAgent {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }

    fn build_url(&self, path: &str, query: &str) -> String {
        let mut full_query = query.to_string();
        if let Some(key) = &self.api_key {
            if !full_query.is_empty() {
                full_query.push('&');
            }
            full_query.push_str(&format!("x_cg_demo_api_key={key}"));
        }

        if full_query.is_empty() {
            format!("https://api.coingecko.com{path}")
        } else {
            format!("https://api.coingecko.com{path}?{full_query}")
        }
    }

    fn get_current_prices_and_ranks(
        &self,
        coin_ids: &[&str],
    ) -> Result<(HashMap<String, f64>, HashMap<String, f64>)> {
        let ids = coin_ids.join(",");
        let query = format!("vs_currency=usd&ids={}&per_page=10&page=1", ids);

        let request = HttpRequest {
            url: self.build_url("/api/v3/coins/markets", &query),
            method: "GET".to_string(),
            headers: vec![
                (
                    "User-Agent".to_string(),
                    "crypto-dma-filter/1.0".to_string(),
                ),
                ("Accept".to_string(), "application/json".to_string()),
            ],
            body: None,
        };

        let response = fetch(&request).map_err(|e| format!("HTTP fetch failed: {}", e))?;
        if response.status != 200 {
            return Err(format!("HTTP error: {}", response.status));
        }
        let body_string = String::from_utf8(response.body)
            .map_err(|e| format!("Failed to decode response: {}", e))?;
        let parsed: JsonValue = body_string
            .parse()
            .map_err(|e| format!("JSON parse error: {}", e))?;

        let mut prices = HashMap::new();
        let mut ranks = HashMap::new();

        if let Some(coins_array) = parsed.get::<Vec<JsonValue>>() {
            for coin_data in coins_array {
                if let Some(coin_id) = coin_data["id"].get::<String>() {
                    // extract current price
                    if let Some(price_ref) = coin_data["current_price"].get::<f64>() {
                        prices.insert(coin_id.clone(), *price_ref);
                    }

                    // extract market cap rank
                    if let Some(rank_ref) = coin_data["market_cap_rank"].get::<f64>() {
                        ranks.insert(coin_id.clone(), *rank_ref);
                    }
                }
            }
        }

        Ok((prices, ranks))
    }

    fn get_historical_prices(&self, coin_id: &str, days: u32) -> Result<Vec<f64>> {
        let request = HttpRequest {
            url: self.build_url(
                &format!("/api/v3/coins/{coin_id}/market_chart"),
                &format!("vs_currency=usd&days={days}"),
            ),
            method: "GET".to_string(),
            headers: vec![
                (
                    "User-Agent".to_string(),
                    "crypto-dma-filter/1.0".to_string(),
                ),
                ("Accept".to_string(), "application/json".to_string()),
            ],
            body: None,
        };

        let response = fetch(&request).map_err(|e| format!("HTTP fetch failed: {}", e))?;

        if response.status != 200 {
            return Err(format!("HTTP error: {}", response.status));
        }

        let body_string = String::from_utf8(response.body)
            .map_err(|e| format!("Failed to decode response: {}", e))?;

        let parsed: JsonValue = body_string
            .parse()
            .map_err(|e| format!("JSON parse error: {}", e))?;

        let mut prices = Vec::new();
        if let Some(obj) = parsed.get::<std::collections::HashMap<String, JsonValue>>() {
            if let Some(prices_val) = obj.get("prices") {
                if let Some(price_points) = prices_val.get::<Vec<JsonValue>>() {
                    for point in price_points.iter() {
                        if let Some(pair) = point.get::<Vec<JsonValue>>() {
                            if pair.len() > 1 {
                                if let Some(price_ref) = pair[1].get::<f64>() {
                                    prices.push(*price_ref);
                                }
                            }
                        }
                    }
                }
            }
        }

        if prices.is_empty() {
            eprintln!("[DEBUG] Could not parse prices array");
            eprintln!(
                "[DEBUG] Response preview: {}",
                &body_string[..body_string.len().min(300)]
            );
            return Err("prices array not found".to_string());
        }

        Ok(prices)
    }

    pub fn compute_trading_signal(&self, coin_ids: &[&str]) -> Result<TradingSignal> {
        let mut price_data = TradingSignal::new();
        let address_map = coingecko_to_address();

        print_log("Getting current prices and market cap ranks...");

        let (current_prices, market_cap_ranks) = self.get_current_prices_and_ranks(coin_ids)?;

        // store current prices
        for (coin_id, price) in current_prices {
            if let Some(address) = address_map.get(coin_id.as_str()) {
                price_data.add_price(address.clone(), price);
            }
        }

        // store market cap ranks
        for (coin_id, rank) in market_cap_ranks {
            if let Some(address) = address_map.get(coin_id.as_str()) {
                price_data.add_indicator("market_cap_rank".to_string(), address.clone(), rank);
            }
        }

        print_log("Calculating daily moving average for candidates...");
        // compute 200DMA using historical prices
        for &coin_id in coin_ids {
            if let Some(address) = address_map.get(coin_id) {
                match self.get_historical_prices(coin_id, 250) {
                    Ok(prices) => {
                        if let Some(dma) = calculate_200dma(&prices) {
                            price_data.add_indicator("dma_200".to_string(), address.clone(), dma);
                        } else {
                            eprintln!("[DEBUG] Not enough data to compute 200DMA for {}", coin_id);
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "[DEBUG] Failed to fetch historical prices for {}: {}",
                            coin_id, e
                        );
                    }
                }
            }
        }

        Ok(price_data)
    }

    pub fn filter_profitable_trades(&self, trading_signal: &TradingSignal) -> Result<Vec<ProfitTrade>> {
        print_log("Filtering profitable trades...");
        let profitable_trades = filter_profitable_trades(trading_signal);
        Ok(profitable_trades)
    }
}
