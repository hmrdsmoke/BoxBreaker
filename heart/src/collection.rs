use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Condition {
    NearMint,
    LightlyPlayed,
    ModeratelyPlayed,
    HeavilyPlayed,
    Poor,
}

impl Condition {
    pub fn label(&self) -> &str {
        match self {
            Condition::NearMint => "Near Mint",
            Condition::LightlyPlayed => "Lightly Played",
            Condition::ModeratelyPlayed => "Moderately Played",
            Condition::HeavilyPlayed => "Heavily Played",
            Condition::Poor => "Poor",
        }
    }

    pub fn all() -> &'static [&'static str] {
        &["Near Mint", "Lightly Played", "Moderately Played", "Heavily Played", "Poor"]
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Lightly Played" => Condition::LightlyPlayed,
            "Moderately Played" => Condition::ModeratelyPlayed,
            "Heavily Played" => Condition::HeavilyPlayed,
            "Poor" => Condition::Poor,
            _ => Condition::NearMint,
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Condition::NearMint => 1.0,
            Condition::LightlyPlayed => 0.85,
            Condition::ModeratelyPlayed => 0.70,
            Condition::HeavilyPlayed => 0.50,
            Condition::Poor => 0.25,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionEntry {
    pub card_id: String,
    pub card_name: String,
    pub set_name: String,
    pub game: String,
    pub quantity: u32,
    pub condition: Condition,
    pub image_url: Option<String>,
    pub tcgplayer_low: Option<f32>,
    pub tcgplayer_mid: Option<f32>,
    pub tcgplayer_high: Option<f32>,
    pub cardmarket_low: Option<f32>,
    pub cardmarket_mid: Option<f32>,
    pub cardmarket_high: Option<f32>,
}

impl CollectionEntry {
    pub fn avg_value(&self) -> Option<f32> {
        let points: Vec<f32> = [
            self.tcgplayer_low,
            self.tcgplayer_mid,
            self.tcgplayer_high,
            self.cardmarket_low,
            self.cardmarket_mid,
            self.cardmarket_high,
        ]
        .iter()
        .flatten()
        .cloned()
        .collect();

        if points.is_empty() {
            return None;
        }

        let avg = points.iter().sum::<f32>() / points.len() as f32;
        Some((avg * self.condition.multiplier() * 100.0).round() / 100.0)
    }

    pub fn total_value(&self) -> Option<f32> {
        self.avg_value().map(|v| (v * self.quantity as f32 * 100.0).round() / 100.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Collection {
    pub entries: Vec<CollectionEntry>,
}

impl Collection {
    pub fn total_value(&self) -> f32 {
        self.entries.iter()
            .filter_map(|e| e.total_value())
            .sum::<f32>()
    }

    pub fn add_or_increment(&mut self, entry: CollectionEntry) {
        if let Some(existing) = self.entries.iter_mut().find(|e| {
            e.card_id == entry.card_id && e.condition == entry.condition
        }) {
            existing.quantity += 1;
            existing.tcgplayer_low = entry.tcgplayer_low;
            existing.tcgplayer_mid = entry.tcgplayer_mid;
            existing.tcgplayer_high = entry.tcgplayer_high;
            existing.cardmarket_low = entry.cardmarket_low;
            existing.cardmarket_mid = entry.cardmarket_mid;
            existing.cardmarket_high = entry.cardmarket_high;
        } else {
            self.entries.push(entry);
        }
    }

    pub fn remove_one(&mut self, card_id: &str, condition: &Condition) {
        if let Some(pos) = self.entries.iter().position(|e| {
            e.card_id == card_id && &e.condition == condition
        }) {
            if self.entries[pos].quantity <= 1 {
                self.entries.remove(pos);
            } else {
                self.entries[pos].quantity -= 1;
            }
        }
    }
}
