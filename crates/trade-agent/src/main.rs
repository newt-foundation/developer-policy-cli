use alloy::primitives::{Address, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::SolCall;
use clap::{Parser, ValueEnum};
use dotenvy::dotenv;
use eyre::Result;
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder};
use serde::{Deserialize, Serialize};
use shared::spawn_loading_animation;
use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub policy_client: Address,
    pub intent: TaskIntent,
    pub quorum_number: Option<Vec<u8>>,
    pub quorum_threshold_percentage: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskIntent {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub data: String,
    pub chain_id: U256,
    pub function_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskIdResponse {
    pub task_request_id: String,
    pub task_request: CreateTaskRequest,
    pub status: String,
    pub result: Option<CreateTaskResult>,
    pub error: Option<String>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskResult {
    pub task_id: String,
    pub tx_hash: String,
}

// Mock swapper contract interface
alloy::sol! {
    interface MockUSDCSwapPool {
        function buy(address buyToken, uint256 amountIn, uint32 slippage) external returns (uint256 amountOut);

        function sell(address sellToken, uint256 amountIn, uint32 slippage) external returns (uint256 amountOut);
    }
}

// USDC token address, since we always swap usdc -> <token>
const USDC_TOKEN_ADDRESS: &str = "0xd1c01582bee80b35898cc3603b75dbb5851b4a85";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TokenPair(BTreeSet<Address>);

impl TokenPair {
    fn new(a: Address, b: Address) -> Self {
        let mut set = BTreeSet::new();
        set.insert(a);
        set.insert(b);
        TokenPair(set)
    }
}

/// swap contract address for a token pair
fn get_swap_contract_address(token_a: Address, token_b: Address) -> Result<Address> {
    let mut token_to_swapper: HashMap<TokenPair, Address> = HashMap::new();

    // USDC < > WETH swapper
    let usdc = Address::from_str(USDC_TOKEN_ADDRESS)?;
    let weth = Address::from_str("0xe42e3458283032c669c98e0d8f883a92fc64fe22")?; // WETH address
    let mock_swapper = Address::from_str("0x03139ec37282064316be0f1e9216a5d4d3a74dda")?;

    token_to_swapper.insert(TokenPair::new(usdc, weth), mock_swapper);

    let key = TokenPair::new(token_a, token_b);
    token_to_swapper.get(&key).cloned().ok_or_else(|| {
        eyre::eyre!("No swap contract found for token pair: {token_a:?}, {token_b:?}")
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, ValueEnum)]
enum BuyOrSell {
    #[value(
        name = "buy",
        help = "Buy the token with USDC - Exchange USDC for the specified token"
    )]
    Buy,
    #[value(
        name = "sell",
        help = "Sell the token for USDC - Exchange the specified token for USDC"
    )]
    Sell,
}

impl BuyOrSell {
    /// Returns the string representation of the enum
    pub fn as_str(&self) -> &'static str {
        match self {
            BuyOrSell::Buy => "buy",
            BuyOrSell::Sell => "sell",
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Newton Trade Agent - Automated trading bot for token swaps",
    long_about = "A trading agent that monitors market conditions and executes token swaps through the Newton protocol. Supports buying and selling tokens with USDC."
)]
struct Args {
    /// The policy client address (i.e. policy-guarded vault address) for the user
    #[arg(short, long)]
    client: Address,

    /// The token address to buy or sell with USDC
    #[arg(short, long)]
    token: Address,

    /// The amount to swap (in token units)
    #[arg(short, long)]
    amount: u64,

    /// The slippage to use for the swap
    #[arg(short, long)]
    slippage: u32,

    /// Whether to buy the token with USDC or sell the token for USDC
    #[arg(short, long)]
    trade: BuyOrSell,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let args = Args::parse();
    let amount_str = (args.amount as f64 / 10f64.powf(9f64)).round() /* USDC decimals */;

    info!("🚀 Newton Trade Agent Starting...");

    info!(
        "[Trading Agent] Operation: {} {} with USDC for {} units ({} USDC)",
        args.trade.as_str(),
        args.token,
        args.amount,
        amount_str
    );

    // create signer and get from address
    let signer = PrivateKeySigner::from_str(&std::env::var("AGENT_PRIVATE_KEY")?)?;
    let from_address = signer.address();

    info!("[Trading Agent] Creating trade intent...");

