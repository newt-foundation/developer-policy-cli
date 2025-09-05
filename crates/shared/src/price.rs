use crate::tokens::{address_to_coingecko, Address};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TradingSignal {
    pub prices_usd: HashMap<Address, f64>,
    pub indicators: HashMap<String, HashMap<Address, f64>>,
}

impl TradingSignal {
    pub fn new() -> Self {
        Self {
            prices_usd: HashMap::new(),
            indicators: HashMap::new(),
        }
    }

    pub fn add_price(&mut self, address: Address, price_usd: f64) {
        self.prices_usd.insert(address, price_usd);
    }

    pub fn add_indicator(&mut self, name: String, address: Address, value: f64) {
        self.indicators
            .entry(name)
            .or_default()
            .insert(address, value);
    }

    /// Display the TradingSignal in pretty JSON format
    pub fn display_pretty(&self) -> String {
        format!("{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_signal_display() {
        let mut signal = TradingSignal::new();

        // Add some sample prices
        signal.add_price(
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
            1.0,
        ); // USDC
        signal.add_price(
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(),
            2000.0,
        ); // WETH

        // Add some sample indicators
        signal.add_indicator(
            "market_cap_rank".to_string(),
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
            3.0,
        );
        signal.add_indicator(
            "market_cap_rank".to_string(),
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(),
            2.0,
        );
        signal.add_indicator(
            "dma_200".to_string(),
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(),
            1950.0,
        );

        println!("TradingSignal Display:\n{}", signal);
    }
}

impl Display for TradingSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Create a custom JSON structure for pretty display
        let mut json_obj = serde_json::Map::new();

        let address_to_coingecko = address_to_coingecko();

        // Format prices_usd: key = "token", value = "price"
        let mut prices_obj = serde_json::Map::new();
        for (token, price) in &self.prices_usd {
            let token_str = address_to_coingecko.get(token.as_str()).unwrap();
            prices_obj.insert(
                format!("{} ({})", token_str, token),
                serde_json::Value::Number(
                    serde_json::Number::from_f64(*price).unwrap_or(serde_json::Number::from(0)),
                ),
            );
        }
        json_obj.insert(
            "prices_usd".to_string(),
            serde_json::Value::Object(prices_obj),
        );

        // Format indicators: key = "signal", value = { key = "token", value = actual_value }
        let mut indicators_obj = serde_json::Map::new();
        for (signal, token_values) in &self.indicators {
            let mut signal_obj = serde_json::Map::new();
            for (token, value) in token_values {
                let token_str = address_to_coingecko.get(token.as_str()).unwrap();
                signal_obj.insert(
                    format!("{} ({})", token_str, token),
                    serde_json::Value::Number(
                        serde_json::Number::from_f64(*value).unwrap_or(serde_json::Number::from(0)),
                    ),
                );
            }
            indicators_obj.insert(signal.clone(), serde_json::Value::Object(signal_obj));
        }
        json_obj.insert(
            "indicators".to_string(),
            serde_json::Value::Object(indicators_obj),
        );

        // Convert to pretty JSON string
        let json_value = serde_json::Value::Object(json_obj);
        let pretty_json = serde_json::to_string_pretty(&json_value)
            .unwrap_or_else(|_| "Error formatting JSON".to_string());

        write!(f, "{}", pretty_json)
    }
}
