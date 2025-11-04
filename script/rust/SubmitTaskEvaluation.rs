#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/get").await?;
    let body = response.text().await?;
    println!("Response: {}", body);
    Ok(())
}
