use iced::widget::container::Style;
use iced::{color, Border, Theme};

pub fn active_slot(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(202, 233, 255).into()),
        border: Border {
            color: color!(202, 233, 255),
            radius: 10.0.into(),
            width: 5.0,
        },
        ..Default::default()
    }
}

pub fn title(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(131, 122, 117).into()),
        ..Default::default()
    }
}

pub fn list(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(172, 193, 138).into()),
        border: Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn active_list(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(172, 193, 138).into()),
        border: Border {
            color: color!(202, 233, 255),
            radius: 10.0.into(),
            width: 5.0,
        },
        ..Default::default()
    }
}

pub fn todo(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(218, 254, 183).into()),
        border: Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn active_todo(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(218, 254, 183).into()),
        border: Border {
            color: color!(202, 233, 255),
            radius: 10.0.into(),
            width: 5.0,
        },
        ..Default::default()
    }
}

pub fn background(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(96, 91, 86).into()),
        ..Default::default()
    }
}

pub fn adder_tooltip(_theme: &Theme) -> Style {
    Style {
        text_color: Some(color!(242, 251, 224)),
        background: Some(color!(131, 122, 117).into()),
        border: Border {
            radius: 5.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
