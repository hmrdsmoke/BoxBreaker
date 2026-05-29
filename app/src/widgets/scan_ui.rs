use iced::{Element, Length};
use iced::widget::{column, text, container, scrollable, row, button, image};

use super::super::ui::{UiState, Message};

pub fn view(state: &UiState) -> Element<'_, Message> {
    let tab_row = row![
        button(text("Card")).on_press(Message::TabSelected(0)),
        button(text("History")).on_press(Message::TabSelected(1)),
        button(text("Collection")).on_press(Message::TabSelected(2)),
        button(text("Scanner")).on_press(Message::TabSelected(3)),
    ]
    .spacing(8)
    .padding(10);

    let mut content = column![
        text("Card Scanner").size(22),
        text("Scan a card to identify it and grade its condition.").size(13),
        row![
            button(text("Open Image File")).on_press(Message::ScanFileSelected),
            button(text("Use Webcam")).on_press(Message::WebcamCapture),
        ]
        .spacing(12)
        .padding(10),
    ]
    .spacing(10)
    .padding(20);

    if let Some(bytes) = &state.scan_image_bytes {
        let handle = image::Handle::from_bytes(bytes.clone());
        content = content.push(image(handle).width(Length::Fixed(250.0)));
    }

    if state.scanning {
        content = content.push(text("Scanning..."));
    } else if let Some(result) = &state.scan_result {
        content = content.push(text("── Condition ──").size(16));
        content = content.push(text(result.condition.label()).size(18));
        content = content.push(text("── Top Matches ──").size(16));

        if result.matches.is_empty() {
            content = content.push(text(
                "No matches found — card may not be in the index yet. \
                 View more cards to build up the index."
            ));
        } else {
            for (i, m) in result.matches.iter().enumerate() {
                content = content.push(
                    row![
                        text(format!("{}.", i + 1)).width(Length::Fixed(20.0)),
                        text(m.card_name.clone()).width(Length::FillPortion(3)),
                        text(m.set_name.clone()).width(Length::FillPortion(2)),
                        text(m.game.clone()).width(Length::FillPortion(2)),
                        text(format!("{:.0}% match", m.similarity * 100.0))
                            .width(Length::FillPortion(1)),
                    ]
                    .spacing(8)
                );
            }
        }
    } else {
        content = content.push(
            text(format!(
                "Index contains {} card(s). View cards to grow it.",
                state.hash_index.entries.len()
            ))
            .size(12)
        );
    }

    column![
        tab_row,
        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill),
    ]
    .into()
}
