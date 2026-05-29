use serde::{Deserialize, Serialize};

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