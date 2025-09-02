mod bindings;
mod api;
mod models;
mod config;
mod output;

use api::{ApiClient, calculate_200dma};
use models::FilteredCoin;
use config::Config;
use output::print_results;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Check for help flag
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return;
    }
    
    let config = Config::new();
    
    if config.verbose {
        println!("Starting crypto DMA trading strategy...");
        println!("Configuration: limit={}, api={:?}, output={:?}, verbose={}", 
                config.limit, config.api_server, config.output_format, config.verbose);
    }
    
    let client = ApiClient::new();
    
    match client.get_coins(&config) {
        Ok(coins) => {
            if config.verbose {
                println!("Fetched {} coins", coins.len());
            }
            
            let mut filtered_coins = Vec::new();
            
            for coin in &coins {
                if config.verbose {
                    println!("Analyzing {}", coin.symbol.to_uppercase());
                }
                
                // Get 200 days of historical data
                match client.get_historical_data(&coin.id, 200, &config) {
                    Ok(prices) => {
                        if let Some(dma_200) = calculate_200dma(&prices) {
                            let filtered = FilteredCoin::new(coin, dma_200);
                            
                            if config.verbose {
                                println!("  Current price: ${:.4}", filtered.current_price);
                                println!("  200-day MA: ${:.4}", filtered.dma_200);
                                println!("  Price vs DMA: {:.2}%", filtered.price_vs_dma_pct);
                            }
                            
                            // Filter: price above 200-day moving average
                            if filtered.price_vs_dma_pct > 0.0 {
                                filtered_coins.push(filtered);
                                if config.verbose {
                                    println!("  ✓ Above 200-day MA - Added to watchlist");
                                }
                            } else if config.verbose {
                                println!("  ✗ Below 200-day MA - Filtered out");
                            }
                        } else if config.verbose {
                            println!("  ⚠ Insufficient data for 200-day MA");
                        }
                    }
                    Err(e) => {
                        if config.verbose {
                            println!("  ⚠ Failed to get historical data: {}", e);
                        }
                    }
                }
                
                if config.verbose {
                    println!();
                }
            }
            
            // Sort by price vs DMA percentage (strongest momentum first)
            filtered_coins.sort_by(|a, b| b.price_vs_dma_pct.partial_cmp(&a.price_vs_dma_pct).unwrap());
            
            print_results(&filtered_coins, &config);
        }
        Err(e) => {
            eprintln!("Error fetching coins: {}", e);
        }
    }
}

fn print_help() {
    println!("Crypto DMA Trading Strategy - WebAssembly Edition");
    println!();
    println!("Usage: main.wasm [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("  --limit <NUMBER>        Number of coins to analyze (default: 10)");
    println!("  --verbose               Enable verbose output");
    println!("  --format <FORMAT>       Output format: json, json_pretty, table (default: table)");
    println!("  --string <JSON>         JSON string with configuration (e.g., '{{\"coins\": [\"bitcoin\", \"ethereum\"]}}')");
    println!("  --coins <LIST>          Comma-separated list of specific coins to analyze");
    println!("  --api-server <SERVER>   API server: coingecko, coinmarketcap (default: coingecko)");
    println!("  --help, -h              Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  main.wasm --limit 5 --verbose");
    println!("  main.wasm --format json --coins bitcoin,ethereum");
    println!("  main.wasm --string '{{\"coins\": [\"bitcoin\", \"ethereum\", \"cardano\"]}}'");
    println!("  main.wasm --format json_pretty --limit 3");
    println!("  main.wasm --api-server coinmarketcap --limit 3");
}
