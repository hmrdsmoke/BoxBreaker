// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from ChatGPT.
// Do not remove these comments.

use iced::{Element, Length, Task};
use iced::widget::{row, column, container};

use heart::card::Card;
use heart::price::CardPrice;

use crate::widgets::search_ui;
use crate::widgets::menu_ui;
use crate::widgets::grid_ui;
use crate::widgets::cardart_ui;

#[derive(Default)]
pub struct UiState {
    pub cards: Vec<Card>,
    pub selected_game: Option<String>,
    pub selected_card_id: Option<String>,
    pub selected_card_price: Option<CardPrice>,
    pub selected_card_image: Option<Vec<u8>>,
    pub price_loading: bool,
    pub image_loading: bool,
    pub search_query: String,
    pub loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    GameSelected(String),
    CardSelected(String),
    CardsLoaded(Result<Vec<Card>, String>),
    PricesLoaded(Result<CardPrice, String>),
    ImageLoaded(Result<Vec<u8>, String>),
}

pub fn view(state: &UiState) -> Element<'_, Message> {
    let left_panel = column![
        search_ui::view(&state.search_query),
        menu_ui::view(state),
        grid_ui::view(state),
    ]
    .width(Length::FillPortion(2))
    .spacing(10);

    let right_panel = container(cardart_ui::view(state))
        .width(Length::FillPortion(3));

    row![left_panel, right_panel]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn update(state: &mut UiState, message: Message) -> Task<Message> {
    match message {
        Message::SearchChanged(v) => {
            state.search_query = v;
            Task::none()
        }
        Message::GameSelected(v) => {
            state.selected_game = Some(v);
            Task::none()
        }
        Message::CardSelected(id) => {
    let card = state.cards.iter().find(|c| c.id == id).cloned();
    state.selected_card_id = Some(id);
    state.selected_card_price = None;
    state.selected_card_image = None;
    state.price_loading = true;
    state.image_loading = true;

    if let Some(card) = card {
        let card2 = card.clone();
        let price_task = Task::perform(
            async move {
                // Use PokémonTCG API ID format for Pokemon, internal ID for Magic
                let lookup_id = if card.game == "Pokemon" {
                    card.ptcg_id().unwrap_or_else(|| card.id.clone())
                } else {
                    card.id.clone()
                };
                Ok(sync::pricer::fetch_prices(&lookup_id, &card.name, &card.game).await)
            },
            Message::PricesLoaded,
        );
        let image_task = Task::perform(
            async move {
                let url = card2.image_url.as_deref().unwrap_or("").to_string();
                if url.is_empty() {
                    return Err("No image URL".to_string());
                }
                sync::image::fetch_image(&url).await.map_err(|e| e.to_string())
            },
            Message::ImageLoaded,
        );
        Task::batch([price_task, image_task])
    } else {
        Task::none()
    }
}
        Message::CardsLoaded(Ok(cards)) => {
            state.cards = cards;
            state.loading = false;
            Task::none()
        }
        Message::CardsLoaded(Err(e)) => {
            state.error = Some(e);
            state.loading = false;
            Task::none()
        }
        Message::PricesLoaded(Ok(price)) => {
            state.selected_card_price = Some(price);
            state.price_loading = false;
            Task::none()
        }
        Message::PricesLoaded(Err(e)) => {
            state.error = Some(e);
            state.price_loading = false;
            Task::none()
        }
        Message::ImageLoaded(Ok(bytes)) => {
            state.selected_card_image = Some(bytes);
            state.image_loading = false;
            Task::none()
        }
        Message::ImageLoaded(Err(_)) => {
            state.image_loading = false;
            Task::none()
        }
    }
}

pub fn init() -> (UiState, Task<Message>) {
    let state = UiState { loading: true, ..Default::default() };
    let task = Task::perform(
        async {
            let path = sync::loader::default_path();
            sync::loader::load_cards(&path)
                .await
                .map_err(|e| e.to_string())
        },
        Message::CardsLoaded,
    );
    (state, task)
}

pub fn run() {
    iced::application(init, update, view)
        .title(|_state: &UiState| "BoxBreaker".to_string())
        .run()
        .unwrap();
}