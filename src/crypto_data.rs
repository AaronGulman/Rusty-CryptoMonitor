pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let request = client.request(reqwest::Method::GET, "https://api.exchange.coinbase.com/currencies")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;
    let parsed :serde_json::value::Value = serde_json::from_str(&body)?;

    
    println!("{:?}", parsed);
    // println!("{:?}", body);

    Ok(())
}