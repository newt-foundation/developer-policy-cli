use serde_json::Value;
// use crate::rego::{evaluate, validate_schema, error::RegoError};
// use crate::entity::intent::parse_intent;
use std::fs;

pub fn evaluate_local_policy(
    policy_path: &str,
    params_path: &str,
    intent_path: &str,
    _schema_path: Option<&str>,
    entrypoint: &str, // e.g. "example.allow"
) -> Result<Value, String> {
    // Read files
    let policy = fs::read_to_string(policy_path).map_err(|_| "Missing policy file".to_string())?;
    let params: Value = serde_json::from_str(&fs::read_to_string(params_path).unwrap()).unwrap();
    let intent: Value = serde_json::from_str(&fs::read_to_string(intent_path).unwrap()).unwrap();

    // Stub: just print what would be used
    println!("Policy file: {}", policy_path);
    println!("Params file: {}", params_path);
    println!("Intent file: {}", intent_path);
    println!("Entrypoint: {}", entrypoint);
    println!("Policy contents: {}", policy);
    println!("Params: {:?}", params);
    println!("Intent: {:?}", intent);

    // Return a dummy value for now
    Ok(serde_json::json!({
        "status": "stubbed",
        "entrypoint": entrypoint
    }))
}