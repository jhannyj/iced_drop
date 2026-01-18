pub mod widget;

use iced_core::{renderer, Element};
use widget::droppable::*;

#[cfg(feature = "helpers")]
use iced_core::Point;
#[cfg(feature = "helpers")]
use iced_core::widget::Id;
#[cfg(feature = "helpers")]
use iced_widget::graphics::futures::MaybeSend;
#[cfg(feature = "helpers")]
use crate::widget::operation::drop;
#[cfg(feature = "helpers")]
use iced_core::Rectangle;
#[cfg(feature = "helpers")]
use iced::advanced::widget::operate;
#[cfg(feature = "helpers")]
use iced::Task;

#[cfg(not(feature = "helpers"))]
use widget::operation::drop;
#[cfg(not(feature = "helpers"))]
pub use drop::find_zones;

pub fn droppable<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Droppable<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    Droppable::new(content)
}

#[cfg(feature = "helpers")]
pub fn zones_on_point<T, MF>(
    msg: MF,
    point: Point,
    options: Option<Vec<Id>>,
    depth: Option<usize>,
) -> Task<T>
where
    T: Send + 'static,
    MF: Fn(Vec<(Id, Rectangle)>) -> T + MaybeSend + Sync + Clone + 'static,
{
    operate(drop::find_zones(
        move |bounds| bounds.contains(point),
        options,
        depth,
    ))
        .map(msg)
}

#[cfg(feature = "helpers")]
pub fn find_zones<Message, MF, F>(
    msg: MF,
    filter: F,
    options: Option<Vec<Id>>,
    depth: Option<usize>,
) -> Task<Message>
where
    Message: Send + 'static,
    MF: Fn(Vec<(Id, Rectangle)>) -> Message
    + MaybeSend
    + Sync
    + Clone
    + 'static,
    F: Fn(&Rectangle) -> bool + Send + 'static,
{
    operate(drop::find_zones(filter, options, depth)).map(msg)
}
