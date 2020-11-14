use iced::{Background, Color};

#[derive(Debug, Clone, Copy)]
pub struct SharmatStyleSheet {
    pub text_color: Color,
    pub background: Background,
}

impl Default for SharmatStyleSheet {
    fn default() -> Self {
        SharmatStyleSheet {
            text_color: Color::BLACK,
            background: Background::Color(Color::from_rgb8(237, 246, 249)),
        }
    }
}

impl iced::widget::container::StyleSheet for SharmatStyleSheet {
    fn style(&self) -> iced::widget::container::Style {
        iced::widget::container::Style {
            text_color: Some(self.text_color),
            background: Some(self.background),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        }
    }
}