    // create intent
    let chain_id = 11155111; // Default chain ID
    let (task_intent, function_signature, token, amount, slippage) = create_swap_intent(
        from_address,
        args.token,
        args.amount,
        args.trade.clone(),
        args.slippage,
        chain_id,
    )?;

    info!("[Trading Agent] Trade intent: {:?}", task_intent);

    info!(
        "[Trading Agent] Function signature: {}, token: {}, amount: {} ({} USDC), slippage: {}%",
        function_signature, token, amount, amount_str, slippage
    );

    let newton_rpc = "http://localhost:8545"; // Default RPC URL
    let client = HttpClientBuilder::default().build(newton_rpc)?;
    let policy_client = args.client;

    info!(
        "[Trading Agent] Vault address: {}. Requesting policy evaluation for intent...",
        policy_client
    );

    // Start loading animation in a separate thread
    let loading_handle = spawn_loading_animation(
        "[Trading Agent] Submitting request to Newton Protocol",
        5000,
    );

    // submit task
    let task_response: TaskIdResponse = client
        .request(
            "newton_createTask",
            vec![CreateTaskRequest {
                policy_client,
                intent: task_intent,
                quorum_number: None,
                quorum_threshold_percentage: None,
            }],
        )
        .await
        .map_err(|e| eyre::eyre!("RPC error: {}", e))?;

    // Stop the loading animation
    {
        let mut stop_guard = loading_handle.lock().unwrap();
        *stop_guard = true;
    }

    // Start loading animation for waiting phase
    let waiting_handle = spawn_loading_animation(
        "Waiting for task id for policy evaluation",
        2000, // 2 seconds max (20s)
    );

    // warmly wait
    let wait_request = WaitForTaskIdRequest {
        task_request_id: task_response.task_request_id.clone(),
        timeout: Some(1000),
    };

    let final_response: TaskIdResponse = client
        .request("newton_waitForTaskId", vec![wait_request])
        .await
        .map_err(|e| eyre::eyre!("RPC error: {}", e))?;

    // Stop the waiting animation
    {
        let mut stop_guard = waiting_handle.lock().unwrap();
        *stop_guard = true;
    }

    match final_response.status.as_str() {
        "Completed" => {
            info!("✅ Task ID: {}", final_response.result.unwrap().task_id);
        }
        "Failed" => {
            error!(
                "❌ Unexpected failure: {}",
                final_response.error.as_ref().unwrap()
            );
            return Err(eyre::eyre!(
                "Failed to get task id: {}",
                final_response.error.as_ref().unwrap()
            ));
        }
        _ => {
            panic!("Unexpected behavior: {}", final_response.status);
        }
    }

    Ok(())
}

fn create_swap_intent(
    from: Address,
    token: Address,
    amount: u64,
    buy_or_sell: BuyOrSell,
    slippage: u32,
    chain_id: u64,
) -> Result<(
    TaskIntent,
    String,  /* function signature */
    Address, /* token */
    U256,    /* amount */
    u32,     /* slippage */
)> {
    let usdc_address = Address::from_str(USDC_TOKEN_ADDRESS)?;
    let swap_contract_address = get_swap_contract_address(usdc_address, token)?;

    // encode the swap function call: swap(uint256 _amount, uint256 _swapNumber)
    let (encoded_data, function_signature) = match buy_or_sell {
        BuyOrSell::Buy => {
            let call = MockUSDCSwapPool::buyCall {
                buyToken: token,
                amountIn: U256::from(amount),
                slippage: slippage,
            };
            (call.abi_encode(), MockUSDCSwapPool::buyCall::SIGNATURE)
        }
        BuyOrSell::Sell => {
            let call = MockUSDCSwapPool::sellCall {
                sellToken: token,
                amountIn: U256::from(amount),
                slippage: slippage,
            };
            (call.abi_encode(), MockUSDCSwapPool::sellCall::SIGNATURE)
        }
    };

    // get the encoded data and function signature

    let intent = TaskIntent {
        from,
        to: swap_contract_address,
        value: U256::ZERO,
        data: hex::encode(&encoded_data),
        chain_id: U256::from(chain_id),
        function_signature: hex::encode(function_signature),
    };

    Ok((
        intent,
        function_signature.to_string(),
        token,
        U256::from(amount),
        slippage,
    ))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitForTaskIdRequest {
    pub task_request_id: String,
    pub timeout: Option<u64>,
}
