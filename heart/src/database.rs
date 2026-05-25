use std::path::Path;

use anyhow::Result;

use crate::card::{Card, CardGame, Rarity};
use crate::price::PriceIndex;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CardDatabase {
    pub cards: Vec<Card>,
    pub price_index: PriceIndex,
}

impl CardDatabase {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            price_index: PriceIndex::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn search(&self, query: &str) -> Vec<&Card> {
        let query_lower = query.to_lowercase();
        let terms: Vec<&str> = query_lower.split_whitespace().collect();

        if terms.is_empty() {
            return self.cards.iter().collect();
        }

        self.cards
            .iter()
            .filter(|card| {
                let name_lower = card.name.to_lowercase();
                let set_lower = card.set_name.to_lowercase();
                let type_lower = card.card_type.to_lowercase();

                terms.iter().all(|term| {
                    name_lower.contains(term)
                        || set_lower.contains(term)
                        || type_lower.contains(term)
                })
            })
            .collect()
    }

    pub fn filter_by_game(&self, game: CardGame) -> Vec<&Card> {
        self.cards.iter().filter(|c| c.game == game).collect()
    }

    pub fn filter_by_rarity(&self, rarity: Rarity) -> Vec<&Card> {
        self.cards.iter().filter(|c| c.rarity == rarity).collect()
    }

    pub fn filter_by_set(&self, set_code: &str) -> Vec<&Card> {
        self.cards
            .iter()
            .filter(|c| c.set_code.eq_ignore_ascii_case(set_code))
            .collect()
    }

    pub fn card_count(&self) -> usize {
        self.cards.len()
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let db: CardDatabase = serde_json::from_str(&json)?;
        Ok(db)
    }
}
