use heart::price::CardPrice;
use std::path::PathBuf;

fn cache_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(format!("{}/.local/share/boxbreaker/prices", home))
}

fn cache_path(card_id: &str) -> PathBuf {
    cache_dir().join(format!("{}.json", card_id))
}

pub async fn load(card_id: &str) -> Option<CardPrice> {
    let path = cache_path(card_id);
    let data = tokio::fs::read_to_string(path).await.ok()?;
    serde_json::from_str(&data).ok()
}

pub async fn save(price: &CardPrice) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let dir = cache_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let path = cache_path(&price.card_id);
    let data = serde_json::to_string_pretty(price)?;
    tokio::fs::write(path, data).await?;
    Ok(())
}