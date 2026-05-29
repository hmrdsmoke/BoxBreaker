use heart::card::{Card, CardDatabase};

pub async fn load_cards(path: &str) -> Result<Vec<Card>, Box<dyn std::error::Error + Send + Sync>> {
    let data = tokio::fs::read_to_string(path).await?;
    let db: CardDatabase = serde_json::from_str(&data)?;
    Ok(db.cards)
}

pub fn default_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.local/share/boxbreaker/cards.json", home)
}