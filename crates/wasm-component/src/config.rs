#![allow(unused)]

use std::env;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Config {
    pub limit: u32,
    pub specific_coins: Vec<String>,
    pub api_server: ApiServer,
    pub output_format: OutputFormat,
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub enum ApiServer {
    CoinGecko,
    CoinMarketCap,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    JsonPretty,
    Table,
}

#[derive(Debug, Deserialize)]
struct InputConfig {
    coins: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            limit: 10,
            specific_coins: vec![],
            api_server: ApiServer::CoinGecko,
            output_format: OutputFormat::Table,
            verbose: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config::default();
        
        // Parse command line arguments
        let args: Vec<String> = env::args().collect();
        
        // Parse --limit
        if let Some(limit_idx) = args.iter().position(|arg| arg == "--limit") {
            if let Some(limit_str) = args.get(limit_idx + 1) {
                if let Ok(limit) = limit_str.parse::<u32>() {
                    config.limit = limit;
                }
            }
        }
        
        // Parse --verbose
        if args.iter().any(|arg| arg == "--verbose") {
            config.verbose = true;
        }
        
        // Parse --format
        if let Some(format_idx) = args.iter().position(|arg| arg == "--format") {
            if let Some(format_str) = args.get(format_idx + 1) {
                config.output_format = match format_str.as_str() {
                    "json" => OutputFormat::Json,
                    "json_pretty" => OutputFormat::JsonPretty,
                    "table" => OutputFormat::Table,
                    _ => OutputFormat::Table,
                };
            }
        }
        
        // Parse --string (JSON format)
        if let Some(string_idx) = args.iter().position(|arg| arg == "--string") {
            if let Some(json_str) = args.get(string_idx + 1) {
                if let Ok(input_config) = serde_json::from_str::<InputConfig>(json_str) {
                    if let Some(coins) = input_config.coins {
                        config.specific_coins = coins;
                    }
                }
            }
        }
        
        // Parse --coins (comma-separated) - kept for backward compatibility
        if let Some(coins_idx) = args.iter().position(|arg| arg == "--coins") {
            if let Some(coins_str) = args.get(coins_idx + 1) {
                config.specific_coins = coins_str.split(',').map(|s| s.to_string()).collect();
            }
        }
        
        // Parse --api-server
        if let Some(api_idx) = args.iter().position(|arg| arg == "--api-server") {
            if let Some(api_str) = args.get(api_idx + 1) {
                config.api_server = match api_str.as_str() {
                    "coingecko" => ApiServer::CoinGecko,
                    "coinmarketcap" => ApiServer::CoinMarketCap,
                    _ => ApiServer::CoinGecko,
                };
            }
        }
        
        config
    }
    
    pub fn use_specific_coins(&self) -> bool {
        !self.specific_coins.is_empty()
    }
}