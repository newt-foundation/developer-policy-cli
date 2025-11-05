use serde::Deserialize;
use serde_json;

// Generate bindings from the WIT file
wit_bindgen::generate!({
    world: "newton-provider",  // Must match the world name in WIT
    path: "wit",               // Path to WIT files
});

// Module declarations
mod api_agent;

// Import the api_agent functionality
use api_agent::verify_user_session;

// Export the implementation
export!(NewtonProvider);

// Input structure for the run function
#[derive(Debug, Deserialize)]
struct InputArgs {
    session_id: String,
    user_id: String,
}

struct NewtonProvider;

impl Guest for NewtonProvider {
    fn run(input: String) -> Result<String, String> {
        // Parse the input JSON to get session_id and user_id
        let args: InputArgs = serde_json::from_str(&input)
            .map_err(|e| format!("Failed to parse input: {}. Expected JSON with 'session_id' and 'user_id' fields", e))?;

        // Call the API agent to verify the session
        verify_user_session(args.session_id, args.user_id)
    }
}

