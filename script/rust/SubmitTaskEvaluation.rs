use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/get").await?;
    let body = response.text().await?;
    println!("Response: {}", body);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <intent_json_file_path>", args[0]);
        std::process::exit(1);
    }

    let intent_json_file_path = &args[1];

    let contents = fs::read_to_string(intent_json_file_path)?;
    let json: serde_json::Value = serde_json::from_str(&contents)?;    
    println!("{}", serde_json::to_string_pretty(&json)?);
        
    Ok(())
}
