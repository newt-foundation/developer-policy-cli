use crate::price::TradingSignal;
use crate::tokens::{Address, USDC_ADDRESS, WETH_ADDRESS};

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
