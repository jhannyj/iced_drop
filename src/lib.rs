pub mod widget;

use iced::{
    advanced::widget::{operate, Id},
    advanced::{graphics::futures::MaybeSend, renderer},
    task::Task,
    Element, Point, Rectangle,
};

use widget::droppable::*;
use widget::operation::drop;

pub fn droppable<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Droppable<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    Droppable::new(content)
}

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
    .map(move |id| msg(id))
}

pub fn find_zones<Message, MF, F>(
    msg: MF,
    filter: F,
    options: Option<Vec<Id>>,
    depth: Option<usize>,
) -> Task<Message>
where
    Message: Send + 'static,
    MF: Fn(Vec<(Id, Rectangle)>) -> Message + MaybeSend + Sync + Clone + 'static,
    F: Fn(&Rectangle) -> bool + Send + 'static,
{
    operate(drop::find_zones(filter, options, depth)).map(move |id| msg(id))
}
