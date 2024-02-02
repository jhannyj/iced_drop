use iced::{border::Radius, widget::scrollable::Scroller, Background, Border, Color};

#[derive(Default)]
pub struct Board;

#[derive(Default)]
pub enum Application {
    #[default]
    Default,
}

impl iced::application::StyleSheet for Board {
    type Style = Application;

    fn appearance(&self, style: &Self::Style) -> iced::application::Appearance {
        match style {
            Application::Default => iced::application::Appearance {
                background_color: Color::WHITE,
                text_color: Color::BLACK,
            },
        }
    }
}

#[derive(Default, Clone)]
pub enum Text {
    #[default]
    Default,
    Title,
    ListName,
    Task,
}

impl iced::widget::text::StyleSheet for Board {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> iced::widget::text::Appearance {
        match style {
            Text::Default => iced::widget::text::Appearance::default(),
            Text::Title => iced::widget::text::Appearance {
                color: Some(Color::from_rgb8(242, 251, 224)),
            },
            Text::ListName => iced::widget::text::Appearance {
                color: Some(Color::from_rgb8(96, 91, 86)),
            },
            Text::Task => iced::widget::text::Appearance {
                color: Some(Color::from_rgb8(131, 122, 117)),
            },
        }
    }
}

#[derive(Default)]
pub enum Container {
    #[default]
    Default,
    ActiveSlot,
    Title,
    List,
    ActiveList,
    Task,
    ActiveTask,
    Background,
    TaskAdderTooltip,
}

impl iced::widget::container::StyleSheet for Board {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        match style {
            Container::Default => iced::widget::container::Appearance::default(),
            Container::ActiveSlot => iced::widget::container::Appearance {
                border: Border {
                    color: Color::from_rgb8(202, 233, 255),
                    width: 5.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            },
            Container::Title => iced::widget::container::Appearance {
                text_color: None,
                background: Some(iced::Background::Color(Color::from_rgb8(131, 122, 117))),
                ..Default::default()
            },
            Container::List => iced::widget::container::Appearance {
                text_color: None,
                background: Some(iced::Background::Color(Color::from_rgb8(172, 193, 138))),
                border: Border::with_radius(10.0),
                ..Default::default()
            },
            Container::ActiveList => iced::widget::container::Appearance {
                text_color: None,
                background: Some(iced::Background::Color(Color::from_rgb8(172, 193, 138))),
                border: Border {
                    color: Color::from_rgb8(202, 233, 255),
                    width: 5.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            },
            Container::Task => iced::widget::container::Appearance {
                text_color: None,
                background: Some(iced::Background::Color(Color::from_rgb8(218, 254, 183))),
                border: Border::with_radius(10.0),
                ..Default::default()
            },
            Container::ActiveTask => iced::widget::container::Appearance {
                text_color: None,
                background: Some(iced::Background::Color(Color::from_rgb8(218, 254, 183))),
                border: Border {
                    color: Color::from_rgb8(202, 233, 255),
                    width: 5.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            },
            Container::Background => iced::widget::container::Appearance {
                text_color: None,
                background: Some(iced::Background::Color(Color::from_rgb8(96, 91, 86))),
                ..Default::default()
            },
            Container::TaskAdderTooltip => iced::widget::container::Appearance {
                text_color: Some(Color::from_rgb8(242, 251, 224)),
                background: Some(iced::Background::Color(Color::from_rgb8(131, 122, 117))),
                border: Border::with_radius(5.0),
                ..Default::default()
            },
        }
    }
}

#[derive(Default)]
pub enum Button {
    #[default]
    Adder,
}

impl iced::widget::button::StyleSheet for Board {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        match style {
            Button::Adder => iced::widget::button::Appearance {
                background: Some(Background::Color(Color::from_rgb8(96, 91, 86))),
                text_color: Color::from_rgb8(242, 251, 224),
                border: Border::with_radius(10.0),
                ..Default::default()
            },
        }
    }
}

#[derive(Default)]
pub enum TextInput {
    #[default]
    ElementAdder,
}

impl iced::widget::text_input::StyleSheet for Board {
    type Style = TextInput;

    fn active(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        match style {
            TextInput::ElementAdder => iced::widget::text_input::Appearance {
                background: iced::Background::Color(Color::from_rgb8(218, 254, 183)),
                border: Border {
                    color: Color::from_rgb8(96, 91, 86),
                    width: 1.0,
                    radius: Radius::from(10.0),
                },
                icon_color: Color::BLACK,
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInput::ElementAdder => Color::from_rgba8(131, 122, 117, 0.7),
        }
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInput::ElementAdder => Color::from_rgb8(131, 122, 117),
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInput::ElementAdder => Color::from_rgb8(131, 122, 117),
        }
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInput::ElementAdder => Color::WHITE,
        }
    }

    fn disabled(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        self.active(style)
    }
}

#[derive(Default)]
pub enum Scrollable {
    #[default]
    Default,
}

impl iced::widget::scrollable::StyleSheet for Board {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> iced::widget::scrollable::Scrollbar {
        match style {
            Scrollable::Default => iced::widget::scrollable::Scrollbar {
                background: Some(Background::Color(Color::from_rgb8(96, 91, 86))),
                border: Border::default(),
                scroller: Scroller {
                    color: Color::WHITE,
                    border: Border::default(),
                },
            },
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> iced::widget::scrollable::Scrollbar {
        self.active(style)
    }
}
