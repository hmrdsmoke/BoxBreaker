use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::card::Condition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceEntry {
    pub card_id: Uuid,
    pub condition: Condition,
    pub price_cents: u64,
    pub source: String,
    pub recorded_at: DateTime<Utc>,
}

impl PriceEntry {
    pub fn price_dollars(&self) -> f64 {
        self.price_cents as f64 / 100.0
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceIndex {
    entries: HashMap<Uuid, Vec<PriceEntry>>,
}

impl PriceIndex {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, entry: PriceEntry) {
        self.entries.entry(entry.card_id).or_default().push(entry);
    }

    pub fn latest_price(&self, card_id: &Uuid) -> Option<&PriceEntry> {
        self.entries
            .get(card_id)?
            .iter()
            .max_by_key(|e| e.recorded_at)
    }

    pub fn price_history(&self, card_id: &Uuid) -> &[PriceEntry] {
        self.entries
            .get(card_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn lowest_price(&self, card_id: &Uuid) -> Option<&PriceEntry> {
        self.entries
            .get(card_id)?
            .iter()
            .min_by_key(|e| e.price_cents)
    }

    pub fn highest_price(&self, card_id: &Uuid) -> Option<&PriceEntry> {
        self.entries
            .get(card_id)?
            .iter()
            .max_by_key(|e| e.price_cents)
    }

    pub fn average_price_cents(&self, card_id: &Uuid) -> Option<u64> {
        let entries = self.entries.get(card_id)?;
        if entries.is_empty() {
            return None;
        }
        let total: u64 = entries.iter().map(|e| e.price_cents).sum();
        Some(total / entries.len() as u64)
    }
}
