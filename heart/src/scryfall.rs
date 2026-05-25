use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::card::{Card, CardGame, Rarity};
use crate::price::PriceEntry;

#[derive(Debug, Deserialize)]
struct BulkDataList {
    data: Vec<BulkDataEntry>,
}

#[derive(Debug, Deserialize)]
struct BulkDataEntry {
    #[serde(rename = "type")]
    data_type: String,
    download_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct ScryfallCard {
    pub name: String,
    pub set: String,
    pub set_name: String,
    pub rarity: String,
    pub collector_number: String,
    #[serde(default)]
    pub type_line: Option<String>,
    #[serde(default)]
    pub oracle_text: Option<String>,
    #[serde(default)]
    pub prices: Option<ScryfallPrices>,
    #[serde(default)]
    pub image_uris: Option<ScryfallImageUris>,
}

#[derive(Debug, Deserialize)]
pub struct ScryfallPrices {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub eur: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScryfallImageUris {
    pub small: Option<String>,
    pub normal: Option<String>,
    pub large: Option<String>,
}

fn map_scryfall_rarity(rarity: &str) -> Rarity {
    match rarity {
        "common" => Rarity::Common,
        "uncommon" => Rarity::Uncommon,
        "rare" => Rarity::Rare,
        "mythic" => Rarity::Mythic,
        "special" | "bonus" => Rarity::Special,
        _ => Rarity::Common,
    }
}

pub async fn fetch_bulk_download_url(client: &reqwest::Client) -> Result<String> {
    let resp: BulkDataList = client
        .get("https://api.scryfall.com/bulk-data")
        .header("User-Agent", "BoxBreaker/0.1.0")
        .send()
        .await?
        .json()
        .await?;

    let oracle = resp
        .data
        .iter()
        .find(|e| e.data_type == "oracle_cards")
        .context("No oracle_cards bulk data found")?;

    Ok(oracle.download_uri.clone())
}

pub async fn download_bulk_file(
    client: &reqwest::Client,
    url: &str,
    dest: &Path,
) -> Result<()> {
    eprintln!("Downloading Scryfall bulk data...");
    let resp = client
        .get(url)
        .header("User-Agent", "BoxBreaker/0.1.0")
        .send()
        .await?;

    let total = resp.content_length().unwrap_or(0);
    let mut file = std::fs::File::create(dest)?;
    let bytes = resp.bytes().await?;

    if total > 0 {
        eprintln!(
            "Downloaded {:.1} MB",
            bytes.len() as f64 / 1_048_576.0
        );
    }

    file.write_all(&bytes)?;
    Ok(())
}

pub fn parse_bulk_file(path: &Path) -> Result<Vec<ScryfallCard>> {
    eprintln!("Parsing Scryfall bulk data...");
    let data = std::fs::read_to_string(path)?;
    let cards: Vec<ScryfallCard> = serde_json::from_str(&data)?;
    eprintln!("Parsed {} MTG cards from Scryfall", cards.len());
    Ok(cards)
}

pub fn convert_scryfall_cards(scryfall_cards: &[ScryfallCard]) -> (Vec<Card>, Vec<PriceEntry>) {
    let mut cards = Vec::with_capacity(scryfall_cards.len());
    let mut prices = Vec::new();
    let now = chrono::Utc::now();

    for sc in scryfall_cards {
        let mut card = Card::new(
            &sc.name,
            CardGame::MagicTheGathering,
            &sc.set_name,
            &sc.set,
            &sc.collector_number,
            map_scryfall_rarity(&sc.rarity),
            sc.type_line.as_deref().unwrap_or("Unknown"),
            sc.oracle_text.as_deref().unwrap_or(""),
        );

        if let Some(imgs) = &sc.image_uris {
            card.image_url = imgs.normal.clone().or_else(|| imgs.small.clone());
        }

        if let Some(ref p) = sc.prices
            && let Some(ref usd) = p.usd
            && let Ok(dollars) = usd.parse::<f64>()
        {
            let cents = (dollars * 100.0).round() as u64;
            prices.push(PriceEntry {
                card_id: card.id,
                condition: crate::card::Condition::NearMint,
                price_cents: cents,
                source: "Scryfall/TCGPlayer".to_string(),
                recorded_at: now,
            });
        }

        cards.push(card);
    }

    (cards, prices)
}
