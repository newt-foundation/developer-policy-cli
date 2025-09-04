# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a proof-of-concept automated trading agent that integrates with the Newton Protocol for policy-based trade execution using Rego policy evaluation. The system executes token swaps through smart contracts while adhering to user-defined policies evaluated by the Newton Protocol.

## Architecture

Three-crate workspace structure:
- **`crates/trade-agent/`**: Main CLI application handling Newton Protocol communication and trade execution
- **`crates/wasm-component/`**: WebAssembly component for market data analysis and trading signal computation 
- **`crates/shared/`**: Common utilities including price data structures, trading strategies, and token mappings

Key architectural components:
- Policy-based trading through Rego policy evaluation (`rego.rego`)
- Modular trading strategy system in `shared/src/strategy.rs`
- Market data integration through WASM components
- Newton Protocol RPC integration for decentralized policy evaluation

## Development Commands

### Building
```bash
# Build all components
make build-all

# Build individual components
make build-agent    # Main trading CLI
make build-wasm     # Market analysis WASM component

# Manual builds
cargo build -p trade-agent --release
cargo build -p https-test --target wasm32-wasip2 --release
```

### Running
```bash
# Show CLI help
make agent-help

# Execute trades (requires environment setup)
make run-agent client=0x... token=0x... amount=1000000000 trade=buy

# Run WASM market analysis
make run-wasm
```

### Testing
```bash
cargo test --workspace
cargo test -p trade-agent
cargo test -p shared
```

### Environment Setup
```bash
cp .env.example .env
# Configure AGENT_PRIVATE_KEY, NEWTON_RPC, CHAIN_ID
```

## Dependencies & Toolchain

Required Rust version: 1.88.0+

Essential tools:
- `cargo-component` for WASM component builds
- `wasmtime` for running WASM components  
- `wasm32-wasip2` target for WebAssembly compilation

Alternative: Use Nix development environment with `nix develop`

## Key Code Patterns

### Token Configuration
Tokens are defined in `crates/shared/src/tokens.rs` with:
- Contract addresses (currently Sepolia testnet)
- CoinGecko ID mappings for market data
- Symbol constants (USDC_ADDRESS, WETH_ADDRESS, etc.)

### Trading Strategy Extension
Implement new strategies in `crates/shared/src/strategy.rs`:
- `should_trade()` function for signal computation
- `calculate_200dma()` for technical indicators
- Returns token pair tuples for swap execution

### Newton Protocol Integration
Trade execution flow in `crates/trade-agent/src/main.rs`:
1. Create trading intent with transaction details
2. Submit to Newton Protocol via `newton_createTask` RPC call
3. Policy evaluation against Rego rules with real-time market data
4. Conditional execution based on policy approval

### Policy Configuration  
Rego policy in `rego.rego` evaluates:
- Token whitelist (`whitelist_contracts`)
- Function restrictions (`allowed_action`)
- Amount limits (`max_limit`)
- Market conditions (price vs 200-day moving average)
- Market cap requirements (rank ≤ 200)

### WASM Market Analysis
Market data fetching in `crates/wasm-component/src/wasi_fetcher.rs`:
- CoinGecko API integration for price data
- Technical indicator computation (200-day moving average)
- Market cap ranking analysis
- Trading signal generation

## Network Configuration

Currently configured for Sepolia testnet:
- Chain ID: 11155111
- USDC: `0xd1c01582bee80b35898cc3603b75dbb5851b4a85`
- WETH: `0xe42e3458283032c669c98e0d8f883a92fc64fe22`
- Mock Swapper: `0x03139ec37282064316be0f1e9216a5d4d3a74dda`

## CLI Usage Patterns

All trade operations require four parameters:
- `--client`: Policy-guarded vault address
- `--token`: ERC-20 contract address to trade
- `--amount`: Amount in token's smallest unit
- `--trade`: Either `buy` (USDC→token) or `sell` (token→USDC)

Example:
```bash
./target/release/trade-agent --client 0x1234... --token 0xe42e... --amount 1000000000 --trade buy
```

## Important Implementation Notes

- All trades go through USDC pairs only
- Market analysis runs independently via WASM components
- Policy evaluation includes real-time market data context
- Token pair mappings are hardcoded in `get_swap_contract_address()`
- Loading animations provide user feedback during Newton Protocol requests
- Debug logging available via `RUST_LOG=debug`
