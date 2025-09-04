use crate::tokens::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}
