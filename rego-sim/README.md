### Pre requisites
The run_rego_policy.sh requires node and cargo to be installed

This step assumes you already have a `policy.wasm` file that returns some real world data.

### Usage

1. Create a rego file in this folder named `policy.rego`
```
package example_policy

default allow := false

allow if {
  data.params.max_gas_price < data.data['11155111']
  input.function.name == "buy"
}
```

2. Create a json file in this folder named `test_intent.json` with the intent you wish you evaluate against your rego
```
{
  "from": "0xF2168be2F3aE8989696705b211c7b9d65E3416dA",
  "to": "0x8f86403A4DE0BB5791fa46B8e795C547942fE4Cf",
  "value": "0x0",
  "data": "0x28dca9f70000000000000000000000008f86403a4de0bb5791fa46b8e795c547942fe4cf000000000000000000000000000000000000000000000000000000174876e8000000000000000000000000000000000000000000000000000000000000000002",
  "chainId": 11155111,
  "functionSignature": "0x62757928616464726573732c75696e743235362c75696e74333229"
}
```

3. Create a json file in this folder named `policy_params_data.json` with the policy data params you intent to set on your policy client
```
{
  "max_gas_price": 1
}
```

4. Locate the reference to your `policy.wasm` file. 
In this case we'll use the max-gas-price policy.wasm located in `/policy-examples/max-gas-price/policy-files/policy.wasm`

5. Evaluate a the rego policy with regorus:
```sh
./run_rego_policy.sh <policy wasm> <wasm args> <rego entry point>

./run_rego_policy.sh ../policy-examples/max-gas-price/policy-files/policy.wasm "{}" "example_policy.allow"
```
