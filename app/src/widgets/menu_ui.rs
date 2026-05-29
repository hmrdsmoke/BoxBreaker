use iced::Element;
use iced::widget::pick_list;

use super::super::ui::{Message, UiState};

const GAME_TYPES: &[&str] = &["All", "Pokemon", "Magic"];

pub fn view(state: &UiState) -> Element<'_, Message> {
    let selected = state
        .selected_game
        .as_deref()
        .unwrap_or("All");

    pick_list(
        GAME_TYPES,
        Some(selected),
        |s: &str| Message::GameSelected(s.to_string()),
    )
    .into()
}