use reqwest::Client;
use serde::Deserialize;
use heart::price::{CardPrice, PriceEntry};
use crate::cache;

fn make_client() -> Client {
    reqwest::Client::builder()
        .user_agent("BoxBreaker/0.1 contact@boxbreaker.app")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap()
}

// ── Scryfall response shapes ─────────────────────────────────────────────────

#[derive(Deserialize)]
struct ScryfallCard {
    prices: ScryfallPrices,
}

#[derive(Deserialize)]
struct ScryfallPrices {
    usd: Option<String>,
    usd_foil: Option<String>,
    eur: Option<String>,
    eur_foil: Option<String>,
}

// ── PokémonTCG response shapes ───────────────────────────────────────────────

#[derive(Deserialize)]
struct PtcgResponse {
    data: Vec<PtcgCard>,
}

#[derive(Deserialize)]
struct PtcgCard {
    tcgplayer: Option<PtcgTcgPlayer>,
    cardmarket: Option<PtcgCardmarket>,
}

#[derive(Deserialize)]
struct PtcgTcgPlayer {
    prices: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct PtcgCardmarket {
    prices: Option<PtcgCardmarketPrices>,
}

#[derive(Deserialize, Clone)]
struct PtcgCardmarketPrices {
    #[serde(rename = "lowPrice")]
    low_price: Option<f32>,
    #[serde(rename = "averageSellPrice")]
    average_sell_price: Option<f32>,
    #[serde(rename = "trendPrice")]
    trend_price: Option<f32>,
}

// ── Currency conversion ──────────────────────────────────────────────────────

async fn eur_to_usd_rate(client: &Client) -> f32 {
    let url = "https://open.er-api.com/v6/latest/EUR";
    if let Ok(resp) = client.get(url).send().await {
        if let Ok(val) = resp.json::<serde_json::Value>().await {
            if let Some(rate) = val["rates"]["USD"].as_f64() {
                return rate as f32;
            }
        }
    }
    1.08 // fallback if API fails
}

fn convert_entry(entry: PriceEntry, rate: f32) -> PriceEntry {
    PriceEntry {
        low: entry.low.map(|v| (v * rate * 100.0).round() / 100.0),
        mid: entry.mid.map(|v| (v * rate * 100.0).round() / 100.0),
        high: entry.high.map(|v| (v * rate * 100.0).round() / 100.0),
        currency: "USD".to_string(),
        source_url: entry.source_url,
    }
}

// ── Public API ───────────────────────────────────────────────────────────────

pub async fn fetch_prices(card_id: &str, card_name: &str, game: &str) -> CardPrice {
    if let Some(cached) = cache::load(card_id).await {
        return cached;
    }

    let client = make_client();

    let price = match game {
        "MagicTheGathering" => fetch_magic(&client, card_id, card_name).await,
        "Pokemon" => fetch_pokemon(&client, card_id, card_name).await,
        _ => CardPrice {
            card_id: card_id.to_string(),
            card_name: card_name.to_string(),
            ..Default::default()
        },
    };

    let _ = cache::save(&price).await;
    price
}

async fn fetch_magic(client: &Client, card_id: &str, card_name: &str) -> CardPrice {
    let url = format!(
        "https://api.scryfall.com/cards/named?exact={}",
        urlencoded(card_name)
    );

    let mut price = CardPrice {
        card_id: card_id.to_string(),
        card_name: card_name.to_string(),
        ..Default::default()
    };

    let rate = eur_to_usd_rate(client).await;

    if let Ok(resp) = client.get(&url).send().await {
        if let Ok(card) = resp.json::<ScryfallCard>().await {
            let p = &card.prices;

            price.tcgplayer = p.usd.as_ref().and_then(|v| v.parse::<f32>().ok()).map(|usd| {
                PriceEntry {
                    low: None,
                    mid: Some(usd),
                    high: p.usd_foil.as_ref().and_then(|v| v.parse::<f32>().ok()),
                    currency: "USD".to_string(),
                    source_url: format!(
                        "https://scryfall.com/cards/named?exact={}",
                        urlencoded(card_name)
                    ),
                }
            });

            // Convert EUR CardMarket prices to USD
            let cm_entry = p.eur.as_ref().and_then(|v| v.parse::<f32>().ok()).map(|eur| {
                PriceEntry {
                    low: None,
                    mid: Some(eur),
                    high: p.eur_foil.as_ref().and_then(|v| v.parse::<f32>().ok()),
                    currency: "EUR".to_string(),
                    source_url: format!(
                        "https://scryfall.com/cards/named?exact={}",
                        urlencoded(card_name)
                    ),
                }
            });

            price.cardmarket = cm_entry.map(|e| convert_entry(e, rate));
        }
    }

    price
}

async fn fetch_pokemon(client: &Client, card_id: &str, card_name: &str) -> CardPrice {
    let mut price = CardPrice {
        card_id: card_id.to_string(),
        card_name: card_name.to_string(),
        ..Default::default()
    };

    let rate = eur_to_usd_rate(client).await;

    // Try direct ID lookup first
    let url = format!("https://api.pokemontcg.io/v2/cards/{}", card_id);
    let card_opt: Option<PtcgCard> = if let Ok(resp) = client.get(&url).send().await {
        resp.json::<serde_json::Value>().await.ok()
            .and_then(|v| serde_json::from_value(v["data"].clone()).ok())
    } else {
        None
    };

    // Fall back to name search
    let card_opt = if card_opt.is_none() {
        let search_url = format!(
            "https://api.pokemontcg.io/v2/cards?q=name:\"{}\"&pageSize=1",
            card_name
        );
        if let Ok(resp) = client.get(&search_url).send().await {
            resp.json::<PtcgResponse>().await.ok()
                .and_then(|r| r.data.into_iter().next())
        } else {
            None
        }
    } else {
        card_opt
    };

    if let Some(card) = card_opt {
        // TCGPlayer prices
        if let Some(tcg) = card.tcgplayer {
            if let Some(prices_val) = tcg.prices {
                let mut lows: Vec<f32> = vec![];
                let mut mids: Vec<f32> = vec![];
                let mut highs: Vec<f32> = vec![];

                if let Some(obj) = prices_val.as_object() {
                    for variant in obj.values() {
                        if let Some(l) = variant["low"].as_f64() { lows.push(l as f32); }
                        if let Some(m) = variant["market"].as_f64() { mids.push(m as f32); }
                        if let Some(h) = variant["high"].as_f64() { highs.push(h as f32); }
                    }
                }

                if !mids.is_empty() {
                    price.tcgplayer = Some(PriceEntry {
                        low: lows.iter().cloned().reduce(f32::min),
                        mid: mids.iter().cloned().reduce(f32::min),
                        high: highs.iter().cloned().reduce(f32::max),
                        currency: "USD".to_string(),
                        source_url: format!("https://prices.pokemontcg.io/tcgplayer/{}", card_id),
                    });
                }
            }
        }

        // CardMarket prices — convert EUR to USD
        if let Some(cm) = card.cardmarket {
            if let Some(p) = cm.prices {
                let eur_entry = PriceEntry {
                    low: p.low_price,
                    mid: p.average_sell_price,
                    high: p.trend_price,
                    currency: "EUR".to_string(),
                    source_url: format!("https://prices.pokemontcg.io/cardmarket/{}", card_id),
                };
                price.cardmarket = Some(convert_entry(eur_entry, rate));
            }
        }
    }

    price
}

fn urlencoded(s: &str) -> String {
    s.replace(' ', "+")
}