use std::path::PathBuf;

use anyhow::Result;
use heart::database::CardDatabase;
use heart::pokemon_tcg;
use heart::scryfall;

fn data_dir() -> PathBuf {
    let dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("boxbreaker");
    std::fs::create_dir_all(&dir).ok();
    dir
}

#[tokio::main]
async fn main() -> Result<()> {
    let data = data_dir();
    let db_path = data.join("cards.json");
    let client = reqwest::Client::new();

    eprintln!("BoxBreaker Card Sync");
    eprintln!("====================");
    eprintln!("Data directory: {}", data.display());
    eprintln!();

    // --- Scryfall (MTG) ---
    eprintln!("=== Magic: The Gathering (Scryfall) ===");
    let bulk_path = data.join("scryfall_oracle.json");
    let url = scryfall::fetch_bulk_download_url(&client).await?;
    scryfall::download_bulk_file(&client, &url, &bulk_path).await?;
    let scryfall_cards = scryfall::parse_bulk_file(&bulk_path)?;
    let (mtg_cards, mtg_prices) = scryfall::convert_scryfall_cards(&scryfall_cards);
    eprintln!("Converted {} MTG cards with {} prices", mtg_cards.len(), mtg_prices.len());
    eprintln!();

    // --- Pokémon TCG ---
    eprintln!("=== Pokémon TCG ===");
    let pokemon_api_cards = pokemon_tcg::fetch_all_cards(&client).await?;
    let (pokemon_cards, pokemon_prices) = pokemon_tcg::convert_pokemon_cards(&pokemon_api_cards);
    eprintln!("Converted {} Pokémon cards with {} prices", pokemon_cards.len(), pokemon_prices.len());
    eprintln!();

    // --- Build database ---
    let mut db = CardDatabase::new();

    for card in mtg_cards {
        db.add_card(card);
    }
    for card in pokemon_cards {
        db.add_card(card);
    }
    for price in mtg_prices {
        db.price_index.add_entry(price);
    }
    for price in pokemon_prices {
        db.price_index.add_entry(price);
    }

    eprintln!("=== Summary ===");
    eprintln!("Total cards: {}", db.card_count());
    eprintln!("Saving database to: {}", db_path.display());

    db.save_to_file(&db_path)?;

    let file_size = std::fs::metadata(&db_path)?.len();
    eprintln!(
        "Database saved ({:.1} MB)",
        file_size as f64 / 1_048_576.0
    );
    eprintln!("Done! Run the app to search all cards.");

    Ok(())
}
