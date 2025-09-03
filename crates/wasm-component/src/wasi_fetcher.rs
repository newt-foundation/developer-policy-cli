use crate::bindings::wasi::http::outgoing_handler::{handle, OutgoingRequest, RequestOptions};
use crate::bindings::wasi::http::types::{Fields, Method, Scheme};
use crate::bindings::wasi::io::poll;
use crate::bindings::wasi::io::streams::StreamError;
use eyre::{eyre, Result};
use serde_json::Value;
use shared::price::PriceData;
use shared::strategy::calculate_200dma;
use shared::tokens::coingecko_to_address;
use std::collections::HashMap;

pub struct WasiPriceFetcher {
    api_key: Option<String>,
}

impl WasiPriceFetcher {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }

    fn build_request(&self, path: &str, query: &str) -> Result<OutgoingRequest> {
        let headers = Fields::new();
        // Add headers to appear more legitimate
        headers
            .set("User-Agent", &[b"crypto-dma-filter/1.0".to_vec()])
            .map_err(|e| eyre!("Failed to set User-Agent: {:?}", e))?;
        headers
            .set("Accept", &[b"application/json".to_vec()])
            .map_err(|e| eyre!("Failed to set Accept: {:?}", e))?;

        let mut full_query = query.to_string();
        if let Some(key) = &self.api_key {
            if !full_query.is_empty() {
                full_query.push('&');
            }
            full_query.push_str(&format!("x_cg_demo_api_key={key}"));
        }

        let request = OutgoingRequest::new(headers);
        request
            .set_method(&Method::Get)
            .map_err(|_| eyre!("Failed to set method"))?;
        request
            .set_scheme(Some(&Scheme::Https))
            .map_err(|_| eyre!("Failed to set scheme"))?;
        request
            .set_authority(Some("api.coingecko.com"))
            .map_err(|_| eyre!("Failed to set authority"))?;
        request
            .set_path_with_query(Some(&format!("{path}?{full_query}")))
            .map_err(|_| eyre!("Failed to set path with query"))?;

        Ok(request)
    }

    fn fetch_json(&self, path: &str, query: &str) -> Result<Value> {
        let request = self.build_request(path, query)?;
        let options = RequestOptions::new();

        let future = handle(request, Some(options))
            .map_err(|e| eyre!("Failed to create request: {:?}", e))?;

        // wait?
        poll::poll(&[&future.subscribe()]);

        let response = future
            .get()
            .ok_or_else(|| eyre!("Response not ready"))?
            .map_err(|_| eyre!("Future failed"))?
            .map_err(|e| eyre!("Request failed: {:?}", e))?;

        let status = response.status();

        let body = response
            .consume()
            .map_err(|_| eyre!("Failed to consume response (status: {})", status))?;
        let stream = body
            .stream()
            .map_err(|_| eyre!("Failed to get input stream"))?;

        // read
        let mut data = Vec::new();
        loop {
            match stream.blocking_read(8192) {
                Ok(chunk) if !chunk.is_empty() => data.extend(chunk),
                Ok(_) => break,                    // empty chunk → EOF
                Err(StreamError::Closed) => break, // normal end-of-stream
                Err(e) => {
                    return Err(eyre!(
                        "Error reading response body (status: {}): {:?}",
                        status,
                        e
                    ))
                }
            }
        }

        let body_string = String::from_utf8(data)
            .map_err(|_| eyre!("Invalid UTF-8 response (status: {})", status))?;
        if status != 200 {
            return Err(eyre!(
                "Non-200 status: {} body-preview: {}",
                status,
                body_string.chars().take(200).collect::<String>()
            ));
        }
        let json: Value =
            serde_json::from_str(&body_string).map_err(|e| eyre!("JSON parse error: {}", e))?;

        Ok(json)
    }

    fn get_current_prices(&self, coin_ids: &[&str]) -> Result<HashMap<String, f64>> {
        let ids = coin_ids.join(",");
        let query = format!("ids={ids}&vs_currencies=usd");
        let json = self.fetch_json("/api/v3/simple/price", &query)?;

        let mut prices = HashMap::new();
        for coin_id in coin_ids {
            if let Some(price) = json[coin_id]["usd"].as_f64() {
                prices.insert(coin_id.to_string(), price);
            }
        }

        Ok(prices)
    }

    fn get_historical_prices(&self, coin_id: &str, days: u32) -> Result<Vec<f64>> {
        let query = format!("vs_currency=usd&days={days}");
        let path = format!("/api/v3/coins/{coin_id}/market_chart");
        let json = self.fetch_json(&path, &query)?;

        let prices: Vec<f64> = json["prices"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|p| p.as_array()?.get(1)?.as_f64())
            .collect();

        Ok(prices)
    }

    pub fn get_price_data(&self, coin_ids: &[&str]) -> Result<PriceData> {
        let mut price_data = PriceData::new();
        let address_map = coingecko_to_address();

        // get current prices
        let current_prices = self.get_current_prices(coin_ids)?;
        for (coin_id, price) in current_prices {
            if let Some(address) = address_map.get(coin_id.as_str()) {
                price_data.add_price(address.clone(), price);
            }
        }

        // get historical data and calculate 200dma
        for &coin_id in coin_ids {
            if let Ok(historical_prices) = self.get_historical_prices(coin_id, 200) {
                if let Some(dma_200) = calculate_200dma(&historical_prices) {
                    if let Some(address) = address_map.get(coin_id) {
                        price_data.add_indicator("dma_200".to_string(), address.clone(), dma_200);
                    }
                }
            }
        }

        Ok(price_data)
    }
}
