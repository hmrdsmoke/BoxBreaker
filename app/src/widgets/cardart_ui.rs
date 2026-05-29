use iced::{Element, Length};
use iced::widget::{column, text, container, scrollable, image, row};

use super::super::ui::UiState;
use super::super::ui::Message;

pub fn view(state: &UiState) -> Element<'_, Message> {
    if state.loading {
        return container(text("Loading card database..."))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
    }

    let content = if let Some(id) = &state.selected_card_id {
        let card = state.cards.iter().find(|c| c.id == *id);
        let mut info_col = column![].spacing(8).padding(20);

        if let Some(card) = card {
            info_col = info_col
                .push(text(card.name.clone()).size(22))
                .push(text(format!("Set: {}", card.set_name.as_deref().unwrap_or("Unknown"))))
                .push(text(format!("Type: {}", card.card_type.as_deref().unwrap_or("Unknown"))))
                .push(text(format!("Rarity: {}", card.rarity.as_deref().unwrap_or("Unknown"))))
                .push(text(format!("Number: {}", card.card_number.as_deref().unwrap_or("?"))));

            if let Some(desc) = &card.description {
                if !desc.is_empty() {
                    info_col = info_col.push(text(desc.clone()).size(13));
                }
            }

            info_col = info_col.push(text(" "));

            if state.price_loading {
                info_col = info_col.push(text("Fetching prices..."));
            } else if let Some(price) = &state.selected_card_price {
                info_col = info_col.push(text("── Prices ──").size(16));

                if let Some(tcg) = &price.tcgplayer {
                    info_col = info_col.push(text(format!(
                        "TCGPlayer  Low: {}  Mid: {}  High: {}",
                        fmt_price(tcg.low, &tcg.currency),
                        fmt_price(tcg.mid, &tcg.currency),
                        fmt_price(tcg.high, &tcg.currency),
                    )));
                } else {
                    info_col = info_col.push(text("TCGPlayer: no results"));
                }

                if let Some(cm) = &price.cardmarket {
                    info_col = info_col.push(text(format!(
                        "CardMarket  Low: {}  Mid: {}  High: {}",
                        fmt_price(cm.low, &cm.currency),
                        fmt_price(cm.mid, &cm.currency),
                        fmt_price(cm.high, &cm.currency),
                    )));
                } else {
                    info_col = info_col.push(text("CardMarket: no results"));
                }
            }
        }

        // Image panel
        let image_panel: Element<Message> = if state.image_loading {
            text("Loading image...").into()
        } else if let Some(bytes) = &state.selected_card_image {
            let handle = image::Handle::from_bytes(bytes.clone());
            image(handle)
                .width(Length::Fixed(250.0))
                .into()
        } else {
            text("No image").into()
        };

        row![
            image_panel,
            scrollable(info_col).height(Length::Fill),
        ]
        .spacing(20)
        .padding(10)

    } else {
        row![
            column![text("Select a card to see details.")].padding(20)
        ]
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn fmt_price(val: Option<f32>, currency: &str) -> String {
    match val {
        Some(v) => {
            let symbol = if currency == "EUR" { "€" } else { "$" };
            format!("{}{:.2}", symbol, v)
        }
        None => "N/A".to_string(),
    }
}