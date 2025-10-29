#!/bin/bash
# run_rego_policy.sh - Run the full Rego policy test pipeline
# Usage: ./run_rego_policy.sh <policy_wasm> <policy_params_data.json> <test_intent.json> <policy.rego> <rego_query>
# Example:
#   ./run_rego_policy.sh ../policy-examples/max-gas-price/policy-files/policy.wasm policy_params_data.json test_intent.json policy.rego "data.example.allow"

set -e

POLICY_WASM="$1"
PARAMS_JSON="$2"
INTENT_JSON="$3"
POLICY_REGO="$4"
REGO_QUERY="$5"

# Intermediate files
WASM_DATA="wasm_data.json"
DATA_JSON="data.json"
INPUT_JSON="input.json"

# 1. Run WASM simulation
cargo run --manifest-path ../op-sim/Cargo.toml --release -- "$POLICY_WASM" '' > "$WASM_DATA"

# 2. Marshal data.json
node marshal_data.js "$PARAMS_JSON" "$WASM_DATA" "$DATA_JSON"

# 3. Marshal input.json
node marshal_input.js "$INTENT_JSON" "$INPUT_JSON"

# 4. Run regorus
./regorus eval --input "$INPUT_JSON" --data "$DATA_JSON" --data "$POLICY_REGO" "$REGO_QUERY"
