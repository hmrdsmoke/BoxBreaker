use crate::card::Card;

pub fn search<'a>(cards: &'a [Card], query: &str, game: Option<&str>) -> Vec<&'a Card> {
    if query.is_empty() && matches!(game, None | Some("All")) {
        return vec![];
    }

    let query_lower = query.to_lowercase();

    let game_match = |card: &&Card| match game {
        None | Some("All") => true,
        Some("Pokemon") => card.game == "Pokemon",
        Some("Magic") => card.game == "MagicTheGathering",
        Some(g) => card.game == *g,
    };

    // Top 10 prefix matches
    let prefix: Vec<&Card> = cards.iter()
        .filter(|c| game_match(c) && c.name.to_lowercase().starts_with(&query_lower))
        .take(10)
        .collect();

    // Up to 40 fuzzy matches (contains, not already in prefix)
    let prefix_ids: std::collections::HashSet<&str> = prefix.iter().map(|c| c.id.as_str()).collect();

    let fuzzy: Vec<&Card> = cards.iter()
        .filter(|c| {
            game_match(c)
            && !prefix_ids.contains(c.id.as_str())
            && c.name.to_lowercase().contains(&query_lower)
        })
        .take(40)
        .collect();

    prefix.into_iter().chain(fuzzy).collect()
}