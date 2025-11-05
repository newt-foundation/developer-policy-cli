use std::env;
use std::fs;
use alloy_primitives::{Address, Bytes, U256, keccak256};
use k256::ecdsa::{SigningKey, signature::hazmat::PrehashSigner, VerifyingKey};
use k256::SecretKey;
use serde_json::Value;
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

fn sign_hash(hash: [u8; 32], private_key: &str) -> Result<(u8, [u8; 32], [u8; 32]), Box<dyn std::error::Error>> {
    // Parse private key (remove 0x prefix if present)
    let private_key = private_key.strip_prefix("0x").unwrap_or(private_key);
    
    // Decode hex string to bytes
    let private_key_bytes = hex::decode(private_key)
        .map_err(|e| format!("Failed to decode private key: {}", e))?;
    
    // Ensure it's exactly 32 bytes
    if private_key_bytes.len() != 32 {
        return Err("Private key must be 32 bytes".into());
    }
    
    // Convert to fixed-size array
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&private_key_bytes);
    
    // Create SecretKey from fixed-size array
    let secret_key = SecretKey::from_bytes(&key_array.into())
        .map_err(|e| format!("Invalid private key: {}", e))?;
    
    // Create SigningKey from SecretKey
    let signing_key = SigningKey::from(&secret_key);
    
    // Sign the hash
    let signature: k256::ecdsa::Signature = signing_key.sign_prehash(&hash)?;
    
    // Extract r and s
    let (r, s) = signature.split_bytes();
    let r_bytes: [u8; 32] = r.into();
    let s_bytes: [u8; 32] = s.into();
    
    // Get verifying key for recovery
    let verifying_key = signing_key.verifying_key();
    
    // Try to recover with v = 27 (recovery_id = 0)
    let recovery_id_0 = k256::ecdsa::RecoveryId::try_from(0u8)?;
    let v = if VerifyingKey::recover_from_prehash(&hash, &signature, recovery_id_0)
        .map(|key| key == *verifying_key)
        .unwrap_or(false) {
        27u8
    } else {
        28u8  // Try recovery_id = 1
    };
    
    Ok((v, r_bytes, s_bytes))
}

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
fn remove_hex_prefix(hex_str: &str) -> String {
    hex_str.strip_prefix("0x").unwrap_or(hex_str).to_string()
}

// Helper to get string value and remove hex prefix
fn get_string_without_prefix(value: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let s = value.as_str()
        .ok_or_else(|| "Expected string")?;
    Ok(remove_hex_prefix(s))
}

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

fn get_prover_avs_url(chain_id: &str, deployment_env: &str) -> Result<String, Box<dyn std::error::Error>> {
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

async fn http_post(url: &str, body: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <task_json_file_path>", args[0]);
        std::process::exit(1);
    }

    let task_json_file_path = &args[1];
    let contents = fs::read_to_string(task_json_file_path)?;
    let task: serde_json::Value = serde_json::from_str(&contents)?;    
    let intent = task.get("intent").ok_or_else(|| "Missing 'intent' field in task")?;

    let normalized_intent = normalize_intent(intent)?;

    let task_with_normalized_intent = serde_json::json!({
        "policyClient": task.get("policyClient"),
        "intent": normalized_intent,
        "quorumNumber": task.get("quorumNumber").cloned()
            .unwrap_or_else(|| serde_json::Value::String("0x".to_string())),
        "quorumThresholdPercentage": task.get("quorumThresholdPercentage").cloned()
            .unwrap_or_else(|| serde_json::Value::Number(0.into())),
        "wasmArgs": task.get("wasmArgs").cloned()
            .unwrap_or_else(|| serde_json::Value::String("0x".to_string())),
        "timeout": task.get("timeout"),
    });

    let evaluation_request_hash = get_evaluation_request_hash(&task_with_normalized_intent)?;
    let private_key = std::env::var("PRIVATE_KEY").map_err(|_| "PRIVATE_KEY not found in .env file")?;
    let (v, r, s) = sign_hash(evaluation_request_hash, &private_key)?;
    
    let mut sig_bytes = Vec::new();
    sig_bytes.extend_from_slice(&r);
    sig_bytes.extend_from_slice(&s);
    sig_bytes.push(v);
    let signature_hex = hex::encode(sig_bytes);

    let sanitized_intent = sanitize_intent_for_request(intent)?;
    
    let request_body = serde_json::json!({
        "policy_client": task.get("policyClient"),
        "intent": sanitized_intent,
        "quorum_number": task.get("quorumNumber")
            .and_then(|v| v.as_str())
            .map(|s| remove_hex_prefix(s))
            .map(|s| serde_json::Value::String(s))
            .unwrap_or(serde_json::Value::Null),
        "quorum_threshold_percentage": task.get("quorumThresholdPercentage")
            .cloned()
            .unwrap_or(serde_json::Value::Null),
        "wasm_args": task.get("wasmArgs")
            .and_then(|v| v.as_str())
            .map(|s| remove_hex_prefix(s))
            .map(|s| serde_json::Value::String(s))
            .unwrap_or(serde_json::Value::Null),
        "timeout": task.get("timeout"),
        "signature": format!("0x{}", signature_hex),
    });
    
    let payload = create_json_rpc_request_payload(
        "newton_createTaskAndWait",
        request_body
    );

    let chain_id = std::env::var("CHAIN_ID")
        .map_err(|_| "CHAIN_ID not found. Set CHAIN_ID environment variable")?;
    let deployment_env = std::env::var("DEPLOYMENT_ENV")
        .map_err(|_| "DEPLOYMENT_ENV not found. Set DEPLOYMENT_ENV environment variable")?;
        
    let prover_avs_url = get_prover_avs_url(&chain_id, &deployment_env)?;

    let response = http_post(&prover_avs_url, &payload).await?;
    println!("Response: {}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
