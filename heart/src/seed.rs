use chrono::Utc;

use crate::card::{Card, CardGame, Condition, Rarity};
use crate::database::CardDatabase;
use crate::price::PriceEntry;

pub fn build_seed_database() -> CardDatabase {
    let mut db = CardDatabase::new();

    let pokemon_cards = vec![
        Card::new(
            "Charizard VMAX",
            CardGame::Pokemon,
            "Darkness Ablaze",
            "DAA",
            "020/189",
            Rarity::UltraRare,
            "VMAX Pokémon",
            "Gigantamax Charizard. G-Max Wildfire: 300 damage.",
        ),
        Card::new(
            "Pikachu V",
            CardGame::Pokemon,
            "Vivid Voltage",
            "VIV",
            "043/185",
            Rarity::UltraRare,
            "V Pokémon",
            "Thunderbolt: 200 damage. Discard all Energy from this Pokémon.",
        ),
        Card::new(
            "Mewtwo GX",
            CardGame::Pokemon,
            "Shiny Vault",
            "SV",
            "SV59/SV94",
            Rarity::SecretRare,
            "GX Pokémon",
            "Psystrike GX: 200 damage. This attack's damage isn't affected by effects on the Defending Pokémon.",
        ),
        Card::new(
            "Umbreon VMAX",
            CardGame::Pokemon,
            "Evolving Skies",
            "EVS",
            "215/203",
            Rarity::SecretRare,
            "VMAX Pokémon",
            "Dark Signal Ability. Max Darkness: 160 damage.",
        ),
        Card::new(
            "Lugia V",
            CardGame::Pokemon,
            "Silver Tempest",
            "SIT",
            "138/195",
            Rarity::UltraRare,
            "V Pokémon",
            "Aero Dive: 130 damage. Discard a Stadium in play.",
        ),
        Card::new(
            "Giratina VSTAR",
            CardGame::Pokemon,
            "Lost Origin",
            "LOR",
            "131/196",
            Rarity::UltraRare,
            "VSTAR Pokémon",
            "Lost Impact: 280 damage. Put 2 Energy attached to your Pokémon in the Lost Zone.",
        ),
        Card::new(
            "Rayquaza VMAX",
            CardGame::Pokemon,
            "Evolving Skies",
            "EVS",
            "218/203",
            Rarity::SecretRare,
            "VMAX Pokémon",
            "Max Burst: 20+ damage. Discard any amount of basic Energy. 80 more for each.",
        ),
        Card::new(
            "Mew VMAX",
            CardGame::Pokemon,
            "Fusion Strike",
            "FST",
            "268/264",
            Rarity::SecretRare,
            "VMAX Pokémon",
            "Cross Fusion Strike. Max Miracle: 130 damage.",
        ),
        Card::new(
            "Arceus VSTAR",
            CardGame::Pokemon,
            "Brilliant Stars",
            "BRS",
            "123/172",
            Rarity::UltraRare,
            "VSTAR Pokémon",
            "Trinity Nova: 200 damage. Search your deck for up to 3 Energy and attach them.",
        ),
        Card::new(
            "Palkia VSTAR",
            CardGame::Pokemon,
            "Astral Radiance",
            "ASR",
            "040/189",
            Rarity::UltraRare,
            "VSTAR Pokémon",
            "Subspace Swell: 60× damage. This attack does 60 damage for each Benched Pokémon.",
        ),
    ];

    let mtg_cards = vec![
        Card::new(
            "Black Lotus",
            CardGame::MagicTheGathering,
            "Alpha",
            "LEA",
            "232",
            Rarity::Rare,
            "Artifact",
            "Sacrifice Black Lotus: Add three mana of any one color.",
        ),
        Card::new(
            "Lightning Bolt",
            CardGame::MagicTheGathering,
            "Magic 2010",
            "M10",
            "146",
            Rarity::Common,
            "Instant",
            "Lightning Bolt deals 3 damage to any target.",
        ),
        Card::new(
            "Jace, the Mind Sculptor",
            CardGame::MagicTheGathering,
            "Worldwake",
            "WWK",
            "031",
            Rarity::Mythic,
            "Planeswalker — Jace",
            "Brainstorm, unsummon, fateseal, and mill abilities.",
        ),
        Card::new(
            "Liliana of the Veil",
            CardGame::MagicTheGathering,
            "Innistrad",
            "ISD",
            "105",
            Rarity::Mythic,
            "Planeswalker — Liliana",
            "+1: Each player discards. -2: Target player sacrifices a creature. -6: Separate permanents into two piles.",
        ),
        Card::new(
            "Ragavan, Nimble Pilferer",
            CardGame::MagicTheGathering,
            "Modern Horizons 2",
            "MH2",
            "138",
            Rarity::Mythic,
            "Legendary Creature — Monkey Pirate",
            "Whenever Ragavan deals combat damage, create a Treasure and exile the top card of defending player's library.",
        ),
        Card::new(
            "Sheoldred, the Apocalypse",
            CardGame::MagicTheGathering,
            "Dominaria United",
            "DMU",
            "107",
            Rarity::Mythic,
            "Legendary Creature — Phyrexian Praetor",
            "Whenever you draw a card, you gain 2 life. Whenever an opponent draws, they lose 2 life.",
        ),
        Card::new(
            "The One Ring",
            CardGame::MagicTheGathering,
            "Lord of the Rings: Tales of Middle-earth",
            "LTR",
            "246",
            Rarity::Mythic,
            "Legendary Artifact",
            "Indestructible. When it enters, protection from everything until your next turn. Burden counters draw cards.",
        ),
        Card::new(
            "Wrenn and Six",
            CardGame::MagicTheGathering,
            "Modern Horizons",
            "MH1",
            "217",
            Rarity::Mythic,
            "Planeswalker — Wrenn",
            "+1: Return land from graveyard. -1: Deal 1 damage to target. -7: Retrace emblem.",
        ),
        Card::new(
            "Force of Will",
            CardGame::MagicTheGathering,
            "Alliances",
            "ALL",
            "028",
            Rarity::Uncommon,
            "Instant",
            "You may pay 1 life and exile a blue card rather than pay this spell's mana cost. Counter target spell.",
        ),
        Card::new(
            "Tarmogoyf",
            CardGame::MagicTheGathering,
            "Future Sight",
            "FUT",
            "153",
            Rarity::Rare,
            "Creature — Lhurgoyf",
            "Tarmogoyf's power is equal to the number of card types among cards in all graveyards.",
        ),
    ];

    let now = Utc::now();

    // Prices in cents (USD)
    let pokemon_prices: Vec<(&str, u64)> = vec![
        ("Charizard VMAX", 27500),
        ("Pikachu V", 850),
        ("Mewtwo GX", 4200),
        ("Umbreon VMAX", 32000),
        ("Lugia V", 1200),
        ("Giratina VSTAR", 1600),
        ("Rayquaza VMAX", 18500),
        ("Mew VMAX", 5500),
        ("Arceus VSTAR", 900),
        ("Palkia VSTAR", 750),
    ];

    let mtg_prices: Vec<(&str, u64)> = vec![
        ("Black Lotus", 50000000),
        ("Lightning Bolt", 150),
        ("Jace, the Mind Sculptor", 3500),
        ("Liliana of the Veil", 2200),
        ("Ragavan, Nimble Pilferer", 6500),
        ("Sheoldred, the Apocalypse", 7500),
        ("The One Ring", 5000),
        ("Wrenn and Six", 4800),
        ("Force of Will", 8500),
        ("Tarmogoyf", 1200),
    ];

    for card in pokemon_cards {
        db.add_card(card);
    }

    for card in mtg_cards {
        db.add_card(card);
    }

    for (name, price_cents) in pokemon_prices {
        if let Some(card) = db.cards.iter().find(|c| c.name == name) {
            db.price_index.add_entry(PriceEntry {
                card_id: card.id,
                condition: Condition::NearMint,
                price_cents,
                source: "TCGPlayer".to_string(),
                recorded_at: now,
            });
        }
    }

    for (name, price_cents) in mtg_prices {
        if let Some(card) = db.cards.iter().find(|c| c.name == name) {
            db.price_index.add_entry(PriceEntry {
                card_id: card.id,
                condition: Condition::NearMint,
                price_cents,
                source: "TCGPlayer".to_string(),
                recorded_at: now,
            });
        }
    }

    db
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_database_has_cards() {
        let db = build_seed_database();
        assert_eq!(db.card_count(), 20);
    }

    #[test]
    fn seed_database_search_works() {
        let db = build_seed_database();
        let results = db.search("charizard");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Charizard VMAX");
    }

    #[test]
    fn seed_database_filter_by_game() {
        let db = build_seed_database();
        let pokemon = db.filter_by_game(CardGame::Pokemon);
        assert_eq!(pokemon.len(), 10);
        let mtg = db.filter_by_game(CardGame::MagicTheGathering);
        assert_eq!(mtg.len(), 10);
    }

    #[test]
    fn seed_database_has_prices() {
        let db = build_seed_database();
        let charizard = db.search("charizard")[0];
        let price = db.price_index.latest_price(&charizard.id);
        assert!(price.is_some());
        assert_eq!(price.unwrap().price_cents, 27500);
    }

    #[test]
    fn price_dollars_conversion() {
        let db = build_seed_database();
        let charizard = db.search("charizard")[0];
        let price = db.price_index.latest_price(&charizard.id).unwrap();
        assert!((price.price_dollars() - 275.00).abs() < f64::EPSILON);
    }

    #[test]
    fn multi_term_search() {
        let db = build_seed_database();
        let results = db.search("pikachu v");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Pikachu V");
    }

    #[test]
    fn search_by_set() {
        let db = build_seed_database();
        let results = db.search("evolving skies");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn empty_search_returns_all() {
        let db = build_seed_database();
        let results = db.search("");
        assert_eq!(results.len(), 20);
    }
}
