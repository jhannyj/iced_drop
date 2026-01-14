use iced::widget::text::Style;
use iced::{Theme, color};

pub fn title(_theme: &Theme) -> Style {
    Style {
        color: Some(color!(242, 251, 224)),
    }
}

pub fn list_name(_theme: &Theme) -> Style {
    Style {
        color: Some(color!(96, 91, 86)),
    }
}

pub fn todo(_theme: &Theme) -> Style {
    Style {
        color: Some(color!(131, 122, 117)),
    }
}
