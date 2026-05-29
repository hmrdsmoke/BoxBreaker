use iced::{Element, Length};
use iced::widget::{column, text, container, scrollable, row, button};

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

    let total = state.collection.total_value();
    let count: u32 = state.collection.entries.iter().map(|e| e.quantity).sum();

    let mut list = column![
        text(format!("Collection — {} cards  |  Total Value: ${:.2}", count, total)).size(18),
        text("Avg of TCGPlayer + CardMarket Low/Mid/High × condition multiplier").size(11),
    ]
    .spacing(6)
    .padding(20);

    if state.collection.entries.is_empty() {
        list = list.push(text("No cards yet — select a card and click Add to Collection."));
    } else {
        list = list.push(
            row![
                text("Card").width(Length::FillPortion(3)),
                text("Set").width(Length::FillPortion(2)),
                text("Condition").width(Length::FillPortion(2)),
                text("Qty").width(Length::FillPortion(1)),
                text("Avg Value").width(Length::FillPortion(1)),
                text("Total").width(Length::FillPortion(1)),
                text("").width(Length::FillPortion(1)),
            ]
            .spacing(8)
        );

        for entry in &state.collection.entries {
            let avg = entry.avg_value()
                .map(|v| format!("${:.2}", v))
                .unwrap_or_else(|| "N/A".to_string());
            let total_val = entry.total_value()
                .map(|v| format!("${:.2}", v))
                .unwrap_or_else(|| "N/A".to_string());

            let card_id = entry.card_id.clone();
            let condition = entry.condition.clone();

            list = list.push(
                row![
                    text(entry.card_name.clone()).width(Length::FillPortion(3)),
                    text(entry.set_name.clone()).width(Length::FillPortion(2)),
                    text(entry.condition.label()).width(Length::FillPortion(2)),
                    text(entry.quantity.to_string()).width(Length::FillPortion(1)),
                    text(avg).width(Length::FillPortion(1)),
                    text(total_val).width(Length::FillPortion(1)),
                    button(text("-"))
                        .on_press(Message::RemoveFromCollection(card_id, condition))
                        .width(Length::FillPortion(1)),
                ]
                .spacing(8)
            );
        }
    }

    column![
        tab_row,
        container(scrollable(list))
            .width(Length::Fill)
            .height(Length::Fill),
    ]
    .into()
}
