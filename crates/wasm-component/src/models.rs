use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub current_price: f64,
    pub market_cap_rank: u32,
}

#[derive(Debug, Deserialize)]
pub struct HistoricalData {
    pub prices: Vec<Vec<f64>>, // [timestamp, price]
}

#[derive(Debug, Serialize)]
pub struct FilteredCoin {
    pub rank: u32,
    pub symbol: String,
    pub current_price: f64,
    pub dma_200: f64,
    pub price_vs_dma_pct: f64,
}

impl FilteredCoin {
    pub fn new(coin: &Coin, dma_200: f64) -> Self {
        let price_vs_dma_pct = ((coin.current_price - dma_200) / dma_200) * 100.0;
        
        Self {
            rank: coin.market_cap_rank,
            symbol: coin.symbol.clone(),
            current_price: coin.current_price,
            dma_200,
            price_vs_dma_pct,
        }
    }
}