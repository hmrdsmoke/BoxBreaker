use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme};
use iced::mouse;
use iced::widget::{button, canvas, column, container, row, text, Canvas};
use iced::widget::canvas::{Frame, Geometry, Stroke};

use super::super::ui::{Message, UiState};
use heart::price::PriceSnapshot;

struct PriceGraph {
    snapshots: Vec<PriceSnapshot>,
}

impl canvas::Program<Message, Theme, Renderer> for PriceGraph {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        if self.snapshots.len() < 2 {
            return vec![frame.into_geometry()];
        }

        let w = bounds.width;
        let h = bounds.height;
        let pad = 24.0f32;
        let plot_w = w - 2.0 * pad;
        let plot_h = h - 2.0 * pad;

        let all_prices: Vec<f32> = self.snapshots.iter()
            .flat_map(|s| [s.tcgplayer_mid, s.cardmarket_mid].into_iter().flatten())
            .collect();

        if all_prices.is_empty() {
            return vec![frame.into_geometry()];
        }

        let min_p = all_prices.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_p = all_prices.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = (max_p - min_p).max(0.01);
        let n = self.snapshots.len();

        let to_pt = |i: usize, price: f32| -> Point {
            let x = pad + (i as f32 / (n - 1) as f32) * plot_w;
            let y = pad + (1.0 - (price - min_p) / range) * plot_h;
            Point::new(x, y)
        };

        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgb(0.08, 0.08, 0.12),
        );

        // TCGPlayer mid — blue
        let tcg: Vec<(usize, f32)> = self.snapshots.iter().enumerate()
            .filter_map(|(i, s)| s.tcgplayer_mid.map(|p| (i, p)))
            .collect();

        if tcg.len() >= 2 {
            let mut b = canvas::path::Builder::new();
            b.move_to(to_pt(tcg[0].0, tcg[0].1));
            for &(i, p) in &tcg[1..] {
                b.line_to(to_pt(i, p));
            }
            frame.stroke(
                &b.build(),
                Stroke::default()
                    .with_color(Color::from_rgb(0.3, 0.6, 1.0))
                    .with_width(2.0),
            );
        }

        // CardMarket mid — orange
        let cm: Vec<(usize, f32)> = self.snapshots.iter().enumerate()
            .filter_map(|(i, s)| s.cardmarket_mid.map(|p| (i, p)))
            .collect();

        if cm.len() >= 2 {
            let mut b = canvas::path::Builder::new();
            b.move_to(to_pt(cm[0].0, cm[0].1));
            for &(i, p) in &cm[1..] {
                b.line_to(to_pt(i, p));
            }
            frame.stroke(
                &b.build(),
                Stroke::default()
                    .with_color(Color::from_rgb(1.0, 0.65, 0.1))
                    .with_width(2.0),
            );
        }

        vec![frame.into_geometry()]
    }
}

pub fn view(state: &UiState) -> Element<'_, Message> {
    let tab_row = row![
        button(text("Card")).on_press(Message::TabSelected(0)),
        button(text("History")).on_press(Message::TabSelected(1)),
        button(text("Collection")).on_press(Message::TabSelected(2)),
        button(text("Scanner")).on_press(Message::TabSelected(3)),
    ]
    .spacing(8)
    .padding(10);

    let body: Element<Message> = if state.history_loading {
        text("Loading price history...").into()
    } else if let Some(history) = &state.selected_card_history {
        if history.snapshots.is_empty() {
            column![
                text("No price history yet.").size(14),
                text("History builds up each time prices are fetched fresh.").size(12),
            ]
            .spacing(6)
            .padding(20)
            .into()
        } else {
            let graph = Canvas::new(PriceGraph { snapshots: history.snapshots.clone() })
                .width(Length::Fill)
                .height(Length::Fixed(240.0));

            let latest = history.snapshots.last().unwrap();
            let tcg_str = latest.tcgplayer_mid
                .map(|v| format!("${:.2}", v))
                .unwrap_or_else(|| "N/A".to_string());
            let cm_str = latest.cardmarket_mid
                .map(|v| format!("${:.2}", v))
                .unwrap_or_else(|| "N/A".to_string());

            column![
                graph,
                row![
                    text(format!("TCGPlayer mid: {}", tcg_str)).size(13),
                    text(format!("CardMarket mid: {}", cm_str)).size(13),
                ]
                .spacing(20)
                .padding([4, 20]),
                text(format!("{} recorded price point(s)", history.snapshots.len())).size(11),
            ]
            .spacing(6)
            .padding(10)
            .into()
        }
    } else if state.selected_card_id.is_some() {
        container(text("No price history for this card yet."))
            .padding(20)
            .into()
    } else {
        container(text("Select a card to view price history."))
            .padding(20)
            .into()
    };

    column![
        tab_row,
        container(body)
            .width(Length::Fill)
            .height(Length::Fill),
    ]
    .into()
}
