use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardGame {
    Pokemon,
    MagicTheGathering,
}

impl std::fmt::Display for CardGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardGame::Pokemon => write!(f, "Pokémon"),
            CardGame::MagicTheGathering => write!(f, "Magic: The Gathering"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    HoloRare,
    UltraRare,
    SecretRare,
    Mythic,
    Special,
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rarity::Common => write!(f, "Common"),
            Rarity::Uncommon => write!(f, "Uncommon"),
            Rarity::Rare => write!(f, "Rare"),
            Rarity::HoloRare => write!(f, "Holo Rare"),
            Rarity::UltraRare => write!(f, "Ultra Rare"),
            Rarity::SecretRare => write!(f, "Secret Rare"),
            Rarity::Mythic => write!(f, "Mythic"),
            Rarity::Special => write!(f, "Special"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Condition {
    NearMint,
    LightlyPlayed,
    ModeratelyPlayed,
    HeavilyPlayed,
    Damaged,
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::NearMint => write!(f, "Near Mint"),
            Condition::LightlyPlayed => write!(f, "Lightly Played"),
            Condition::ModeratelyPlayed => write!(f, "Moderately Played"),
            Condition::HeavilyPlayed => write!(f, "Heavily Played"),
            Condition::Damaged => write!(f, "Damaged"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: Uuid,
    pub name: String,
    pub game: CardGame,
    pub set_name: String,
    pub set_code: String,
    pub card_number: String,
    pub rarity: Rarity,
    pub card_type: String,
    pub description: String,
    pub image_url: Option<String>,
}

impl Card {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl Into<String>,
        game: CardGame,
        set_name: impl Into<String>,
        set_code: impl Into<String>,
        card_number: impl Into<String>,
        rarity: Rarity,
        card_type: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            game,
            set_name: set_name.into(),
            set_code: set_code.into(),
            card_number: card_number.into(),
            rarity,
            card_type: card_type.into(),
            description: description.into(),
            image_url: None,
        }
    }
}
