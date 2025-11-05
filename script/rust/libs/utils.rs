use alloy_primitives::{U256};
use std::sync::atomic::{AtomicU64, Ordering};

// Static counter for JSON-RPC request IDs
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn get_next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed) + 1
}

pub fn create_json_rpc_request_payload(
    method: &str,
    params: serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": get_next_id(),
        "method": method,
        "params": params,
    })
}

// Helper function to convert hex string to U256
pub fn hex_to_u256(hex_str: &str) -> Result<U256, Box<dyn std::error::Error>> {
    // Remove "0x" prefix if present
    let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    
    // U256 can parse hex strings directly
    U256::from_str_radix(hex_str, 16)
        .map_err(|e| format!("Failed to parse hex string '{}': {}", hex_str, e).into())
}

pub fn get_prover_avs_url(chain_id: &str, deployment_env: &str) -> Result<String, Box<dyn std::error::Error>> {
    let chain_id_num: u64 = chain_id.parse()
        .map_err(|_| format!("Invalid CHAIN_ID: {}", chain_id))?;
    
    match (deployment_env, chain_id_num) {
        ("stagef", 11155111) => Ok("https://prover-avs.stagef.sepolia.newt.foundation".to_string()),
        ("stagef", 1) => Ok("https://prover-avs.stagef.newt.foundation".to_string()),
        ("prod", 11155111) => Ok("https://prover-avs.sepolia.newt.foundation".to_string()),
        ("prod", 1) => Ok("https://prover-avs.newt.foundation".to_string()),
        _ => Err(format!("Unsupported combination: DEPLOYMENT_ENV={}, CHAIN_ID={}", deployment_env, chain_id).into()),
    }
}

pub async fn http_post(url: &str, body: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(body)
        .send()
        .await?;
    
    let status = response.status();
    
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!("HTTP error {}: {}", status, error_text).into());
    }
    
    let response_json: serde_json::Value = response.json().await?;
    Ok(response_json)
}