use std::env;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
mod libs {
    pub mod utils;
}
use libs::utils::{get_evaluation_request_hash, normalize_intent, sanitize_intent_for_request};
use libs::utils::{remove_hex_prefix, sign_hash, http_post, get_prover_avs_url};
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
