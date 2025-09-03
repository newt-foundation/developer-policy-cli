mod price_fetcher;

use alloy::primitives::{Address, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::SolCall;
use clap::Parser;
use dotenvy::dotenv;
use eyre::Result;
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder};
use price_fetcher::{NativePriceFetcher, PriceFetcher};
use shared::strategy::should_trade;
use std::str::FromStr;
use tracing::{error, info};

// Vendored types from newton-prover-rpc (simplified)
use serde::{Deserialize, Serialize};

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
    interface MockSwapper {
        function swap(uint256 _amount, uint256 _swapNumber) external;
    }
}

// uh this is the venue where weth we swap token1 for token2
const MOCK_SWAPPER_ADDRESS: &str = "0x03139ec37282064316be0f1e9216a5d4d3a74dda";
const SWAP_AMOUNT: u64 = 1_000_000;
const TOKEN1_FOR_TOKEN2: u64 = 1; // USDC -> WETH direction

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Policy client address
    policy_client: Address,

    /// Newton RPC URL
    #[arg(
        long,
        env = "NEWTON_RPC",
        default_value = "https://prover-avs.stagef.newt.foundation/"
    )]
    newton_rpc: String,

    /// Chain ID
    #[arg(long, env = "CHAIN_ID", default_value = "11155111")]
    chain_id: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let args = Args::parse();
    let coingecko_api_key = std::env::var("COINGECKO_API_KEY").ok();
    let eth_rpc_signer =
        std::env::var("ETH_SIGNER").expect("ETH_SIGNER environment variable is required");

    info!(
        "Starting trade agent with policy client: {}",
        args.policy_client
    );

    // fetch price data
    let fetcher = NativePriceFetcher::new(coingecko_api_key);
    let coin_ids = ["usd-coin", "weth", "bitcoin", "cosmos"];
    let price_data = fetcher.get_price_data(&coin_ids).await?;

    info!("Fetched price data: {:?}", price_data);

    // check trading signal
    if let Some((from_token, to_token)) = should_trade(&price_data) {
        info!("Trade signal: {} -> {}", from_token, to_token);

        // create signer and get from address
        let signer = PrivateKeySigner::from_str(&eth_rpc_signer)?;
        let from_address = signer.address();

        // create intent
        let task_intent = create_swap_intent(from_address, args.chain_id)?;

        info!("Created swap intent: {:?}", task_intent);

        let client = HttpClientBuilder::default().build(&args.newton_rpc)?;

        // submit task
        let task_response = create_task(
            &client,
            CreateTaskRequest {
                policy_client: args.policy_client,
                intent: task_intent,
                quorum_number: None,
                quorum_threshold_percentage: None,
            },
        )
        .await?;

        info!("Task created with ID: {}", task_response.task_request_id);

        // warmly wait
        let final_response = wait_for_task(&client, &task_response.task_request_id).await?;

        match final_response.status.as_str() {
            "Completed" => {
                info!("Task completed successfully: {:?}", final_response);
            }
            "Failed" => {
                error!("Task failed: {:?}", final_response.error);
                return Err(eyre::eyre!("Task failed"));
            }
            _ => {
                info!("Task status: {}", final_response.status);
            }
        }
    } else {
        info!("No trading signal detected");
    }

    Ok(())
}

fn create_swap_intent(from: Address, chain_id: u64) -> Result<TaskIntent> {
    let mock_swapper_address = Address::from_str(MOCK_SWAPPER_ADDRESS)?;

    // encode the swap function call: swap(uint256 _amount, uint256 _swapNumber)
    let swap_call = MockSwapper::swapCall {
        _amount: U256::from(SWAP_AMOUNT),
        _swapNumber: U256::from(TOKEN1_FOR_TOKEN2),
    };

    // get the encoded data and function signature
    let encoded_data = swap_call.abi_encode();
    let function_signature = MockSwapper::swapCall::SIGNATURE;

    let intent = TaskIntent {
        from,
        to: mock_swapper_address,
        value: U256::ZERO, // no ETH being sent
        data: hex::encode(&encoded_data),
        chain_id: U256::from(chain_id),
        function_signature: hex::encode(function_signature),
    };

    Ok(intent)
}

async fn create_task(client: &impl ClientT, request: CreateTaskRequest) -> Result<TaskIdResponse> {
    let task_response: TaskIdResponse = client
        .request("newton_createTask", vec![request])
        .await
        .map_err(|e| eyre::eyre!("RPC error: {}", e))?;

    Ok(task_response)
}

async fn wait_for_task(client: &impl ClientT, task_request_id: &str) -> Result<TaskIdResponse> {
    let wait_request = WaitForTaskIdRequest {
        task_request_id: task_request_id.to_string(),
        timeout: Some(300),
    };

    let final_response: TaskIdResponse = client
        .request("newton_waitForTaskId", vec![wait_request])
        .await
        .map_err(|e| eyre::eyre!("RPC error: {}", e))?;

    Ok(final_response)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitForTaskIdRequest {
    pub task_request_id: String,
    pub timeout: Option<u64>,
}
