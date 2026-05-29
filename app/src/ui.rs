// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from ChatGPT.
// Do not remove these comments.

use iced::{Element, Length, Task};
use iced::widget::{row, column, container};

use heart::card::Card;
use heart::price::{CardPrice, PriceHistory};
use heart::collection::{Collection, CollectionEntry, Condition};

use crate::widgets::search_ui;
use crate::widgets::menu_ui;
use crate::widgets::grid_ui;
use crate::widgets::cardart_ui;
use crate::widgets::graph_ui;
use crate::widgets::collection_ui;
use crate::widgets::scan_ui;

#[derive(Default, PartialEq, Clone)]
pub enum ActiveTab {
    #[default]
    Card,
    History,
    Collection,
    Scanner,
}

#[derive(Default)]
pub struct UiState {
    pub cards: Vec<Card>,
    pub selected_game: Option<String>,
    pub selected_card_id: Option<String>,
    pub selected_card_price: Option<CardPrice>,
    pub selected_card_image: Option<Vec<u8>>,
    pub selected_card_history: Option<PriceHistory>,
    pub price_loading: bool,
    pub image_loading: bool,
    pub history_loading: bool,
    pub search_query: String,
    pub loading: bool,
    pub error: Option<String>,
    pub active_tab: ActiveTab,
    pub collection: Collection,
    pub collection_loading: bool,
    pub hash_index: eyes::index::HashIndex,
    pub scan_result: Option<eyes::scanner::ScanResult>,
    pub scan_image_bytes: Option<Vec<u8>>,
    pub scanning: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    GameSelected(String),
    CardSelected(String),
    CardsLoaded(Result<Vec<Card>, String>),
    PricesLoaded(Result<CardPrice, String>),
    ImageLoaded(Result<Vec<u8>, String>),
    HistoryLoaded(Result<Option<PriceHistory>, String>),
    CollectionLoaded(Collection),
    AddToCollection,
    RemoveFromCollection(String, Condition),
    CollectionSaved,
    TabSelected(u8),
    IndexCard,
    HashIndexUpdated(eyes::index::HashIndex),
    ScanFileSelected,
    ScanImageLoaded(Result<Vec<u8>, String>),
    ScanComplete(eyes::scanner::ScanResult),
    WebcamCapture,
}

