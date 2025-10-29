#!/bin/bash
# run_rego_policy.sh - Run the full Rego policy test pipeline (dev-friendly version)
# Usage: ./run_rego_policy.sh <policy_wasm> <wasm_args> <rego_query>
# Example:
#   ./run_rego_policy.sh ../policy-examples/max-gas-price/policy-files/policy.wasm '' investment_guardrails.allow

set -e

# Hardcoded file paths for dev convenience
PARAMS_JSON="policy_params_data.json"
INTENT_JSON="test_intent.json"
POLICY_REGO="policy.rego"

POLICY_WASM="$1"
WASM_ARGS="$2"

# Auto-prefix 'data.' if not present
REGO_QUERY="$3"
if [[ "$REGO_QUERY" != data.* ]]; then
  REGO_QUERY="data.$REGO_QUERY"
fi

# Intermediate files
WASM_DATA="wasm_data.json"
DATA_JSON="data.json"
INPUT_JSON="input.json"

# 1. Run WASM simulation
cargo run --manifest-path ../op-sim/Cargo.toml --release -- "$POLICY_WASM" "$WASM_ARGS" > "$WASM_DATA"

# 2. Marshal data.json
node marshal_data.js "$PARAMS_JSON" "$WASM_DATA" "$DATA_JSON"

# 3. Marshal input.json
node marshal_input.js "$INTENT_JSON" "$INPUT_JSON"

# 4. Run regorus
./regorus eval --input "$INPUT_JSON" --data "$DATA_JSON" --data "$POLICY_REGO" "$REGO_QUERY"
