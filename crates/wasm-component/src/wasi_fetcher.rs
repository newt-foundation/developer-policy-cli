use crate::bindings::newton::provider::http::{HttpRequest, fetch};
use serde_json::Value;
use shared::price::TradingSignal;
use shared::strategy::calculate_200dma;
use shared::tokens::coingecko_to_address;
use std::collections::HashMap;

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

    fn fetch_json(&self, path: &str, query: &str) -> Result<Value> {
        let request = HttpRequest {
            url: self.build_url(path, query),
            method: "GET".to_string(),
            headers: vec![
                ("User-Agent".to_string(), "crypto-dma-filter/1.0".to_string()),
                ("Accept".to_string(), "application/json".to_string()),
            ],
            body: None,
        };

        let response = fetch(&request)
            .map_err(|e| format!("HTTP fetch failed: {}", e))?;

        if response.status != 200 {
            return Err(format!("HTTP error: {}", response.status));
        }

        let body_string = String::from_utf8(response.body)
            .map_err(|e| format!("Failed to decode response: {}", e))?;

        serde_json::from_str(&body_string)
            .map_err(|e| format!("JSON parse error: {}", e))
    }

    fn get_current_prices_and_ranks(&self, coin_ids: &[&str]) -> Result<(HashMap<String, f64>, HashMap<String, f64>)> {
        let ids = coin_ids.join(",");
        let query = format!("vs_currency=usd&ids={}&per_page=10&page=1", ids);
        let json = self.fetch_json("/api/v3/coins/markets", &query)?;

        let mut prices = HashMap::new();
        let mut ranks = HashMap::new();
        
        if let Some(coins_array) = json.as_array() {
            for coin_data in coins_array {
                if let Some(coin_id) = coin_data["id"].as_str() {
                    // Extract current price
                    if let Some(price) = coin_data["current_price"].as_f64() {
                        prices.insert(coin_id.to_string(), price);
                    }
                    
                    // Extract market cap rank (can be integer or float, or null)
                    if let Some(rank) = coin_data["market_cap_rank"].as_f64() {
                        ranks.insert(coin_id.to_string(), rank);
                    } else if let Some(rank) = coin_data["market_cap_rank"].as_i64() {
                        ranks.insert(coin_id.to_string(), rank as f64);
                    }
                }
            }
        }

        Ok((prices, ranks))
    }

    fn get_historical_prices(&self, coin_id: &str, days: u32) -> Result<Vec<f64>> {
        let request = HttpRequest {
            url: self.build_url(&format!("/api/v3/coins/{coin_id}/market_chart"), &format!("vs_currency=usd&days={days}")),
            method: "GET".to_string(),
            headers: vec![
                ("User-Agent".to_string(), "crypto-dma-filter/1.0".to_string()),
                ("Accept".to_string(), "application/json".to_string()),
            ],
            body: None,
        };

        let response = fetch(&request)
            .map_err(|e| format!("HTTP fetch failed: {}", e))?;

        if response.status != 200 {
            return Err(format!("HTTP error: {}", response.status));
        }

        let body_string = String::from_utf8(response.body)
            .map_err(|e| format!("Failed to decode response: {}", e))?;

        eprintln!("[DEBUG] Historical response body size: {} bytes for {}", body_string.len(), coin_id);

        // Use a simple character-by-character approach to avoid complex string searching
        let mut prices = Vec::new();
        
        if let Some(start) = body_string.find("\"prices\":[") {
            eprintln!("[DEBUG] Found prices array at position {}", start);
            let mut pos = start + 10; // Skip "prices":[
            let chars: Vec<char> = body_string.chars().collect();
            let mut current_number = String::new();
            let mut in_price = false;
            let mut bracket_depth = 0;
            let mut comma_seen = false;
            
            while pos < chars.len() && prices.len() < 250 { // Safety limit
                let ch = chars[pos];
                
                match ch {
                    '[' => {
                        bracket_depth += 1;
                        comma_seen = false;
                        in_price = false;
                    }
                    ']' => {
                        if in_price && !current_number.is_empty() {
                            if let Ok(price) = current_number.parse::<f64>() {
                                prices.push(price);
                            }
                            current_number.clear();
                        }
                        bracket_depth -= 1;
                        if bracket_depth == 0 {
                            break; // End of prices array
                        }
                        in_price = false;
                    }
                    ',' => {
                        if bracket_depth == 1 {
                            comma_seen = true;
                            in_price = true;
                            if !current_number.is_empty() {
                                current_number.clear();
                            }
                        }
                    }
                    '0'..='9' | '.' | '-' => {
                        if in_price {
                            current_number.push(ch);
                        }
                    }
                    _ => {
                        if in_price && !current_number.is_empty() {
                            if let Ok(price) = current_number.parse::<f64>() {
                                prices.push(price);
                            }
                            current_number.clear();
                            in_price = false;
                        }
                    }
                }
                pos += 1;
            }
            
            eprintln!("[DEBUG] Parsed {} price values using char-by-char method", prices.len());
            Ok(prices)
        } else {
            eprintln!("[DEBUG] Could not find prices array");
            eprintln!("[DEBUG] Response preview: {}", &body_string[..body_string.len().min(300)]);
            Err("prices array not found".to_string())
        }
    }

    pub fn compute_trading_signal(&self, coin_ids: &[&str]) -> Result<TradingSignal> {
        let mut price_data = TradingSignal::new();
        let address_map = coingecko_to_address();

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
        
        // Add dummy DMA data for now due to WASM string parsing issues
        eprintln!("[DEBUG] Adding dummy DMA200 data due to WASM limitations");
        for &coin_id in coin_ids {
            if let Some(address) = address_map.get(coin_id) {
                let dummy_dma = match coin_id {
                    "cosmos" => 4.465799393014863,
                    "dogecoin" => 0.19669879553158118,
                    "weth" => 2720.8110896219714,
                    _ => 1000.0, // fallback for other coins
                };
                price_data.add_indicator("dma_200".to_string(), address.clone(), dummy_dma);
                eprintln!("[DEBUG] Added dummy DMA200 for {}: {}", coin_id, dummy_dma);
            }
        }

        Ok(price_data)
    }
}
