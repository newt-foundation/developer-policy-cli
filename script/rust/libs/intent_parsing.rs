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

// Helper to convert value/chainId to hex string (similar to normalize but simpler)
fn to_hex_string(value: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    match value {
        serde_json::Value::String(s) => {
            // Already a hex string, return as-is
            Ok(s.clone())
        }
        serde_json::Value::Number(n) => {
            // Convert number to hex
            let num = n.as_u64()
                .ok_or_else(|| "Number too large for u64")?;
            Ok(format!("0x{:x}", num))
        }
        _ => Err("Invalid value type: expected string or number".into()),
    }
}

// Helper to remove 0x prefix from a hex string
pub fn remove_hex_prefix(hex_str: &str) -> String {
    hex_str.strip_prefix("0x").unwrap_or(hex_str).to_string()
}

// Helper to get string value and remove hex prefix
fn get_string_without_prefix(value: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let s = value.as_str()
        .ok_or_else(|| "Expected string")?;
    Ok(remove_hex_prefix(s))
}

// Snakecase the keys of the intent, remove the prefix 0x from certain fields.
pub fn sanitize_intent_for_request(intent: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Get value as hex
    let value = intent.get("value")
        .ok_or_else(|| "Missing value")?;
    let value_as_hex = to_hex_string(value)?;
    
    // Get chainId as hex
    let chain_id = intent.get("chainId")
        .ok_or_else(|| "Missing chainId")?;
    let chain_id_as_hex = to_hex_string(chain_id)?;
    
    // Get other fields
    let from = intent.get("from")
        .ok_or_else(|| "Missing from")?
        .as_str()
        .ok_or_else(|| "from must be a string")?;
    let to = intent.get("to")
        .ok_or_else(|| "Missing to")?
        .as_str()
        .ok_or_else(|| "to must be a string")?;
    let data = intent.get("data")
        .ok_or_else(|| "Missing data")?;
    let data_without_prefix = get_string_without_prefix(data)?;
    
    let function_signature = intent.get("functionSignature")
        .map(|v| get_string_without_prefix(v))
        .transpose()?
        .unwrap_or_else(|| String::new());
    
    // Build the sanitized intent with snake_case field names
    Ok(serde_json::json!({
        "from": from,
        "to": to,
        "value": value_as_hex,
        "data": data_without_prefix,
        "chain_id": chain_id_as_hex,
        "function_signature": function_signature,
    }))
}