use serde::{Deserialize, Serialize};

use crate::price::TradingSignal;
use crate::tokens::{address_to_coingecko, Address, USDC_ADDRESS, WETH_ADDRESS};

pub fn calculate_200dma(prices: &[f64]) -> Option<f64> {
    if prices.len() < 200 {
        return None;
    }

    let sum: f64 = prices.iter().rev().take(200).sum();
    Some(sum / 200.0)
}

pub fn should_trade(price_data: &TradingSignal) -> Option<(Address, Address)> {
    let dma_indicators = price_data.indicators.get("dma_200")?;

    let weth_price = price_data.prices_usd.get(WETH_ADDRESS)?;
    let weth_dma = dma_indicators.get(WETH_ADDRESS)?;

    // just a weth strategy
    if *weth_price > *weth_dma {
        Some((USDC_ADDRESS.into(), WETH_ADDRESS.into()))
    } else {
        None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfitTrade {
    pub rank: f64,
    pub address: Address,
    pub symbol: String,
    pub current_price: f64,
    pub dma_200: f64,
    pub price_vs_dma_pct: f64,
}

pub fn filter_profitable_trades(trading_signal: &TradingSignal) -> Vec<ProfitTrade> {
    let mut profitable_trades = Vec::new();

    let address_to_coingecko = address_to_coingecko();
    for (address, price) in trading_signal.prices_usd.clone() {
        if !address_to_coingecko.contains_key(address.as_str()) {
            continue;
        }

        let symbol = *address_to_coingecko.get(address.as_str()).unwrap();

        let market_cap_rank = trading_signal.indicators.get("market_cap_rank").unwrap();
        let dma_200 = trading_signal.indicators.get("dma_200").unwrap();
        if !market_cap_rank.contains_key(address.as_str()) || !dma_200.contains_key(address.as_str()) {
            continue;
        }

        let rank = market_cap_rank.get(address.as_str()).unwrap();
        let dma_200 = dma_200.get(address.as_str()).unwrap();
        let price_vs_dma_pct = ((price - dma_200) / dma_200) * 100.0;

        if price_vs_dma_pct > 20.0 {
            profitable_trades.push(ProfitTrade {
                rank: *rank,
                address,
                symbol: symbol.to_string(),
                current_price: price,
                dma_200: *dma_200,
                price_vs_dma_pct,
            });
        }        
    }

    profitable_trades
}