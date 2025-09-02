use crate::models::FilteredCoin;
use crate::config::{Config, OutputFormat};
use serde_json;

pub fn print_results(filtered_coins: &[FilteredCoin], config: &Config) {
    match config.output_format {
        OutputFormat::Json => print_json(filtered_coins),
        OutputFormat::JsonPretty => print_json_pretty(filtered_coins),
        OutputFormat::Table => print_table(filtered_coins),
    }
}

fn print_json(filtered_coins: &[FilteredCoin]) {
    match serde_json::to_string(filtered_coins) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

fn print_json_pretty(filtered_coins: &[FilteredCoin]) {
    match serde_json::to_string_pretty(filtered_coins) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

fn print_table(filtered_coins: &[FilteredCoin]) {
    if filtered_coins.is_empty() {
        println!("No coins pass the filter criteria.");
        return;
    }

    println!("\n=== FILTERED COINS (Above 200-day MA) ===");
    println!("{:<10} {:<20} {:<12} {:<12} {:<10}", "RANK", "SYMBOL", "PRICE", "200-DMA", "% ABOVE");
    println!("{}", "-".repeat(70));
    
    for coin in filtered_coins {
        println!("{:<10} {:<20} ${:<11.4} ${:<11.4} {:<10.2}%", 
            coin.rank,
            format!("{}", coin.symbol.to_uppercase()),
            coin.current_price,
            coin.dma_200,
            coin.price_vs_dma_pct
        );
    }
    
    println!("\nTrading strategy complete. {} coins pass the filter.", filtered_coins.len());
}