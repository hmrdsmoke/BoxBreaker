use iced::Element;
use iced::widget::text_input;

use super::super::ui::Message;

pub fn view(query: &str) -> Element<'_, Message> {
    text_input("Search cards...", query)
        .on_input(Message::SearchChanged)
        .padding(10)
        .into()
}