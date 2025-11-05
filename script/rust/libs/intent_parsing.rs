use alloy_primitives::{U256};
use super::utils::hex_to_u256;

// Normalize intent value - handles bigint, number, or hex string
fn normalize_value(value: &serde_json::Value) -> Result<U256, Box<dyn std::error::Error>> {
    match value {
        serde_json::Value::String(s) => {
            // Try parsing as hex string
            hex_to_u256(s)
        }
        serde_json::Value::Number(n) => {
            // Parse as u64 first, then convert to U256
            let num = n.as_u64()
                .ok_or_else(|| "Number too large for u64".to_string())?;
            Ok(U256::from(num))
        }
        _ => Err("Invalid value type: expected string or number".into()),
    }
}

// Normalize intent chainId - handles bigint, number, or hex string
fn normalize_chain_id(chain_id: &serde_json::Value) -> Result<U256, Box<dyn std::error::Error>> {
    match chain_id {
        serde_json::Value::String(s) => {
            // Try parsing as hex string
            hex_to_u256(s)
        }
        serde_json::Value::Number(n) => {
            // Parse as u64 first, then convert to U256
            let num = n.as_u64()
                .ok_or_else(|| "Number too large for u64".to_string())?;
            Ok(U256::from(num))
        }
        _ => Err("Invalid chainId type: expected string or number".into()),
    }
}

// Main normalize function - takes a JSON intent and returns hex versions of chain id and value
pub fn normalize_intent(intent: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut normalized = intent.clone();
    
    // Normalize value
    if let Some(value) = normalized.get("value") {
        let normalized_value = normalize_value(value)?;
        normalized["value"] = serde_json::Value::String(format!("0x{:x}", normalized_value));
    }
    
    // Normalize chainId
    if let Some(chain_id) = normalized.get("chainId") {
        let normalized_chain_id = normalize_chain_id(chain_id)?;
        normalized["chainId"] = serde_json::Value::String(format!("0x{:x}", normalized_chain_id));
    }
    
    Ok(normalized)
}