use eyre::Result;
use reqwest::Client;
use serde_json::Value;
use shared::price::PriceData;
use shared::strategy::calculate_200dma;
use shared::tokens::coingecko_to_address;
use std::collections::HashMap;

pub trait PriceFetcher {
    async fn get_price_data(&self, coin_ids: &[&str]) -> Result<PriceData>;
}

pub struct NativePriceFetcher {
    client: Client,
    api_key: Option<String>,
}

impl NativePriceFetcher {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    async fn get_current_prices(&self, coin_ids: &[&str]) -> Result<HashMap<String, f64>> {
        let ids = coin_ids.join(",");
        let mut url =
            format!("https://api.coingecko.com/api/v3/simple/price?ids={ids}&vs_currencies=usd");

        if let Some(key) = &self.api_key {
            url.push_str(&format!("&x_cg_demo_api_key={key}"));
        }

        let response: Value = self.client.get(&url).send().await?.json().await?;

        let mut prices = HashMap::new();
        for coin_id in coin_ids {
            if let Some(price) = response[coin_id]["usd"].as_f64() {
                prices.insert(coin_id.to_string(), price);
            }
        }

        Ok(prices)
    }

    async fn get_historical_prices(&self, coin_id: &str, days: u32) -> Result<Vec<f64>> {
        let mut url = format!(
            "https://api.coingecko.com/api/v3/coins/{coin_id}/market_chart?vs_currency=usd&days={days}"
        );

        if let Some(key) = &self.api_key {
            url.push_str(&format!("&x_cg_demo_api_key={key}"));
        }

        let response: Value = self.client.get(&url).send().await?.json().await?;

        let prices: Vec<f64> = response["prices"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|p| p.as_array()?.get(1)?.as_f64())
            .collect();

        Ok(prices)
    }
}

impl PriceFetcher for NativePriceFetcher {
    async fn get_price_data(&self, coin_ids: &[&str]) -> Result<PriceData> {
        let mut price_data = PriceData::new();
        let address_map = coingecko_to_address();

        // get current prices
        let current_prices = self.get_current_prices(coin_ids).await?;
        for (coin_id, price) in current_prices {
            if let Some(address) = address_map.get(coin_id.as_str()) {
                price_data.add_price(address.clone(), price);
            }
        }

        // get historical data and calculate 200dma
        for &coin_id in coin_ids {
            if let Ok(historical_prices) = self.get_historical_prices(coin_id, 200).await {
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
