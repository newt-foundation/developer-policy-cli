use alloy_dyn_abi::JsonAbiExt;
use alloy_dyn_abi::DynSolValue;
use alloy_json_abi::Function;
use alloy_primitives::{Address, Bytes, ChainId, U256};
use eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedIntent {
	pub from: Address,
	pub to: Address,
	pub value: U256,
	pub data: Option<Bytes>,
	pub chain_id: Option<u64>,
	pub function_signature: Option<Bytes>,
	pub function: Option<serde_json::Value>,
	pub decoded_function_signature: Option<String>,
	pub decoded_function_arguments: Option<Vec<serde_json::Value>>,
}

pub fn parse_intent(value: serde_json::Value) -> anyhow::Result<ParsedIntent> {
    let from = value
        .get("from")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing or invalid from field"))?
        .parse::<Address>()
        .map_err(|e| anyhow::anyhow!("invalid from address: {}", e))?;

    let to = value
        .get("to")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing or invalid to field"))?
        .parse::<Address>()
        .map_err(|e| anyhow::anyhow!("invalid to address: {}", e))?;

    let _value = value
        .get("value")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing or invalid value field"))?
        .parse::<U256>()
        .map_err(|e| anyhow::anyhow!("invalid value format: {}", e))?;

    let chain_id = value.get("chainId").and_then(|v| {
        v.as_str().and_then(|s| {
            if let Some(hex_str) = s.strip_prefix("0x") {
                u64::from_str_radix(hex_str, 16).ok()
            } else {
                s.parse::<ChainId>().ok()
            }
        })
    });

    let function_signature = value.get("functionSignature").and_then(|v| {
        v.as_str().map(|s| {
            // Try to decode as hex first, otherwise treat as UTF-8 string
            let hex_str = s.strip_prefix("0x").unwrap_or(s);
            hex::decode(hex_str)
                .map(Bytes::from)
                .unwrap_or_else(|_| Bytes::from(s.as_bytes().to_vec()))
        })
    });

    let data = value.get("data").and_then(|v| {
        v.as_str().and_then(|s| {
            let hex_str = s.strip_prefix("0x").unwrap_or(s);
            hex::decode(hex_str).ok().map(Bytes::from)
        })
    });

    let decoded = if function_signature.is_some() && data.is_some() {
        tracing::info!("decoding calldata with function signature");

        match decode_calldata(&data.clone().unwrap(), &function_signature.clone().unwrap()) {
            Ok((func, decoded_args)) => Some((func, decoded_args)),
            Err(e) => {
                tracing::warn!("failed to decode calldata: {}", e);
                None
            }
        }
    } else {
        None
    };

    let serialized_function_arguments = decoded.clone().map(|(_, decoded_function_arguments)| {
        decoded_function_arguments
            .iter()
            .map(serialize_sol_value)
            .collect::<Vec<serde_json::Value>>()
    });

    Ok(ParsedIntent {
        from,
        to,
        value: _value,
        data,
        chain_id,
        function_signature,
        function: decoded.clone().map(|(func, _)| serde_json::to_value(&func).unwrap()),
        decoded_function_signature: decoded.map(|(func, _)| func.full_signature()),
        decoded_function_arguments: serialized_function_arguments,
    })
}

