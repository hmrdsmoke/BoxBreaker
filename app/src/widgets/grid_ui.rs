use iced::{Element, Length};
use iced::widget::{button, column, scrollable, row, text};

use heart::search::search;
use super::super::ui::{Message, UiState};

pub fn view(state: &UiState) -> Element<'_, Message> {
    if state.loading {
        return scrollable(column![text("Loading cards...")]).into();
    }

    if let Some(err) = &state.error {
        return scrollable(column![text(format!("Error: {}", err))]).into();
    }

    let has_filter = state.selected_game.as_deref().map(|g| g != "All").unwrap_or(false);
    let has_query = !state.search_query.is_empty();

    if !has_filter && !has_query {
        return scrollable(column![text("Select a game or search to see cards.")]).into();
    }

    let game_filter = state.selected_game.as_deref();
    let results = search(&state.cards, &state.search_query, game_filter);
    let count = results.len();

    let mut col = column![];

    for card in results {
        col = col.push(
           button(
    row![
        text(card.name.clone()).width(Length::Fill),
        text(card.rarity.clone().unwrap_or_else(|| "Unknown".to_string())).size(12),
    ]
    .spacing(8)
)
            .on_press(Message::CardSelected(card.id.clone()))
            .width(Length::Fill)
        );
    }

    col = col.push(text(format!("Showing {} results", count)));

    scrollable(col)
        .height(Length::Fill)
        .into()
}