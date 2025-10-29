### Usage

To evaluate a policy with regorus:

```sh
./run_rego_policy.sh <policy wasm> <wasm args> <rego entry point>

./run_rego_policy.sh ../policy-examples/max-gas-price/policy-files/policy.wasm "{}" "max_gas_price.allow"
```
