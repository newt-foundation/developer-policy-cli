// Generate bindings from the WIT file
wit_bindgen::generate!({
    world: "newton-provider",  // Must match the world name in WIT
    path: "wit",               // Path to WIT files
});

// Module declarations
mod api_agent;

// Import the api_agent functionality
use api_agent::get_fed_interest_rate_sentiment;

// Export the implementation
export!(NewtonProvider);

struct NewtonProvider;

impl Guest for NewtonProvider {
    fn run(_input: String) -> Result<String, String> {
        // Always fetch and return the Fed interest rate sentiment from Polymarket
        get_fed_interest_rate_sentiment()
    }
}