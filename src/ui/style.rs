use iced::{Color, Theme};
use iced::widget::{button, container, checkbox, text_input};

pub enum ContainerStyle {
    MainContainer,
    TodoItem,
    CompletedTodoItem,
}

impl container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &Theme) -> container::Appearance {
        match self {
            ContainerStyle::MainContainer => container::Appearance {
                background: Some(Color::from_rgb(0.95, 0.95, 0.95).into()),
                border_radius: 5.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.8, 0.8, 0.8),
                ..Default::default()
            },
            ContainerStyle::TodoItem => container::Appearance {
                background: Some(Color::from_rgb(1.0, 1.0, 1.0).into()),
                border_radius: 3.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.9, 0.9, 0.9),
                ..Default::default()
            },
            ContainerStyle::CompletedTodoItem => container::Appearance {
                background: Some(Color::from_rgb(0.97, 0.97, 0.97).into()),
                border_radius: 3.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.9, 0.9, 0.9),
                ..Default::default()
            },
        }
    }
}

pub enum ButtonStyle {
    Add,
    Delete,
    Filter,
    FilterActive,
}

impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        match self {
            ButtonStyle::Add => button::Appearance {
                background: Some(Color::from_rgb(0.2, 0.6, 0.2).into()),
                text_color: Color::WHITE,
                border_radius: 3.0.into(),
                border_width: 0.0,
                ..Default::default()
            },
            ButtonStyle::Delete => button::Appearance {
                background: Some(Color::from_rgb(0.8, 0.2, 0.2).into()),
                text_color: Color::WHITE,
                border_radius: 3.0.into(),
                border_width: 0.0,
                ..Default::default()
            },
            ButtonStyle::Filter => button::Appearance {
                background: Some(Color::from_rgb(0.9, 0.9, 0.9).into()),
                text_color: Color::from_rgb(0.2, 0.2, 0.2),
                border_radius: 3.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.7, 0.7, 0.7),
                ..Default::default()
            },
            ButtonStyle::FilterActive => button::Appearance {
                background: Some(Color::from_rgb(0.3, 0.5, 0.8).into()),
                text_color: Color::WHITE,
                border_radius: 3.0.into(),
                border_width: 0.0,
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 1.0),
            ..active
        }
    }
}

pub enum CheckboxStyle {
    Todo,
}

impl checkbox::StyleSheet for CheckboxStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: if is_checked {
                Color::from_rgb(0.2, 0.6, 0.2).into()
            } else {
                Color::from_rgb(1.0, 1.0, 1.0).into()
            },
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: if is_checked {
                Color::from_rgb(0.2, 0.6, 0.2)
            } else {
                Color::from_rgb(0.7, 0.7, 0.7)
            },
            icon_color: Color::WHITE,
            text_color: None,
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let active = self.active(style, is_checked);

        checkbox::Appearance {
            border_color: if is_checked {
                Color::from_rgb(0.1, 0.5, 0.1)
            } else {
                Color::from_rgb(0.5, 0.5, 0.5)
            },
            ..active
        }
    }
}

pub enum TextInputStyle {
    Default,
    Focused,
}

impl text_input::StyleSheet for TextInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        match self {
            TextInputStyle::Default => text_input::Appearance {
                background: Color::WHITE.into(),
                border_radius: 3.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.7, 0.7, 0.7),
                icon_color: Color::from_rgb(0.7, 0.7, 0.7)
            },
            TextInputStyle::Focused => text_input::Appearance {
                background: Color::WHITE.into(),
                border_radius: 3.0.into(),
                border_width: 1.0,
                border_color: Color::from_rgb(0.2, 0.6, 0.9),
                icon_color: Color::from_rgb(0.2, 0.6, 0.9)
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let active = self.active(style);

        text_input::Appearance {
            border_color: Color::from_rgb(0.3, 0.7, 1.0),
            ..active
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::WHITE.into(),
            border_radius: 3.0.into(),
            border_width: 2.0,
            border_color: Color::from_rgb(0.2, 0.6, 0.9),
            icon_color: Color::from_rgb(0.2, 0.6, 0.9)
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color::BLACK
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color::from_rgb(0.8, 0.8, 0.8)
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let mut appearance = self.active(style);
        appearance.border_color = iced::Color::from_rgb(0.5, 0.5, 0.5);
        appearance
    }
} 