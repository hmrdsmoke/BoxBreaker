use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use crate::card::{Card, CardGame, Condition, Rarity};
use crate::price::PriceEntry;

const API_BASE: &str = "https://api.pokemontcg.io/v2";
const PAGE_SIZE: u32 = 250;

#[derive(Debug, Deserialize)]
struct PokemonTcgResponse {
    data: Vec<PokemonTcgCard>,
    #[serde(default, rename = "totalCount")]
    total_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonTcgCard {
    pub id: String,
    pub name: String,
    pub supertype: Option<String>,
    #[serde(default)]
    pub subtypes: Vec<String>,
    pub set: PokemonTcgSet,
    pub number: String,
    pub rarity: Option<String>,
    #[serde(default)]
    pub images: Option<PokemonTcgImages>,
    #[serde(default)]
    pub tcgplayer: Option<PokemonTcgPriceSource>,
    #[serde(default, rename = "flavorText")]
    pub flavor_text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonTcgSet {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PokemonTcgImages {
    pub small: Option<String>,
    pub large: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonTcgPriceSource {
    #[serde(default)]
    pub prices: Option<HashMap<String, PokemonTcgPriceData>>,
    #[serde(default, rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonTcgPriceData {
    pub market: Option<f64>,
    pub low: Option<f64>,
    pub mid: Option<f64>,
    pub high: Option<f64>,
}

fn map_pokemon_rarity(rarity: &str) -> Rarity {
    let lower = rarity.to_lowercase();
    if lower.contains("secret") || lower.contains("hyper") {
        Rarity::SecretRare
    } else if lower.contains("ultra") || lower.contains("illustration") {
        Rarity::UltraRare
    } else if lower.contains("holo") && lower.contains("rare") {
        Rarity::HoloRare
    } else if lower.contains("rare") {
        Rarity::Rare
    } else if lower.contains("uncommon") {
        Rarity::Uncommon
    } else if lower.contains("common") {
        Rarity::Common
    } else if lower.contains("promo") || lower.contains("amazing") {
        Rarity::Special
    } else {
        Rarity::Common
    }
}

pub async fn fetch_all_cards(client: &reqwest::Client) -> Result<Vec<PokemonTcgCard>> {
    let mut all_cards = Vec::new();
    let mut page: u32 = 1;

    loop {
        eprintln!("Fetching Pokémon TCG page {} (pageSize={})...", page, PAGE_SIZE);

        let url = format!(
            "{}/cards?pageSize={}&page={}",
            API_BASE, PAGE_SIZE, page
        );

        let resp: PokemonTcgResponse = client
            .get(&url)
            .header("User-Agent", "BoxBreaker/0.1.0")
            .send()
            .await?
            .json()
            .await?;

        let count = resp.data.len();
        let total = resp.total_count.unwrap_or(0);
        all_cards.extend(resp.data);

        eprintln!(
            "  Got {} cards (total so far: {}/{})",
            count,
            all_cards.len(),
            total
        );

        if count < PAGE_SIZE as usize {
            break;
        }

        page += 1;

        // Rate limit: Pokémon TCG API allows ~100 req/min without key
        tokio::time::sleep(std::time::Duration::from_millis(700)).await;
    }

    eprintln!("Fetched {} total Pokémon cards", all_cards.len());
    Ok(all_cards)
}

pub fn convert_pokemon_cards(pokemon_cards: &[PokemonTcgCard]) -> (Vec<Card>, Vec<PriceEntry>) {
    let mut cards = Vec::with_capacity(pokemon_cards.len());
    let mut prices = Vec::new();
    let now = chrono::Utc::now();

    for pc in pokemon_cards {
        let card_type = {
            let super_t = pc.supertype.as_deref().unwrap_or("Unknown");
            if pc.subtypes.is_empty() {
                super_t.to_string()
            } else {
                format!("{} — {}", super_t, pc.subtypes.join(", "))
            }
        };

        let rarity = pc
            .rarity
            .as_deref()
            .map(map_pokemon_rarity)
            .unwrap_or(Rarity::Common);

        let description = pc.flavor_text.as_deref().unwrap_or("").to_string();

        let mut card = Card::new(
            &pc.name,
            CardGame::Pokemon,
            &pc.set.name,
            &pc.set.id,
            &pc.number,
            rarity,
            &card_type,
            &description,
        );

        if let Some(imgs) = &pc.images {
            card.image_url = imgs.large.clone().or_else(|| imgs.small.clone());
        }

        if let Some(tcg) = &pc.tcgplayer
            && let Some(price_map) = &tcg.prices
            && let Some(price_data) = price_map.values().next()
            && let Some(market) = price_data.market
        {
            let cents = (market * 100.0).round() as u64;
            prices.push(PriceEntry {
                card_id: card.id,
                condition: Condition::NearMint,
                price_cents: cents,
                source: "TCGPlayer".to_string(),
                recorded_at: now,
            });
        }

        cards.push(card);
    }

    (cards, prices)
}
