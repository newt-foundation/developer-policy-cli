use serde::{Deserialize, Serialize};
use serde_json;

// Import the HTTP client from generated bindings
use crate::newton::provider::http::{HttpRequest, HttpResponse, fetch};

// Polymarket API response structures
#[derive(Debug, Deserialize)]
pub struct PolymarketResponse {
    pub markets: Vec<Market>,
}

#[derive(Debug, Deserialize)]
pub struct Market {
    pub id: String,
    #[serde(rename = "lastTradePrice")]
    pub last_trade_price: Option<f64>,
}

// Interest rate sentiment response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct InterestRateSentiment {
    pub sentiment: String, // "decrease" or "not_decrease"
    pub decrease_percentage: f64,
    pub not_decrease_percentage: f64,
}

pub struct ApiAgent;

impl ApiAgent {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_interest_rate_sentiment(&self) -> Result<String, String> {
        // Hardcoded Polymarket API URL for Fed decision in December
        let polymarket_url = "https://gamma-api.polymarket.com/events/slug/fed-decision-in-december";

        // Make the GET request to Polymarket API
        let response = match self.make_get_request(polymarket_url) {
            Ok(resp) => resp,
            Err(e) => {
                return Err(format!("Failed to fetch Polymarket data: {}", e));
            }
        };

        // Parse the response body as string
        let response_str = String::from_utf8(response.body)
            .map_err(|e| format!("Failed to decode response as UTF-8: {}", e))?;

        // Parse the JSON response
        let polymarket_data: PolymarketResponse = serde_json::from_str(&response_str)
            .map_err(|e| format!("Failed to parse Polymarket response: {}", e))?;

        // Process the market data
        let sentiment = self.calculate_sentiment(&polymarket_data.markets)?;

        // Return the sentiment as JSON string
        serde_json::to_string_pretty(&sentiment)
            .map_err(|e| format!("Failed to format sentiment response: {}", e))
    }

    fn calculate_sentiment(&self, markets: &[Market]) -> Result<InterestRateSentiment, String> {
        // Hardcoded market IDs
        const DECREASE_50_BPS_ID: &str = "570360";  // 50+ bps decrease
        const DECREASE_25_BPS_ID: &str = "570361";  // 25 bps decrease
        const NO_CHANGE_ID: &str = "570362";        // No change
        const INCREASE_25_BPS_ID: &str = "570363";  // 25+ bps increase

        let mut decrease_sum = 0.0;
        let mut not_decrease_sum = 0.0;

        for market in markets {
            if let Some(price) = market.last_trade_price {
                match market.id.as_str() {
                    DECREASE_50_BPS_ID | DECREASE_25_BPS_ID => {
                        decrease_sum += price;
                    }
                    NO_CHANGE_ID | INCREASE_25_BPS_ID => {
                        not_decrease_sum += price;
                    }
                    _ => {
                        // Ignore other market IDs
                    }
                }
            }
        }

        // Calculate total and percentages
        let total = decrease_sum + not_decrease_sum;

        if total == 0.0 {
            return Err("No valid market data found".to_string());
        }

        let decrease_percentage = (decrease_sum / total) * 100.0;
        let not_decrease_percentage = (not_decrease_sum / total) * 100.0;

        // Determine sentiment
        let sentiment = if decrease_percentage > not_decrease_percentage {
            "decrease".to_string()
        } else {
            "not_decrease".to_string()
        };

        Ok(InterestRateSentiment {
            sentiment,
            decrease_percentage,
            not_decrease_percentage,
        })
    }

    fn make_get_request(&self, url: &str) -> Result<HttpResponse, String> {
        // Prepare headers for GET request
        let mut headers = Vec::new();
        headers.push(("User-Agent".to_string(), "newton-api-agent/1.0".to_string()));
        headers.push(("Accept".to_string(), "application/json".to_string()));

        // Create the HTTP GET request
        let request = HttpRequest {
            url: url.to_string(),
            method: "GET".to_string(),
            headers,
            body: None,
        };

        // Make the request using the generated fetch function
        fetch(&request)
    }
}

// Public function to get interest rate sentiment from Polymarket
pub fn get_fed_interest_rate_sentiment() -> Result<String, String> {
    let agent = ApiAgent::new();
    agent.get_interest_rate_sentiment()
}