use iced::{
    border,
    widget::container,
    Color, Element, Length, Theme,
};

use crate::widgets::frame;

pub struct CardUI;

impl CardUI {
    pub fn wrap<'a, Message: 'a>(
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message>
    where
        Element<'a, Message>: From<iced::widget::Container<'a, Message>>,
    {
        container(content.into())
            .padding(20)
            .width(420)
            .style(frame::style)
            .into()
    }

    pub fn wrap_full<'a, Message: 'a>(
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message>
    where
        Element<'a, Message>: From<iced::widget::Container<'a, Message>>,
    {
        container(content.into())
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(frame::style)
            .into()
    }
}

pub fn card_row_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Color::from_rgb(0.14, 0.14, 0.16).into()),
        border: border::rounded(10)
            .width(1)
            .color(Color::from_rgb(0.22, 0.22, 0.26)),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}
