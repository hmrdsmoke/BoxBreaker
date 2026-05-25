mod card_ui;
mod widgets;

use std::path::PathBuf;

use heart::database::CardDatabase;
use heart::seed::build_seed_database;

use iced::{
    widget::{column, container, row, scrollable, space, text, text_input, Column},
    Element, Length, Task,
};

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

struct App {
    search: String,
    database: CardDatabase,
}

fn load_database() -> CardDatabase {
    let db_path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("boxbreaker")
        .join("cards.json");

    if db_path.exists() {
        match CardDatabase::load_from_file(&db_path) {
            Ok(db) => {
                eprintln!("Loaded {} cards from {}", db.card_count(), db_path.display());
                return db;
            }
            Err(e) => {
                eprintln!("Failed to load database: {e}, using seed data");
            }
        }
    } else {
        eprintln!(
            "No synced database found at {}. Run `cargo run -p sync` to fetch all cards. Using seed data.",
            db_path.display()
        );
    }

    build_seed_database()
}

impl Default for App {
    fn default() -> Self {
        Self {
            search: String::new(),
            database: load_database(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(value) => {
                self.search = value;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message, iced::Theme> {
        let results = self.database.search(&self.search);

        let header = column![
            text("BOXBREAKER").size(40),
            text_input("Search cards...", &self.search)
                .on_input(Message::SearchChanged)
                .padding(12),
        ]
        .spacing(12);

        let result_count = text(format!("{} cards found", results.len())).size(14);

        let card_list: Element<'_, Message, iced::Theme> = if results.is_empty() {
            container(text("No cards match your search.").size(16))
                .padding(20)
                .into()
        } else {
            let cards: Vec<Element<'_, Message, iced::Theme>> = results
                .iter()
                .map(|card| {
                    let price_display =
                        if let Some(price) = self.database.price_index.latest_price(&card.id) {
                            format!("${:.2}", price.price_dollars())
                        } else {
                            "N/A".to_string()
                        };

                    let name_and_price = row![
                        text(&card.name).size(18),
                        space::horizontal(),
                        text(price_display).size(18),
                    ]
                    .spacing(8);

                    let details = row![
                        text(card.game.to_string()).size(12),
                        text(" · ").size(12),
                        text(&card.set_name).size(12),
                        text(" · ").size(12),
                        text(card.rarity.to_string()).size(12),
                    ]
                    .spacing(0);

                    let card_type_line = text(&card.card_type).size(11);

                    container(
                        column![name_and_price, details, card_type_line]
                            .spacing(4)
                            .width(Length::Fill),
                    )
                    .padding(12)
                    .width(Length::Fill)
                    .style(card_ui::card_row_style)
                    .into()
                })
                .collect();

            scrollable(Column::with_children(cards).spacing(8))
                .height(Length::Fill)
                .into()
        };

        let content: Element<'_, Message, iced::Theme> =
            column![header, result_count, card_list]
                .spacing(16)
                .width(Length::Fill)
                .height(Length::Fill)
                .into();

        card_ui::CardUI::wrap_full(content)
    }
}
