use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/get").await?;
    let body = response.text().await?;
    println!("Response: {}", body);

    let contents = fs::read_to_string("../../sample_intent.json")?;
    let json: serde_json::Value = serde_json::from_str(&contents)?;    
    println!("{}", serde_json::to_string_pretty(&json)?);
        
    Ok(())
}
