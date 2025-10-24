# Farcaster User Data-Dependent Newton Policy (Demo)
Source files & compiled binaries for example Newton Protocol Policy demonstrating Farcaster user profile dependent guardrails.

## Structure

| directory    | purpose                                                                           |
|--------------|-----------------------------------------------------------------------------------|
| op-sim       | tooling for http provider to be used in WASM (Rust)                               |
| py_bindings  | Python code to compile to WASM                                                    |
| server       | external deployed lambda to wrap ext. API & keep API keys secure & offchain       |
| policy-files | Completed JSON/Rego/WASM for policy template files                                |

## Instructions
1. set up & deploy the API wrapper server as per [Server README](server)
2. Set the `URL` constant in [py_bindings/app.py](py_bindings/app.py) to the desired URL on the above API
3. compile to WASM

    cd py_bindings
    pip install componentize-py
    componentize-py -d ../newton-provider.wit -w newton-provider componentize --stub-wasi app -o ../policy.wasm
cd ../op-sim

4. follow steps in CLI README to deploy the template [CLI](https://github.com/newt-foundation/developer-policy-cli)
