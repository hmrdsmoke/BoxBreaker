mod card_ui;
mod widgets;

use iced::{
    widget::{column, text, text_input},
    Element,
    Task,
};

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

#[derive(Default)]
struct App {
    search: String,
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
}

impl App {
    fn title(&self) -> String {
        String::from("BoxBreaker Pricing")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(value) => {
                self.search = value;
            }
        }

        Task::none()
    }

fn view(&self) -> Element<'_, Message, iced::Theme> {
    let content: Element<'_, Message, iced::Theme> = column![
        text("BOXBREAKER").size(40),

        text_input("Search cards...", &self.search)
            .on_input(Message::SearchChanged)
            .padding(12),
        ]
        .spacing(20)
        .into();

        card_ui::CardUI::wrap_full(content)
    }
}