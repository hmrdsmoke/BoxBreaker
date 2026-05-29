pub async fn fetch_image(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::builder()
        .user_agent("BoxBreaker/0.1")
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let bytes = client.get(url).send().await?.bytes().await?;
    Ok(bytes.to_vec())
}