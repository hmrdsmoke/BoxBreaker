use heart::price::{CardPrice, PriceHistory, PriceSnapshot};
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

fn history_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(format!("{}/.local/share/boxbreaker/history", home))
}

fn history_path(card_id: &str) -> PathBuf {
    history_dir().join(format!("{}.json", card_id))
}

pub async fn load_history(card_id: &str) -> Option<PriceHistory> {
    let data = tokio::fs::read_to_string(history_path(card_id)).await.ok()?;
    serde_json::from_str(&data).ok()
}

pub async fn append_history(price: &CardPrice) {
    let mut history = load_history(&price.card_id).await.unwrap_or_else(|| PriceHistory {
        card_id: price.card_id.clone(),
        snapshots: vec![],
    });

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    history.snapshots.push(PriceSnapshot {
        timestamp,
        tcgplayer_low: price.tcgplayer.as_ref().and_then(|p| p.low),
        tcgplayer_mid: price.tcgplayer.as_ref().and_then(|p| p.mid),
        tcgplayer_high: price.tcgplayer.as_ref().and_then(|p| p.high),
        cardmarket_low: price.cardmarket.as_ref().and_then(|p| p.low),
        cardmarket_mid: price.cardmarket.as_ref().and_then(|p| p.mid),
        cardmarket_high: price.cardmarket.as_ref().and_then(|p| p.high),
    });

    let path = history_path(&price.card_id);
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    if let Ok(data) = serde_json::to_string_pretty(&history) {
        let _ = tokio::fs::write(path, data).await;
    }
}