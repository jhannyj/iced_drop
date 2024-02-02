use iced::{
    advanced::widget::Id,
    theme,
    widget::{column, container, row, text},
    Application, Border, Length, Point, Rectangle,
};
use iced_drop::droppable;

const HEADER_HEIGHT: f32 = 80.0;
const COLORS_HEIGHT: f32 = 40.0;
const COLORS_ROUNDNESS: f32 = 10.0;
const COLORS_CONTAINER_WIDTH: f32 = 300.0;

fn main() -> iced::Result {
    ColorDropper::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    DropColor(DColor, Point, Rectangle),
    HandleZonesFound(DColor, Vec<(Id, Rectangle)>),
}

#[derive(Default)]
struct ColorDropper {
    zone_color: DColor,
}

impl Application for ColorDropper {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::theme::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        "Basic".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::DropColor(color, point, _bounds) => {
                return iced_drop::zones_on_point(
                    move |zones| Message::HandleZonesFound(color, zones),
                    point,
                    None,
                    None,
                );
            }
            Message::HandleZonesFound(color, _zones) => {
                self.zone_color = color;
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let header = container(text("Color Dropper").size(30))
            .style(theme::Container::Box)
            .padding(10.0)
            .width(Length::Fill)
            .height(Length::Fixed(HEADER_HEIGHT));
        let colors = DColor::SHOWN.iter().map(|color| {
            let color = *color;
            droppable(
                container(text(color.to_string()).size(20))
                    .center_x()
                    .center_y()
                    .style(theme::Container::Custom(Box::new(
                        DColor::White.container_style(true),
                    )))
                    .width(Length::Fill)
                    .height(Length::Fixed(COLORS_HEIGHT)),
            )
            .on_drop(move |point, rect| Message::DropColor(color, point, rect))
            .into()
        });
        let colors_holder = container(column(colors).spacing(20.0).padding(20.0))
            .center_x()
            .center_y()
            .style(theme::Container::Custom(Box::new(
                DColor::Background.container_style(false),
            )))
            .height(Length::Fill)
            .width(Length::Fixed(COLORS_CONTAINER_WIDTH));
        let drop_zone = container(
            text(self.zone_color.fun_fact())
                .size(20)
                .style(theme::Text::Color(self.zone_color.text_color())),
        )
        .id(iced::widget::container::Id::new("drop_zone_1"))
        .style(theme::Container::Custom(Box::new(
            self.zone_color.container_style(false),
        )))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();
        column![header, row![colors_holder, drop_zone]].into()
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum DColor {
    #[default]
    Default,
    Background,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Orange,
    Black,
    White,
    Gray,
    Pink,
}

impl std::fmt::Display for DColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DColor {
    const SHOWN: [DColor; 10] = [
        DColor::Red,
        DColor::Green,
        DColor::Blue,
        DColor::Yellow,
        DColor::Purple,
        DColor::Orange,
        DColor::Black,
        DColor::White,
        DColor::Gray,
        DColor::Pink,
    ];

    fn color(&self) -> iced::Color {
        match self {
            DColor::Default => iced::Color::WHITE,
            DColor::Background => iced::Color::from_rgb8(165, 42, 42),
            DColor::Red => iced::Color::from_rgb8(255, 0, 0),
            DColor::Green => iced::Color::from_rgb8(0, 255, 0),
            DColor::Blue => iced::Color::from_rgb8(0, 0, 255),
            DColor::Yellow => iced::Color::from_rgb8(255, 255, 0),
            DColor::Purple => iced::Color::from_rgb8(128, 0, 128),
            DColor::Orange => iced::Color::from_rgb8(255, 165, 0),
            DColor::Black => iced::Color::BLACK,
            DColor::White => iced::Color::WHITE,
            DColor::Gray => iced::Color::from_rgb8(128, 128, 128),
            DColor::Pink => iced::Color::from_rgb8(255, 192, 203),
        }
    }

    fn text_color(&self) -> iced::Color {
        match self {
            DColor::Default
            | DColor::Yellow
            | DColor::Orange
            | DColor::Green
            | DColor::White
            | DColor::Pink => iced::Color::BLACK,
            _ => iced::Color::WHITE,
        }
    }

    fn container_style(&self, droppable: bool) -> ContainerStyle {
        ContainerStyle {
            color: self.color(),
            droppable,
        }
    }

    fn fun_fact<'a>(&self) -> &'a str {
        match self {
            DColor::Default => "Drop a color here to learn about it!",
            DColor::Red => "Red is the color of fire and blood",
            DColor::Green => "Green is the color of life and renewal",
            DColor::Blue => "Blue is the color of the sky and sea",
            DColor::Yellow => "Yellow is the color of sunshine and happiness",
            DColor::Purple => "Purple is the color of royalty and luxury",
            DColor::Orange => "Orange is the color of creativity and determination",
            DColor::Black => "Black is the color of power and elegance",
            DColor::White => "White is the color of purity and innocence",
            DColor::Gray => "Gray is the color of compromise and control",
            DColor::Pink => "Pink is the color of love and compassion",
            DColor::Background => "Hacker level achieved",
        }
    }
}

struct ContainerStyle {
    color: iced::Color,
    droppable: bool,
}

impl iced::widget::container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let border = if self.droppable {
            Border::with_radius(COLORS_ROUNDNESS)
        } else {
            Border::default()
        };
        container::Appearance {
            background: Some(iced::Background::Color(self.color)),
            border,
            ..Default::default()
        }
    }
}
