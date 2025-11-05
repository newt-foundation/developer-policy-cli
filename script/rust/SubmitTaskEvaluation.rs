use std::env;
use std::fs;

use alloy_primitives::{Address, Bytes, U256, keccak256};
// use alloy_encode_packed::encode_packed;
use serde_json::Value;

fn encode_packed(values: &[&[u8]]) -> Vec<u8> {
    let mut result = Vec::new();
    for value in values {
        result.extend_from_slice(value);
    }
    result
}

// Helper to convert hex string to Bytes
fn hex_to_bytes(hex_str: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
    let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    let bytes = hex::decode(hex_str)
        .map_err(|e| format!("Failed to decode hex: {}", e))?;
    Ok(Bytes::copy_from_slice(&bytes))
}

// Helper functions to get values from JSON (same as before)
fn get_address(value: &Value) -> Result<Address, Box<dyn std::error::Error>> {
    let s = value.as_str()
        .ok_or_else(|| "Expected string for address")?;
    s.parse::<Address>()
        .map_err(|e| format!("Invalid address: {}", e).into())
}

fn get_bytes(value: &Value) -> Result<Bytes, Box<dyn std::error::Error>> {
    if value.is_null() {
        return Ok(Bytes::new());
    }
    if let Some(s) = value.as_str() {
        hex_to_bytes(s)
    } else {
        Err("Expected string for bytes".into())
    }
}

fn get_u256(value: &Value) -> Result<U256, Box<dyn std::error::Error>> {
    if let Some(s) = value.as_str() {
        hex_to_u256(s)
    } else if let Some(n) = value.as_u64() {
        Ok(U256::from(n))
    } else {
        Err("Expected string or number for U256".into())
    }
}

pub fn get_evaluation_request_hash(args: &serde_json::Value) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    // Extract and normalize intent
    let intent = args.get("intent")
        .ok_or_else(|| "Missing intent")?;
    let normalized_intent = normalize_intent(intent)?;
    
    // Extract all values
    let policy_client = get_address(args.get("policyClient").ok_or_else(|| "Missing policyClient")?)?;
    let from = get_address(normalized_intent.get("from").ok_or_else(|| "Missing intent.from")?)?;
    let to = get_address(normalized_intent.get("to").ok_or_else(|| "Missing intent.to")?)?;
    let value = get_u256(normalized_intent.get("value").ok_or_else(|| "Missing intent.value")?)?;
    let data = get_bytes(normalized_intent.get("data").ok_or_else(|| "Missing intent.data")?)?;
    let chain_id = get_u256(normalized_intent.get("chainId").ok_or_else(|| "Missing intent.chainId")?)?;
    let function_signature = normalized_intent.get("functionSignature")
        .map(|v| get_bytes(v))
        .transpose()?
        .unwrap_or_else(|| Bytes::new());
    
    let quorum_number = args.get("quorumNumber")
        .map(|v| get_bytes(v))
        .transpose()?
        .unwrap_or_else(|| Bytes::new());
    let quorum_threshold = args.get("quorumThresholdPercentage")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    let wasm_args = args.get("wasmArgs")
        .map(|v| get_bytes(v))
        .transpose()?
        .unwrap_or_else(|| Bytes::new());
    let timeout = args.get("timeout")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "Invalid timeout")?;
    
    // Encode packed manually
    let packed = encode_packed(&[
        policy_client.as_ref(),      // address (20 bytes) - Address implements AsRef<[u8]>
        from.as_ref(),               // address (20 bytes)
        to.as_ref(),                 // address (20 bytes)
        &value.to_be_bytes::<32>(),    // uint256 (32 bytes)
        data.as_ref(),               // bytes (variable) - Bytes implements AsRef<[u8]>
        &chain_id.to_be_bytes::<32>(), // uint256 (32 bytes)
        function_signature.as_ref(), // bytes (variable)
        quorum_number.as_ref(),      // bytes (variable, or empty)
        &quorum_threshold.to_be_bytes(), // uint32 (4 bytes)
        wasm_args.as_ref(),          // bytes (variable, or empty)
        &timeout.to_be_bytes(),        // uint64 (8 bytes)
    ]);
    
    // Compute keccak256 hash
    let hash = keccak256(packed);
    
    Ok(hash.0)
}

// Helper function to convert hex string to U256
fn hex_to_u256(hex_str: &str) -> Result<U256, Box<dyn std::error::Error>> {
    // Remove "0x" prefix if present
    let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    
    // U256 can parse hex strings directly
    U256::from_str_radix(hex_str, 16)
        .map_err(|e| format!("Failed to parse hex string '{}': {}", hex_str, e).into())
}

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

// Main normalize function - takes a JSON intent and returns normalized version
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*let response = reqwest::get("https://httpbin.org/get").await?;
    let body = response.text().await?;
    println!("Response: {}", body);*/

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <task_json_file_path>", args[0]);
        std::process::exit(1);
    }

    let task_json_file_path = &args[1];

    let contents = fs::read_to_string(task_json_file_path)?;
    let task: serde_json::Value = serde_json::from_str(&contents)?;    
    println!("{}", serde_json::to_string_pretty(&task)?);

    let intent = task.get("intent").ok_or_else(|| "Missing 'intent' field in task")?;
    let normalized_intent = normalize_intent(intent)?;
    println!("Normalized intent: {}", serde_json::to_string_pretty(&normalized_intent)?);

    let task_with_normalized_intent = serde_json::json!({
        "policyClient": task.get("policyClient"),
        "intent": normalized_intent,
        "quorumNumber": task.get("quorumNumber"),
        "quorumThresholdPercentage": task.get("quorumThresholdPercentage"),
        "wasmArgs": task.get("wasmArgs"),
        "timeout": task.get("timeout"),
    });

    let evaluation_request_hash = get_evaluation_request_hash(&task_with_normalized_intent)?;
    println!("Evaluation request hash: {}", hex::encode(evaluation_request_hash));

    Ok(())
}
