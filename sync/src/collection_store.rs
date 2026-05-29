use heart::collection::Collection;
use std::path::PathBuf;

fn collection_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(format!("{}/.local/share/boxbreaker/collection.json", home))
}

pub async fn load() -> Collection {
    let path = collection_path();
    let data = tokio::fs::read_to_string(path).await.unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub async fn save(collection: &Collection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = collection_path();
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let data = serde_json::to_string_pretty(collection)?;
    tokio::fs::write(path, data).await?;
    Ok(())
}
