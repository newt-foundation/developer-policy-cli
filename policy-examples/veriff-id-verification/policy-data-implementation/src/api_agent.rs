use serde::{Deserialize, Serialize};
use serde_json;

// Import the HTTP client from generated bindings
use crate::newton::provider::http::{HttpRequest, HttpResponse, fetch};

// Request structure that will be sent to the API
#[derive(Debug, Serialize)]
pub struct VerifyRequest {
    pub session_id: String,
    pub user_id: String,
}

// Response structure from the API
#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyResponse {
    pub verification_status: String,
    pub session_id: String,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_time: Option<String>,
}

pub struct ApiAgent {
    api_url: String,
}

impl ApiAgent {
    pub fn new() -> Self {
        Self {
            // Using the provided API URL
            api_url: "https://218736da946d.ngrok-free.app/api/newton/verify".to_string(),
        }
    }

    pub fn verify_session(&self, session_id: String, user_id: String) -> Result<String, String> {
        // Create the request payload
        let request_payload = VerifyRequest {
            session_id: session_id.clone(),
            user_id: user_id.clone(),
        };

        // Serialize the request to JSON
        let json_body = serde_json::to_string(&request_payload)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        // Make the HTTP request
        let response = match self.make_request(json_body) {
            Ok(resp) => resp,
            Err(_e) => {
                // Handle network errors gracefully
                let error_response = VerifyResponse {
                    verification_status: "error".to_string(),
                    session_id,
                    user_id,
                    firstname: None,
                    lastname: None,
                    document_type: None,
                    country: None,
                    submission_time: None,
                    decision_time: None,
                };

                return serde_json::to_string_pretty(&error_response)
                    .map_err(|e| format!("Failed to format error response: {}", e));
            }
        };

        // Parse the response body as string
        let response_str = String::from_utf8(response.body)
            .map_err(|e| format!("Failed to decode response as UTF-8: {}", e))?;

        // Check if the response looks like JSON (basic check)
        let trimmed = response_str.trim();
        if !trimmed.starts_with('{') && !trimmed.starts_with('[') {
            // Response is not JSON, likely an error page
            let error_response = VerifyResponse {
                verification_status: "server_unavailable".to_string(),
                session_id,
                user_id,
                firstname: None,
                lastname: None,
                document_type: None,
                country: None,
                submission_time: None,
                decision_time: None,
            };

            return serde_json::to_string_pretty(&error_response)
                .map_err(|e| format!("Failed to format error response: {}", e));
        }

        // Try to parse the response
        match serde_json::from_str::<VerifyResponse>(&response_str) {
            Ok(verify_response) => {
                // Successfully parsed, return the response
                serde_json::to_string_pretty(&verify_response)
                    .map_err(|e| format!("Failed to format response: {}", e))
            }
            Err(_) => {
                // Failed to parse as expected structure, return error response
                let error_response = VerifyResponse {
                    verification_status: "invalid_response".to_string(),
                    session_id,
                    user_id,
                    firstname: None,
                    lastname: None,
                    document_type: None,
                    country: None,
                    submission_time: None,
                    decision_time: None,
                };

                serde_json::to_string_pretty(&error_response)
                    .map_err(|e| format!("Failed to format error response: {}", e))
            }
        }
    }

    fn make_request(&self, body: String) -> Result<HttpResponse, String> {
        // Prepare headers
        let mut headers = Vec::new();
        headers.push(("Content-Type".to_string(), "application/json".to_string()));
        headers.push(("User-Agent".to_string(), "newton-api-agent/1.0".to_string()));
        headers.push(("Accept".to_string(), "application/json".to_string()));

        // Add ngrok-skip-browser-warning header for ngrok URLs
        if self.api_url.contains("ngrok") {
            headers.push(("ngrok-skip-browser-warning".to_string(), "true".to_string()));
        }

        // Create the HTTP request
        let request = HttpRequest {
            url: self.api_url.clone(),
            method: "POST".to_string(),
            headers,
            body: Some(body.into_bytes()),
        };

        // Make the request using the generated fetch function
        fetch(&request)
    }
}

// Public function that can be called from lib.rs
pub fn verify_user_session(session_id: String, user_id: String) -> Result<String, String> {
    let agent = ApiAgent::new();
    agent.verify_session(session_id, user_id)
}