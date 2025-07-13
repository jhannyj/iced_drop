use iced::Border;
use iced::widget::container::Id as CId;
use iced::{
    Element, Fill, Length, Point, Rectangle, Task,
    advanced::widget::Id,
    widget::{column, container, row, text},
};
use iced_drop::droppable;

const HEADER_HEIGHT: f32 = 80.0;
const COLORS_HEIGHT: f32 = 40.0;
const COLORS_ROUNDNESS: f32 = 10.0;
const COLORS_CONTAINER_WIDTH: f32 = 130.0;

fn main() -> iced::Result {
    iced::application(
        ColorDropper::default,
        ColorDropper::update,
        ColorDropper::view,
    )
    .title(ColorDropper::title)
    .theme(ColorDropper::theme)
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    DropColor(DColor, Point, Rectangle),
    HandleZonesFound(DColor, Vec<(Id, Rectangle)>),
}

struct ColorDropper {
    left_color: DColor,
    right_color: DColor,
    left: iced::widget::container::Id,
    right: iced::widget::container::Id,
}

impl Default for ColorDropper {
    fn default() -> Self {
        Self {
            left_color: DColor::Default,
            right_color: DColor::Default,
            left: CId::new("left"),
            right: CId::new("right"),
        }
    }
}

impl ColorDropper {
    fn title(&self) -> String {
        "Basic".to_string()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinFrappe
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DropColor(color, point, _bounds) => {
                return iced_drop::zones_on_point(
                    move |zones| Message::HandleZonesFound(color, zones),
                    point,
                    None,
                    None,
                );
            }
            Message::HandleZonesFound(color, zones) => {
                if let Some((zone, _)) = zones.get(0) {
                    if *zone == self.left.clone().into() {
                        self.left_color = color;
                    } else {
                        self.right_color = color;
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let header = container(text("Color Dropper").size(30))
            .padding(10.0)
            .width(Length::Fill)
            .height(Length::Fixed(HEADER_HEIGHT));
        let colors = DColor::SHOWN.iter().map(|color| {
            let color = *color;
            droppable(
                container(text(color.to_string()).size(20))
                    .center(Fill)
                    .style(move |_| color.style())
                    .width(Length::Fill)
                    .height(Length::Fixed(COLORS_HEIGHT)),
            )
            .on_drop(move |point, rect| Message::DropColor(color, point, rect))
            .into()
        });
        let colors_holder =
            container(column(colors).spacing(20.0).padding(20.0))
                .center(Fill)
                .height(Length::Fill)
                .width(Length::Fixed(COLORS_CONTAINER_WIDTH));
        column![
            header,
            row![
                colors_holder,
                drop_zone(self.left_color, self.left.clone()),
                drop_zone(self.right_color, self.right.clone())
            ]
            .spacing(5)
        ]
        .padding(5)
        .into()
    }
}

fn drop_zone<'a>(
    color: DColor,
    id: iced::widget::container::Id,
) -> iced::Element<'a, Message, iced::Theme, iced::Renderer> {
    container(text(color.fun_fact()).size(20))
        .style(move |_| color.style())
        .id(id)
        .width(Fill)
        .height(Fill)
        .center(Fill)
        .into()
}

#[derive(Debug, Clone, Copy, Default)]
enum DColor {
    #[default]
    Default,
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
            DColor::Default => iced::Color::from_rgb8(245, 245, 245),
            DColor::Red => iced::Color::from_rgb8(220, 20, 20),
            DColor::Green => iced::Color::from_rgb8(50, 205, 50),
            DColor::Blue => iced::Color::from_rgb8(40, 80, 150),
            DColor::Yellow => iced::Color::from_rgb8(255, 215, 0),
            DColor::Purple => iced::Color::from_rgb8(100, 50, 150),
            DColor::Orange => iced::Color::from_rgb8(255, 140, 0),
            DColor::Black => iced::Color::from_rgb8(20, 20, 20),
            DColor::White => iced::Color::from_rgb8(250, 250, 250),
            DColor::Gray => iced::Color::from_rgb8(105, 105, 105),
            DColor::Pink => iced::Color::from_rgb8(255, 105, 180),
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

    fn style(&self) -> container::Style {
        iced::widget::container::Style {
            background: Some(self.color().into()),
            border: Border {
                color: iced::Color::BLACK,
                width: 1.0,
                radius: COLORS_ROUNDNESS.into(),
            },
            text_color: Some(self.text_color()),
            ..Default::default()
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
            DColor::Orange => {
                "Orange is the color of creativity and determination"
            }
            DColor::Black => "Black is the color of power and elegance",
            DColor::White => "White is the color of purity and innocence",
            DColor::Gray => "Gray is the color of compromise and control",
            DColor::Pink => "Pink is the color of love and compassion",
        }
    }
}
