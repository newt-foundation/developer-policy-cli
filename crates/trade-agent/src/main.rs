mod price_fetcher;

use alloy::primitives::{Address, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::SolCall;
use clap::Parser;
use dotenvy::dotenv;
use eyre::Result;
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder};
use price_fetcher::{NativePriceFetcher, PriceFetcher};
use serde::{Deserialize, Serialize};
use shared::strategy::should_trade;
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
    interface MockSwapper {
        function swap(uint256 _amount, uint256 _swapNumber) external;
    }
}

// not sure of a better way to provide this. agent probably just needs to know
const POLICY_CLIENT_ADDRESS: &str = "0xb1ad5f82407bc0f19f42b2614fb9083035a36b69";
// USDC -> <token> direction
const TOKEN1_FOR_TOKEN2: u64 = 1;
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// token to buy with usdc
    token: Address,

    /// amount to swap
    amount: u64,

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

    // fetch price data
    let fetcher = NativePriceFetcher::new(coingecko_api_key);
    let coin_ids = ["usd-coin", "weth", "bitcoin", "cosmos"];
    let price_data = fetcher.get_price_data(&coin_ids).await?;

    info!("Fetched price data: {:?}", price_data);

    // check trading signal
    if let Some((from_token, to_token)) = should_trade(&price_data) {
        info!("Trade signal: {} -> {}", from_token, to_token);

        // create signer and get from address
        let signer = PrivateKeySigner::random();
        let from_address = signer.address();

        // create intent
        let task_intent = create_swap_intent(from_address, args.token, args.amount, args.chain_id)?;

        info!("Created swap intent: {:?}", task_intent);

        let client = HttpClientBuilder::default().build(&args.newton_rpc)?;
        let policy_client = Address::from_str(POLICY_CLIENT_ADDRESS)?;

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

        info!("Task created with ID: {}", task_response.task_request_id);

        // warmly wait
        let wait_request = WaitForTaskIdRequest {
            task_request_id: task_response.task_request_id.clone(),
            timeout: Some(300),
        };

        let final_response: TaskIdResponse = client
            .request("newton_waitForTaskId", vec![wait_request])
            .await
            .map_err(|e| eyre::eyre!("RPC error: {}", e))?;

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

fn create_swap_intent(
    from: Address,
    token: Address,
    amount: u64,
    chain_id: u64,
) -> Result<TaskIntent> {
    // Always swap from USDC to the specified token
    let usdc_address = Address::from_str(USDC_TOKEN_ADDRESS)?;
    let swap_contract_address = get_swap_contract_address(usdc_address, token)?;

    // encode the swap function call: swap(uint256 _amount, uint256 _swapNumber)
    let swap_call = MockSwapper::swapCall {
        _amount: U256::from(amount),
        _swapNumber: U256::from(TOKEN1_FOR_TOKEN2),
    };

    // get the encoded data and function signature
    let encoded_data = swap_call.abi_encode();
    let function_signature = MockSwapper::swapCall::SIGNATURE;

    let intent = TaskIntent {
        from,
        to: swap_contract_address,
        value: U256::ZERO,
        data: hex::encode(&encoded_data),
        chain_id: U256::from(chain_id),
        function_signature: hex::encode(function_signature),
    };

    Ok(intent)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitForTaskIdRequest {
    pub task_request_id: String,
    pub timeout: Option<u64>,
}
