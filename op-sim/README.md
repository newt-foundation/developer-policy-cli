## op-sim: Policy WASM Simulator

This directory contains a Rust-based tool for running and testing policy WebAssembly (WASM) files with custom input parameters. It allows developers to simulate and observe the output of policy logic in a local environment.

### Prerequisites
Make sure you are in the `op-sim` directory in your command line:

```sh
cd op-sim
```

### Install Cargo and Rust
Install Rust and Cargo (the Rust package manager) if you haven't already:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts, then restart your terminal.

### Run your compiled policy WASM file with input parameters

```sh
cargo run --release -- /path/to/policy/wasm '<your_input_string>'
```

### Example

```sh
cargo run --release -- ../policy-examples/max-gas-price/policy-files/policy.wasm ''
```

### Sample output

```
...
Compiling op-sim v0.1.0 (/Users/davidhe/Desktop/code/magic/developer-policy-cli/op-sim)
 Finished `release` profile [optimized] target(s) in 1m 26s
  Running `target/release/op-sim ../policy-examples/max-gas-price/policy-files/policy.wasm ''`
{"1":0.297310309,"11155111":0.001}
```