pub fn decode_calldata(calldata: &Bytes, function_signature: &Bytes) -> anyhow::Result<(Function, Vec<DynSolValue>)> {
    let function_signature_str = String::from_utf8(function_signature.to_vec())
        .map_err(|e| anyhow::anyhow!("Invalid UTF-8 in function signature: {}", e))?;

    let func = Function::parse(&function_signature_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse function signature: {}", e))?;

    // Validate calldata length
    if calldata.len() < 4 {
        anyhow::bail!("Calldata too short: expected at least 4 bytes for function selector");
    }

    let selector: [u8; 4] = calldata[..4]
        .try_into()
        .map_err(|e| anyhow::anyhow!("Failed to extract function selector: {}", e))?;

    // Ensure selector matches
    if selector != func.selector() {
        anyhow::bail!(
            "Function selector mismatch. Expected: 0x{}, Found: 0x{}",
            hex::encode(func.selector()),
            hex::encode(selector)
        );
    }

    // Use alloy's built-in ABI decoding for comprehensive type support
    let inputs: Vec<DynSolValue> = func
        .abi_decode_input(&calldata[4..])
        .map_err(|e| anyhow::anyhow!("Failed to decode calldata: {}", e))?;

    Ok((func, inputs))
}

/// Serialize the sol value to a JSON value with proper type handling and padding removal
/// NOTE: uint and int are serialized as strings to preserve precision for large numbers
/// NOTE: addresses are serialized as lowercase strings
pub fn serialize_sol_value(value: &DynSolValue) -> serde_json::Value {
    match value {
        DynSolValue::Bool(b) => serde_json::Value::Bool(*b),

        // Handle signed integers with proper sign handling
        DynSolValue::Int(i, _bits) => {
            // Convert to string to preserve precision for large numbers
            serde_json::Value::String(format!("{}", i))
        }

        // Handle unsigned integers with proper precision
        DynSolValue::Uint(u, _bits) => {
            // Convert to string to preserve precision for large numbers
            serde_json::Value::String(format!("{}", u))
        }

        // Handle fixed-size bytes (remove padding)
        DynSolValue::FixedBytes(b, _) => {
            // Remove trailing zeros (padding) from fixed bytes
            let trimmed = b.iter().rev().skip_while(|&&x| x == 0).collect::<Vec<_>>();
            let trimmed: Vec<u8> = trimmed.into_iter().rev().copied().collect();
            serde_json::Value::String(format!("0x{}", hex::encode(trimmed)))
        }

        // Handle addresses (already properly formatted)
        DynSolValue::Address(a) => serde_json::Value::String(a.to_string().to_lowercase()),

        // Handle function selectors
        DynSolValue::Function(f) => serde_json::Value::String(format!("0x{}", hex::encode(f.0))),

        // Handle dynamic bytes (no padding to remove)
        DynSolValue::Bytes(b) => serde_json::Value::String(format!("0x{}", hex::encode(b))),

        // Handle strings
        DynSolValue::String(s) => serde_json::Value::String(s.to_string()),

        // Handle dynamic arrays
        DynSolValue::Array(a) => serde_json::Value::Array(a.iter().map(serialize_sol_value).collect()),

        // Handle fixed-size arrays
        DynSolValue::FixedArray(a) => serde_json::Value::Array(a.iter().map(serialize_sol_value).collect()),

        // Handle tuples (structs)
        DynSolValue::Tuple(t) => serde_json::Value::Array(t.iter().map(serialize_sol_value).collect()),
    }
}
pub fn generate_local_policy_input(
	_policy_path: &str,
	params_path: &str,
	intent_path: &str,
	_schema_path: Option<&str>,
	entrypoint: &str, // e.g. "example.allow"
) -> Result<Value, String> {
	// Read files
	let params: Value = serde_json::from_str(&fs::read_to_string(params_path).map_err(|_| "Missing params file".to_string())?).map_err(|_| "Invalid params JSON".to_string())?;
	let intent: Value = serde_json::from_str(&fs::read_to_string(intent_path).map_err(|_| "Missing intent file".to_string())?).map_err(|_| "Invalid intent JSON".to_string())?;

	// Prepare intermediary directory
	let intermediary_dir = "intermediary";
	fs::create_dir_all(intermediary_dir).map_err(|e| format!("Failed to create intermediary dir: {}", e))?;

	// Write data.json (policy params and wasm simulation data)
	let data_json_path = format!("{}/data.json", intermediary_dir);
	let mut data_obj = serde_json::Map::new();
	data_obj.insert("params".to_string(), params.clone());
	// Merge with wasm_data.json
	let wasm_data_path = format!("{}/wasm_data.json", intermediary_dir);
	if let Ok(wasm_data_str) = fs::read_to_string(&wasm_data_path) {
		if let Ok(wasm_data_json) = serde_json::from_str::<Value>(&wasm_data_str) {
			data_obj.insert("data".to_string(), wasm_data_json);
		}
	}
	let data_json = Value::Object(data_obj);
	fs::write(&data_json_path, serde_json::to_string_pretty(&data_json).unwrap()).map_err(|e| format!("Failed to write data.json: {}", e))?;


    // Parse the intent and write parsed intent to input.json
    let input_json_path = format!("{}/input.json", intermediary_dir);
    let parsed_intent = match parse_intent(intent) {
        Ok(parsed) => parsed,
        Err(e) => return Err(format!("Failed to parse intent: {}", e)),
    };
    fs::write(&input_json_path, serde_json::to_string_pretty(&parsed_intent).unwrap())
        .map_err(|e| format!("Failed to write input.json: {}", e))?;

	Ok(serde_json::json!({
		"status": "ok",
		"wrote": {
			"data_json": data_json_path,
			"input_json": input_json_path
		},
		"entrypoint": entrypoint
	}))
}