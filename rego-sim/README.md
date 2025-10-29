## Using Regorus for Rego Policy Testing

This directory usesour included build of [regorus](https://github.com/microsoft/regorus) for evaluating Rego policies.

### Usage

To evaluate a policy with regorus:

```sh
cargo run --manifest-path ../op-sim/Cargo.toml --release -- ../policy-examples/max-gas-price/policy-files/policy.wasm '' > wasm_data.json

node merge_and_parse.js policy_params_data.json wasm_data.json data.json

./regorus eval --input input.json --data data.json --data policy.rego "data.example.allow"
```

Replace `input.json`, `data.json`, and `policy.rego` with your actual files as needed.