pub fn view(state: &UiState) -> Element<'_, Message> {
    let left_panel = column![
        search_ui::view(&state.search_query),
        menu_ui::view(state),
        grid_ui::view(state),
    ]
    .width(Length::FillPortion(2))
    .spacing(10);

    let right_panel = container(
        match state.active_tab {
            ActiveTab::Card => cardart_ui::view(state),
            ActiveTab::History => graph_ui::view(state),
            ActiveTab::Collection => collection_ui::view(state),
            ActiveTab::Scanner => scan_ui::view(state),
        }
    )
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
            state.selected_card_id = Some(id.clone());
            state.selected_card_price = None;
            state.selected_card_image = None;
            state.selected_card_history = None;
            state.price_loading = true;
            state.image_loading = true;
            state.history_loading = true;

            if let Some(card) = card {
                let card2 = card.clone();
                let lookup_id = if card.game == "Pokemon" {
                    card.ptcg_id().unwrap_or_else(|| card.id.clone())
                } else {
                    card.id.clone()
                };
                let lookup_id2 = lookup_id.clone();

                let price_task = Task::perform(
                    async move {
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
                let history_task = Task::perform(
                    async move {
                        Ok(sync::cache::load_history(&lookup_id2).await)
                    },
                    Message::HistoryLoaded,
                );

                Task::batch([price_task, image_task, history_task])
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
            Task::done(Message::IndexCard)
        }
        Message::ImageLoaded(Err(_)) => {
            state.image_loading = false;
            Task::none()
        }
        Message::HistoryLoaded(Ok(history)) => {
            state.selected_card_history = history;
            state.history_loading = false;
            Task::none()
        }
        Message::HistoryLoaded(Err(_)) => {
            state.history_loading = false;
            Task::none()
        }
        Message::CollectionLoaded(collection) => {
            state.collection = collection;
            state.collection_loading = false;
            Task::none()
        }
        Message::AddToCollection => {
            if let Some(id) = &state.selected_card_id {
                let card = state.cards.iter().find(|c| c.id == *id).cloned();
                if let Some(card) = card {
                    let price = state.selected_card_price.clone();
                    let entry = CollectionEntry {
                        card_id: id.clone(),
                        card_name: card.name.clone(),
                        set_name: card.set_name.clone().unwrap_or_default(),
                        game: card.game.clone(),
                        quantity: 1,
                        condition: Condition::NearMint,
                        image_url: card.image_url.clone(),
                        tcgplayer_low: price.as_ref().and_then(|p| p.tcgplayer.as_ref()?.low),
                        tcgplayer_mid: price.as_ref().and_then(|p| p.tcgplayer.as_ref()?.mid),
                        tcgplayer_high: price.as_ref().and_then(|p| p.tcgplayer.as_ref()?.high),
                        cardmarket_low: price.as_ref().and_then(|p| p.cardmarket.as_ref()?.low),
                        cardmarket_mid: price.as_ref().and_then(|p| p.cardmarket.as_ref()?.mid),
                        cardmarket_high: price.as_ref().and_then(|p| p.cardmarket.as_ref()?.high),
                    };
                    state.collection.add_or_increment(entry);
                    let collection = state.collection.clone();
                    return Task::perform(
                        async move {
                            let _ = sync::collection_store::save(&collection).await;
                        },
                        |_| Message::CollectionSaved,
                    );
                }
            }
            Task::none()
        }
        Message::RemoveFromCollection(card_id, condition) => {
            state.collection.remove_one(&card_id, &condition);
            let collection = state.collection.clone();
            Task::perform(
                async move {
                    let _ = sync::collection_store::save(&collection).await;
                },
                |_| Message::CollectionSaved,
            )
        }
        Message::CollectionSaved => Task::none(),
        Message::TabSelected(0) => {
            state.active_tab = ActiveTab::Card;
            Task::none()
        }
        Message::TabSelected(1) => {
            state.active_tab = ActiveTab::History;
            Task::none()
        }
        Message::TabSelected(2) => {
            state.active_tab = ActiveTab::Collection;
            Task::none()
        }
        Message::TabSelected(3) => {
            state.active_tab = ActiveTab::Scanner;
            Task::none()
        }
        Message::TabSelected(_) => Task::none(),
        Message::IndexCard => {
            if let (Some(id), Some(bytes)) = (
                state.selected_card_id.clone(),
                state.selected_card_image.clone(),
            ) {
                let card = state.cards.iter().find(|c| c.id == id).cloned();
                if let Some(card) = card {
                    let mut index = state.hash_index.clone();
                    return Task::perform(
                        async move {
                            eyes::index::index_card(
                                &id,
                                &card.name,
                                &card.game,
                                card.set_name.as_deref().unwrap_or(""),
                                card.image_url.as_deref().unwrap_or(""),
                                &bytes,
                                &mut index,
                            ).await;
                            let _ = eyes::index::save(&index).await;
                            index
                        },
                        Message::HashIndexUpdated,
                    );
                }
            }
            Task::none()
        }
        Message::HashIndexUpdated(index) => {
            state.hash_index = index;
            Task::none()
        }
        Message::ScanFileSelected => {
            Task::perform(
                async {
                    let file = rfd::AsyncFileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                        .pick_file()
                        .await;
                    match file {
                        Some(f) => Ok(f.read().await),
                        None => Err("No file selected".to_string()),
                    }
                },
                Message::ScanImageLoaded,
            )
        }
        Message::ScanImageLoaded(Ok(bytes)) => {
            let index = state.hash_index.clone();
            state.scan_image_bytes = Some(bytes.clone());
            state.scanning = true;
            Task::perform(
                async move { eyes::scanner::scan(&bytes, &index) },
                Message::ScanComplete,
            )
        }
        Message::ScanImageLoaded(Err(_)) => Task::none(),
        Message::ScanComplete(result) => {
            state.scan_result = Some(result);
            state.scanning = false;
            Task::none()
        }
        Message::WebcamCapture => {
            // Stub — will implement when webcam is connected
            Task::none()
        }
    }
}

pub fn init() -> (UiState, Task<Message>) {
    let state = UiState { loading: true, collection_loading: true, ..Default::default() };
    let cards_task = Task::perform(
        async {
            let path = sync::loader::default_path();
            sync::loader::load_cards(&path)
                .await
                .map_err(|e| e.to_string())
        },
        Message::CardsLoaded,
    );
    let collection_task = Task::perform(
        async { sync::collection_store::load().await },
        Message::CollectionLoaded,
    );
    let index_task = Task::perform(
        async { eyes::index::load().await },
        Message::HashIndexUpdated,
    );
    (state, Task::batch([cards_task, collection_task, index_task]))
}

pub fn run() {
    iced::application(init, update, view)
        .title(|_state: &UiState| "BoxBreaker".to_string())
        .run()
        .unwrap();
}
