use iced::widget::button::{Status, Style};
use iced::{color, Border, Theme};

pub fn adder(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(color!(96, 91, 86).into()),
        text_color: color!(242, 251, 224),
        border: Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
