use iced::widget::text_input::{Status, Style};
use iced::{Border, Color, Theme, color};

pub fn element_adder(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: color!(218, 254, 183).into(),
        border: Border {
            color: color!(96, 91, 86),
            width: 1.0,
            radius: 10.0.into(),
        },
        icon: Color::BLACK,
        placeholder: color!(131, 122, 117, 0.7),
        value: color!(131, 122, 117),
        selection: Color::WHITE,
    }
}
