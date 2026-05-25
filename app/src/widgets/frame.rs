use iced::{
    border,
    widget::container,
    Color,
    Theme,
};

pub fn style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(
            Color::from_rgb(0.10, 0.10, 0.12).into()
        ),

        border: border::rounded(18)
            .width(2)
            .color(Color::from_rgb(0.25, 0.25, 0.30)),

        text_color: Some(Color::WHITE),

        shadow: iced::Shadow {
            color: Color::BLACK.scale_alpha(0.6),
            offset: iced::Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },

        ..Default::default()
    }
}