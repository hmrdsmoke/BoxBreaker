use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::hash::{phash_bytes, similarity};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HashIndex {
    /// card_id -> (phash, card_name, game)
    pub entries: HashMap<String, HashEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashEntry {
    pub hash: u64,
    pub card_name: String,
    pub game: String,
    pub set_name: String,
    pub image_url: String,
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub card_id: String,
    pub card_name: String,
    pub game: String,
    pub set_name: String,
    pub similarity: f32,
}

fn index_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(format!("{}/.local/share/boxbreaker/hash_index.json", home))
}

pub async fn load() -> HashIndex {
    let path = index_path();
    let data = tokio::fs::read_to_string(path).await.unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub async fn save(index: &HashIndex) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = index_path();
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let data = serde_json::to_string(index)?;
    tokio::fs::write(path, data).await?;
    Ok(())
}

/// Add a card to the index from its image bytes — call this lazily when a card is viewed
pub async fn index_card(
    card_id: &str,
    card_name: &str,
    game: &str,
    set_name: &str,
    image_url: &str,
    image_bytes: &[u8],
    index: &mut HashIndex,
) {
    if index.entries.contains_key(card_id) {
        return; // already indexed
    }

    if let Some(hash) = phash_bytes(image_bytes) {
        index.entries.insert(card_id.to_string(), HashEntry {
            hash,
            card_name: card_name.to_string(),
            game: game.to_string(),
            set_name: set_name.to_string(),
            image_url: image_url.to_string(),
        });
    }
}

/// Find top N matches for a query image
pub fn find_matches(query_bytes: &[u8], index: &HashIndex, top_n: usize) -> Vec<MatchResult> {
    let query_hash = match phash_bytes(query_bytes) {
        Some(h) => h,
        None => return vec![],
    };

    let mut results: Vec<MatchResult> = index.entries.iter()
        .map(|(id, entry)| MatchResult {
            card_id: id.clone(),
            card_name: entry.card_name.clone(),
            game: entry.game.clone(),
            set_name: entry.set_name.clone(),
            similarity: similarity(query_hash, entry.hash),
        })
        .collect();

    results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
    results.truncate(top_n);
    results
}
