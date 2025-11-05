use std::env;
use std::fs;

mod libs {
    pub mod utils;
    pub mod intent_parsing;
    pub mod signature;
}
use libs::intent_parsing::{normalize_intent, sanitize_intent_for_request, remove_hex_prefix};
use libs::utils::{http_post, get_prover_avs_url, create_json_rpc_request_payload};
use libs::signature::{sign_hash, get_evaluation_request_hash};

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

    // Normalize intent.value, intent.chainId, quorumNumber, wasmArgs to hex strings.
    let normalized_intent = normalize_intent(intent)?;
    let normalized_task = serde_json::json!({
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

    // Sign the normalized task.
    let evaluation_request_hash = get_evaluation_request_hash(&normalized_task)?;
    let private_key = std::env::var("PRIVATE_KEY").map_err(|_| "PRIVATE_KEY not found in .env file")?;
    let signature = sign_hash(evaluation_request_hash, &private_key)?;

    // Snake case the keys of the task + intent, remove the prefix 0x from certain fields.
    let sanitized_intent = sanitize_intent_for_request(intent)?;
    let sanitized_task = serde_json::json!({
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
        "signature": signature,
    });
    
    let payload = create_json_rpc_request_payload("newton_createTaskAndWait", sanitized_task);

    let chain_id = std::env::var("CHAIN_ID").map_err(|_| "CHAIN_ID env var not found")?;
    let deployment_env = std::env::var("DEPLOYMENT_ENV").map_err(|_| "DEPLOYMENT_ENV env var not found")?;
    let prover_avs_url = get_prover_avs_url(&chain_id, &deployment_env)?;

    let response = http_post(&prover_avs_url, &payload).await?;
    println!("Response: {}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
