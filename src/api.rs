use crate::models::{Coin, HistoricalData};
use crate::config::{ApiServer, Config};
use crate::bindings::wasi::http::types::*;
use crate::bindings::wasi::http::outgoing_handler;
use crate::bindings::wasi::io::poll;

pub struct ApiClient;

impl ApiClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get_coins(&self, config: &Config) -> Result<Vec<Coin>, String> {
        if config.use_specific_coins() {
            self.get_coins_by_ids(&config.specific_coins, config)
        } else {
            self.get_top_coins(config.limit, config)
        }
    }

    fn get_top_coins(&self, limit: u32, config: &Config) -> Result<Vec<Coin>, String> {
        let (host, url) = match config.api_server {
            ApiServer::CoinGecko => {
                let url = format!(
                    "/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page={}&page=1&sparkline=false",
                    limit
                );
                ("api.coingecko.com", url)
            }
            ApiServer::CoinMarketCap => {
                return Err("CoinMarketCap not implemented yet".to_string());
            }
        };
        
        let response = self.make_request(host, &url)?;
        
        if config.verbose {
            eprintln!("Raw response: {}", response.chars().take(200).collect::<String>());
            eprintln!("Response length: {} bytes", response.len());
        }
        
        // Check if response looks like an error
        if response.contains("error") || response.contains("<!DOCTYPE") || response.starts_with("{") && !response.starts_with("[") {
            return Err(format!("API Error Response: {}", response.chars().take(500).collect::<String>()));
        }
        
        serde_json::from_str(&response).map_err(|e| {
            if config.verbose {
                eprintln!("JSON error at: {}", response.chars().take(200).collect::<String>());
            }
            format!("JSON parse error: {}. Response preview: {}", e, response.chars().take(200).collect::<String>())
        })
    }

    fn get_coins_by_ids(&self, coin_ids: &[String], config: &Config) -> Result<Vec<Coin>, String> {
        let (host, url) = match config.api_server {
            ApiServer::CoinGecko => {
                let ids_str = coin_ids.join(",");
                let url = format!(
                    "/api/v3/coins/markets?vs_currency=usd&ids={}&order=market_cap_desc&sparkline=false",
                    ids_str
                );
                ("api.coingecko.com", url)
            }
            ApiServer::CoinMarketCap => {
                return Err("CoinMarketCap not implemented yet".to_string());
            }
        };
        
        let response = self.make_request(host, &url)?;
        
        if config.verbose {
            eprintln!("Response for specific coins: {}", response.chars().take(200).collect::<String>());
        }
        
        serde_json::from_str(&response).map_err(|e| format!("JSON parse error: {}", e))
    }

    pub fn get_historical_data(&self, coin_id: &str, days: u32, config: &Config) -> Result<Vec<f64>, String> {
        let url = format!(
            "/api/v3/coins/{}/market_chart?vs_currency=usd&days={}&interval=daily",
            coin_id, days
        );

        let response = self.make_request("api.coingecko.com", &url)?;
        
        if config.verbose {
            eprintln!("Historical data response for {}: {}", coin_id, response.chars().take(300).collect::<String>());
        }
        
        let historical: HistoricalData = serde_json::from_str(&response)
            .map_err(|e| format!("JSON parse error for {}: {}. Response: {}", coin_id, e, response.chars().take(300).collect::<String>()))?;
        
        let prices: Vec<f64> = historical.prices
            .iter()
            .map(|price_data| price_data[1])
            .collect();

        Ok(prices)
    }

    fn make_request(&self, host: &str, path: &str) -> Result<String, String> {
        let headers = Fields::new();
        
        // Add headers to appear more legitimate
        headers.set(&"User-Agent".to_string(), &[b"crypto-dma-filter/1.0".to_vec()])
            .map_err(|_| "Failed to set User-Agent")?;
        headers.set(&"Accept".to_string(), &[b"application/json".to_vec()])
            .map_err(|_| "Failed to set Accept")?;
        
        let request = OutgoingRequest::new(headers);
        
        request.set_method(&Method::Get)
            .map_err(|_| "Failed to set method")?;
        request.set_scheme(Some(&Scheme::Https))
            .map_err(|_| "Failed to set scheme")?;
        request.set_authority(Some(host))
            .map_err(|_| "Failed to set authority")?;
        request.set_path_with_query(Some(path))
            .map_err(|_| "Failed to set path")?;
        
        let future = outgoing_handler::handle(request, Some(RequestOptions::new()))
            .map_err(|e| format!("Failed to create request: {:?}", e))?;
        
        poll::poll(&[&future.subscribe()]);
        
        let response = future.get()
            .ok_or("Response not ready")?
            .map_err(|_| "Future failed")?
            .map_err(|e| format!("Request failed: {:?}", e))?;
        
        let body = response.consume()
            .map_err(|_| "Failed to consume response")?;
        let stream = body.stream()
            .map_err(|_| "Failed to get input stream")?;
        
        let mut data = Vec::new();
        loop {
            match stream.blocking_read(8192) {
                Ok(chunk) if !chunk.is_empty() => {
                    data.extend(chunk);
                }
                Ok(_) => break, // Empty chunk, end of stream
                Err(_) => break, // Error reading
            }
        }
        
        String::from_utf8(data).map_err(|_| "Invalid UTF-8 response".to_string())
    }
}

pub fn calculate_200dma(prices: &[f64]) -> Option<f64> {
    if prices.len() < 200 {
        return None;
    }

    let recent_200: Vec<f64> = prices.iter().rev().take(200).cloned().collect();
    let sum: f64 = recent_200.iter().sum();
    Some(sum / 200.0)
}