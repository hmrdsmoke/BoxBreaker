use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub game: String,
    pub set_name: Option<String>,
    pub set_code: Option<String>,
    pub card_number: Option<String>,
    pub rarity: Option<String>,
    pub card_type: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

impl Card {
    /// Returns the PokémonTCG API ID format e.g. "hgss4-1"
    pub fn ptcg_id(&self) -> Option<String> {
        match (self.set_code.as_deref(), self.card_number.as_deref()) {
            (Some(set), Some(num)) => Some(format!("{}-{}", set, num)),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CardDatabase {
    pub cards: Vec<Card>,
}