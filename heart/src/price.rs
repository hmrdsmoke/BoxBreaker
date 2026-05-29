use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceSnapshot {
    pub timestamp: i64,
    pub tcgplayer_low: Option<f32>,
    pub tcgplayer_mid: Option<f32>,
    pub tcgplayer_high: Option<f32>,
    pub cardmarket_low: Option<f32>,
    pub cardmarket_mid: Option<f32>,
    pub cardmarket_high: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriceHistory {
    pub card_id: String,
    pub snapshots: Vec<PriceSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardPrice {
    pub card_id: String,
    pub card_name: String,
    pub tcgplayer: Option<PriceEntry>,
    pub cardmarket: Option<PriceEntry>,
    pub ebay_sold: Option<PriceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceEntry {
    pub low: Option<f32>,
    pub mid: Option<f32>,
    pub high: Option<f32>,
    pub currency: String,
    pub source_url: String,
}