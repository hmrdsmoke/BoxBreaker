use iced::{
    widget::container,
    Element,
    Length,
};

use crate::widgets::frame;

pub struct CardUI;

impl CardUI {
    pub fn wrap<'a, Message: 'a>(
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        container(content.into())
            .padding(20)
            .width(420)
            .style(frame::style)
            .into()
    }

    pub fn wrap_full<'a, Message: 'a>(
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        container(content.into())
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(frame::style)
            .into()
    }
}