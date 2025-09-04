# Newton Trade Agent

A proof-of-concept automated trading agent that integrates with the [Newton Protocol](https://newton.foundation/) for policy-based trade execution using [Rego](https://www.openpolicyagent.org/docs/latest/policy-language/) policy evaluation.

## Overview

The Newton Trade Agent is a modular Rust-based trading system designed to execute token swaps through smart contracts while adhering to user-defined policies. The agent submits trading intents to the Newton Protocol, which evaluates them against Rego policies before execution.

### Key Features

- **Policy-Based Trading**: All trades are evaluated against Rego policies before execution
- **Newton Protocol Integration**: Seamless integration with Newton's decentralized policy evaluation system
- **Modular Architecture**: Clean separation between trading logic, market analysis, and policy evaluation
- **WASM-Based Market Analysis**: Extensible market signal computation using WebAssembly
- **CLI Interface**: Simple command-line interface for executing trades
- **Multi-Token Support**: Support for various ERC-20 token swaps with USDC

## Architecture

The project follows a clean, extensible architecture with three main components:

```
poc-newton-trade-agent/
├── crates/
│   ├── trade-agent/     # Main CLI application
│   ├── wasm-component/  # Market analysis WASM component
│   └── shared/          # Common utilities and types
├── policy.rego           # Trading policy definition
└── Makefile           # Build automation
```

### Components

- **`trade-agent`**: CLI application that handles trade execution, Newton Protocol communication, and user interaction
- **`wasm-component`**: WebAssembly component for market data analysis and trading signal computation
- **`shared`**: Common utilities including price data structures, trading strategies, and token mappings

## Installation

### Dependencies

1. **Rust Toolchain** (1.88.0 or later):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **WASM Target**:

   ```bash
   rustup target add wasm32-wasip2
   ```

3. **cargo-component** (for WASM components):

   ```bash
   cargo install cargo-component
   ```

4. **Wasmtime** (for running WASM components):
   ```bash
   curl https://wasmtime.dev/install.sh -sSf | bash
   ```

### Alternative: Nix Development Environment

If you use Nix, you can enter a development shell with all dependencies:

```bash
nix develop
```

## Building

### Build All Components

```bash
make build-all
```

### Build Individual Components

```bash
# Build the main trading agent
make build-agent

# Build the WASM market analysis component
make build-wasm
```

### Manual Build Commands

```bash
# Trading agent (release build)
cargo build -p trade-agent --release

# WASM component
cargo build -p newton-trade-agent-wasm --target wasm32-wasip2 --release
```

## Configuration

### Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
cp .env.example .env
```

Required environment variables:

```env
# Your trading agent's private key (hex format without 0x prefix)
AGENT_PRIVATE_KEY=your_private_key_here

# Newton Protocol RPC endpoint
NEWTON_RPC=https://prover-avs.stagef.newt.foundation/

# Ethereum chain ID (11155111 for Sepolia)
CHAIN_ID=11155111

# Optional: API keys for market data
CMC_API_KEY=your_coinmarketcap_api_key_here
COINGECKO_API_KEY=your_coingecko_api_key_here
```

### Rego Policy Configuration

The trading policy is defined in `policy.rego`. The policy evaluates:

- **Token whitelist**: Only allows trading with approved tokens
- **Function restrictions**: Restricts allowed trading functions (`buy`, `sell`)
- **Amount limits**: Enforces maximum trade sizes
- **Market conditions**: Evaluates current price vs. 200-day moving average
- **Market cap requirements**: Only allows trading tokens with market cap rank ≤ 200

Example policy structure:

```rego
package newton_trading_agent

default allow := false

allow {
    # Token is whitelisted
    token in whitelist_contracts
    # Function is allowed
    function_name in allowed_action
    # Amount is within limits
    amount_in <= max_limit
    # Market conditions are favorable
    token_price >= token_daily_moving_average
    token_market_cap_rank <= 200
}
```

## Usage

### CLI Interface

The trade agent provides a comprehensive CLI interface:

```bash
# Show help and available options
make agent-help

# Or directly:
./target/release/trade-agent --help
```

### Basic Trading Commands

#### Buy Token with USDC

```bash
# Buy WETH with USDC
make run-agent client=0x1234...abcd token=0xe42e3458283032c669c98e0d8f883a92fc64fe22 amount=1000000000 trade=buy

# Or directly:
./target/release/trade-agent \
  --client 0x1234567890123456789012345678901234567890 \
  --token 0xe42e3458283032c669c98e0d8f883a92fc64fe22 \
  --amount 1000000000 \
  --trade buy
```

#### Sell Token for USDC

```bash
# Sell WETH for USDC
make run-agent client=0x1234...abcd token=0xe42e3458283032c669c98e0d8f883a92fc64fe22 amount=500000000 trade=sell
```

### Parameters

- `--client`: Your policy-guarded vault address (policy client)
- `--token`: ERC-20 token contract address to trade
- `--amount`: Amount in token's smallest unit (e.g., wei for ETH, considering token decimals)
- `--trade`: Either `buy` (exchange USDC for token) or `sell` (exchange token for USDC)

### Supported Tokens

Currently configured tokens:

- **USDC**: `0xd1c01582bee80b35898cc3603b75dbb5851b4a85` (Sepolia)
- **WETH**: `0xe42e3458283032c669c98e0d8f883a92fc64fe22` (Sepolia)

## Newton Protocol Integration

### How It Works

1. **Intent Creation**: The agent creates a trading intent with transaction details
2. **Policy Submission**: Intent is submitted to Newton Protocol with your policy client address
3. **Rego Evaluation**: Newton evaluates the intent against your Rego policy
4. **Market Data Integration**: Real-time market data is provided for policy evaluation
5. **Execution**: If policy allows, the trade is executed on-chain

### Request Flow

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Trade Agent   │───▶│ Newton Protocol │───▶│  Rego Evaluator │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │              ┌─────────────────┐              │
         └─────────────▶│  Smart Contract │◀─────────────┘
                        │   (DEX/Pool)    │
                        └─────────────────┘
```

### API Endpoints Used

- `newton_createTask`: Submit trading intent for policy evaluation
- `newton_waitForTaskId`: Wait for task completion and get results

## Development

### Running Market Analysis

The WASM component can be run independently for market analysis:

```bash
# Build and run the WASM market analyzer
make run-wasm
```

### Adding New Trading Strategies

Extend the strategy logic in `crates/shared/src/strategy.rs`:

```rust path=null start=null
pub fn my_custom_strategy(price_data: &TradingSignal) -> Option<(Address, Address)> {
    // Your custom trading logic here
    // Return Some((from_token, to_token)) to trade, None to skip
}
```

### Adding New Tokens

1. Update token mappings in `crates/shared/src/tokens.rs`
2. Add contract addresses to the swap contract mapping in `main.rs`
3. Update the Rego policy whitelist as needed

### Modifying Rego Policies

Edit `policy.rego` to customize trading rules:

```rego
# Add new conditions
allow {
    # Existing conditions...

    # Your custom conditions
    my_custom_condition
}

my_custom_condition {
    # Custom logic here
}
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Test specific component
cargo test -p trade-agent
cargo test -p shared
```

### Cleaning Build Artifacts

```bash
make clean
```

## Project Structure Details

```
poc-newton-trade-agent/
├── crates/
│   ├── trade-agent/
│   │   ├── src/main.rs          # CLI interface and Newton integration
│   │   └── Cargo.toml           # Trading agent dependencies
│   ├── wasm-component/
│   │   ├── src/
│   │   │   ├── main.rs          # WASM entry point
│   │   │   ├── wasi_fetcher.rs  # Market data fetching
│   │   │   └── bindings.rs      # WASM bindings
│   │   └── Cargo.toml           # WASM component config
│   └── shared/
│       ├── src/
│       │   ├── lib.rs           # Common utilities
│       │   ├── price.rs         # Price data structures
│       │   ├── strategy.rs      # Trading strategies
│       │   └── tokens.rs        # Token mappings
│       └── Cargo.toml           # Shared dependencies
├── policy.rego                    # Trading policy
├── .env.example                 # Environment template
├── Makefile                     # Build automation
├── flake.nix                    # Nix development environment
└── Cargo.toml                   # Workspace configuration
```

## Troubleshooting

### Common Issues

1. **"No swap contract found for token pair"**: Ensure the token addresses are configured in the swap contract mapping

2. **"RPC error"**: Check your Newton Protocol RPC endpoint and network connectivity

3. **"Policy evaluation failed"**: Review your Rego policy and ensure all conditions are met

4. **WASM build errors**: Ensure you have the correct Rust target and cargo-component installed:
   ```bash
   rustup target add wasm32-wasip2
   cargo install cargo-component
   ```

### Debug Mode

Run with debug logging:

```bash
RUST_LOG=debug ./target/release/trade-agent [options]
```

## Contributing

This is a proof-of-concept project. Contributions are welcome for:

- Additional trading strategies
- New token integrations
- Enhanced market analysis
- Policy template improvements
- Documentation updates

## License

This project is provided as-is for demonstration purposes. Please review and understand the code before using with real funds.

## Disclaimer

⚠️ **Important**: This is experimental software for educational purposes. Always test thoroughly with small amounts on testnets before using with real funds. Trading cryptocurrencies involves substantial risk of loss.
