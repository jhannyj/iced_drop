#![feature(get_many_mut)]
#![feature(hash_raw_entry)]

use std::time::Instant;

use highlight::{should_update_droppable, zone_update, Highlight, Highlightable, ZoneUpdate};
use iced::{
    advanced::widget::Id,
    widget::{column, container, text, text_input},
    Application, Command, Length, Point, Rectangle,
};
use iced_drop::find_zones;
use iced_drop::widget::droppable::State as DroppableState;
use operation::swap_modify_states;
use tree::{List, Slot, Task, TreeData, TreeElement, TreeLocation};

mod highlight;
mod operation;
mod theme;
mod tree;

const HEADER_HEIGHT: f32 = 80.0;
const DOUBLE_CLICK_TIME: u128 = 500;

fn main() -> iced::Result {
    TodoBoard::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    // Task editing
    EditTask(TreeLocation, iced::widget::text_input::Id),
    UpdateTask(TreeLocation, String),
    StopEditingTask,
    // Task creation
    UpdateTaskWriter(TreeLocation, String),
    WriteTask(TreeLocation),
    // Drag/drop
    DragTask(TreeLocation, Point, Rectangle),
    HandleTaskZones(Vec<(Id, Rectangle)>),
    DropTask(TreeLocation, Point, Rectangle),
    DragList(TreeLocation, Point, Rectangle),
    HandleListZones(Vec<(Id, Rectangle)>),
    DropList(TreeLocation, Point, Rectangle),
    TaskDropCanceled,
    ListDropCanceled,
}

struct TodoBoard {
    tree: TreeData,
    clicked_task: (TreeLocation, Instant),
    editing_task: Option<TreeLocation>,
    tasks_highlight: highlight::Highlight,
    lists_highlight: highlight::Highlight,
}

impl Default for TodoBoard {
    fn default() -> Self {
        Self {
            tree: TreeData::new(vec![
                Slot::new(List::new("Todo", vec![Task::new("Fix bugs")])),
                Slot::new(List::new("Doing", vec![Task::new("Write code")])),
                Slot::new(List::new("Done", vec![Task::new("Drag and drop")])),
            ]),
            clicked_task: (tree::NULL_TASK_LOC, Instant::now()),
            editing_task: None,
            tasks_highlight: Highlight::default(),
            lists_highlight: Highlight::default(),
        }
    }
}

impl iced::Application for TodoBoard {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = theme::Board;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (TodoBoard::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        "Todo".to_string()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let header = container(text("TODO Board").size(30).style(theme::Text::Title))
            .padding(10.0)
            .width(Length::Fill)
            .height(Length::Fixed(HEADER_HEIGHT))
            .style(theme::Container::Title);
        container(
            column![header, self.tree.view()]
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Background)
        .into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::EditTask(t_loc, ti_id) => {
                self.stop_editing_task();

                let (clicked, time) = &self.clicked_task;
                if *clicked == t_loc && time.elapsed().as_millis() < DOUBLE_CLICK_TIME {
                    if let Some(task) = self.tree.task_mut(&t_loc) {
                        task.editing = true;
                        self.editing_task = Some(t_loc);
                        return text_input::focus(ti_id);
                    }
                }
                self.clicked_task = (t_loc, Instant::now());
            }
            Message::UpdateTask(t_loc, content) => {
                if let Some(task) = self.tree.task_mut(&t_loc) {
                    task.content = content;
                }
            }
            Message::StopEditingTask => {
                self.stop_editing_task();
            }
            // Task drag/drop
            Message::DragTask(t_loc, __, t_bounds) => {
                let new_highlight =
                    highlight::dragged(&self.tasks_highlight, t_loc.clone(), t_bounds);
                if should_update_droppable(&self.tasks_highlight, &new_highlight, &t_loc) {
                    if let Some(task) = self.tree.task_mut(&t_loc) {
                        task.set_highlight(true)
                    }
                }
                self.tasks_highlight = new_highlight;
                return find_zones(
                    Message::HandleTaskZones,
                    move |zone_bounds| zone_bounds.intersects(&t_bounds),
                    Some(self.tree.task_options(&t_loc)),
                    None,
                );
            }
            Message::HandleTaskZones(zones) => {
                let new_highlight =
                    highlight::zones_found(&self.tasks_highlight, &map_zones(&self.tree, zones));
                zone_update(&self.tasks_highlight, &new_highlight).update(
                    &mut self.tree,
                    &self.tasks_highlight,
                    &new_highlight,
                );
                self.tasks_highlight = new_highlight;
            }
            Message::DropTask(t_loc, _, _) => {
                if let Some(h_loc) = &self.tasks_highlight.hovered {
                    match h_loc.element() {
                        TreeElement::List => task_dropped_on_list(&mut self.tree, &t_loc, &h_loc),
                        TreeElement::Task(_) => {
                            task_dropped_on_task(&mut self.tree, &t_loc, &h_loc)
                        }
                        _ => (),
                    }
                } else {
                    self.tree.list_mut(&t_loc).remove_task(&t_loc);
                }
                self.tasks_highlight = highlight::dropped();
            }

            // List drag/drop
            Message::DragList(l_loc, _, l_bounds) => {
                let new_highlight =
                    highlight::dragged(&self.lists_highlight, l_loc.clone(), l_bounds);
                if should_update_droppable(&self.lists_highlight, &new_highlight, &l_loc) {
                    self.tree.list_mut(&l_loc).set_highlight(false);
                }
                self.lists_highlight = new_highlight;
                return find_zones(
                    Message::HandleListZones,
                    move |zone_bounds| zone_bounds.intersects(&l_bounds),
                    Some(self.tree.list_options()),
                    None,
                );
            }
            Message::HandleListZones(zones) => {
                let new_info =
                    highlight::zones_found(&self.lists_highlight, &map_zones(&self.tree, zones));
                let highlight_update = zone_update(&self.lists_highlight, &new_info);
                highlight_update.update(&mut self.tree, &self.lists_highlight, &new_info);
                self.lists_highlight = new_info;

                if highlight_update == ZoneUpdate::Replace {
                    if let Some(d_loc) = &self.lists_highlight.dragging {
                        if let Some(h_loc) = &self.lists_highlight.hovered {
                            return move_list_to_zone(&mut self.tree, &d_loc.0, &h_loc);
                        }
                    }
                }
            }
            Message::DropList(l_loc, _, _) => {
                self.tree.list_mut(&l_loc).set_highlight(false);
                if let Some(s_loc) = &self.lists_highlight.hovered {
                    self.tree.slot_mut(s_loc.slot()).set_highlight(false);
                }
                self.tasks_highlight = highlight::dropped();
            }
            Message::UpdateTaskWriter(l_loc, new_str) => {
                self.stop_editing_task();
                self.tree.list_mut(&l_loc).task_adder.text = new_str;
            }
            Message::WriteTask(l_loc) => {
                let (text, id) = {
                    let adder = &mut self.tree.list_mut(&l_loc).task_adder;
                    let text = adder.text.clone();
                    adder.text.clear();
                    (text, adder.id())
                };
                if text.is_empty() {
                    return Command::none();
                }
                let task = Task::new(&text);
                self.tree.list_mut(&l_loc).push_task(task);
                return text_input::focus(id);
            }
            Message::TaskDropCanceled => {
                if let Some(d_loc) = &self.tasks_highlight.dragging {
                    if let Some(task) = self.tree.task_mut(&d_loc.0) {
                        task.set_highlight(false);
                        highlight::set_hovered(&mut self.tree, &self.tasks_highlight, false);
                    }
                }
                self.tasks_highlight = highlight::dropped();
            }
            Message::ListDropCanceled => {
                if let Some(d_loc) = &self.lists_highlight.dragging {
                    self.tree.list_mut(&d_loc.0).set_highlight(false);
                    self.tree.slot_mut(d_loc.0.slot()).set_highlight(false);
                }
                self.lists_highlight = highlight::dropped();
            }
        }
        Command::none()
    }
}

impl TodoBoard {
    fn stop_editing_task(&mut self) {
        if let Some(loc) = self.editing_task {
            if let Some(task) = self.tree.task_mut(&loc) {
                task.editing = false;
                self.editing_task = None;
            }
        }
    }
}

fn map_zones(tree: &TreeData, zones: Vec<(Id, Rectangle)>) -> Vec<(TreeLocation, Rectangle)> {
    zones
        .into_iter()
        .filter_map(|(id, rect)| {
            if let Some(loc) = tree.find(&id) {
                Some((loc, rect))
            } else {
                None
            }
        })
        .collect()
}

fn task_dropped_on_list(tree: &mut TreeData, d_loc: &TreeLocation, h_loc: &TreeLocation) {
    if let Some(task) = tree.task_mut(d_loc) {
        task.set_highlight(false);
        let task = {
            let list = tree.list_mut(h_loc);
            list.set_highlight(false);
            if d_loc.slot() == h_loc.slot() {
                return;
            }
            if let Some(task) = tree.list_mut(d_loc).remove_task(d_loc) {
                task
            } else {
                return;
            }
        };
        tree.list_mut(h_loc).push_task(task);
    }
}

fn task_dropped_on_task(tree: &mut TreeData, d_loc: &TreeLocation, h_loc: &TreeLocation) {
    if let Some(d_task) = tree.task_mut(d_loc) {
        d_task.set_highlight(false);
        let h_task = tree.task_mut(h_loc);
        if let Some(task) = h_task {
            task.set_highlight(false);
        } else {
            return;
        }
    }

    if d_loc.slot() != h_loc.slot() {
        if let TreeElement::Task(i) = h_loc.element() {
            let task = tree.list_mut(d_loc).remove_task(d_loc).unwrap();
            tree.list_mut(h_loc).inser_task(task, *i);
        }
    } else {
        tree.list_mut(d_loc).move_task(d_loc, h_loc);
    }
}

fn move_list_to_zone(
    tree: &mut TreeData,
    d_loc: &TreeLocation,
    h_loc: &TreeLocation,
) -> Command<Message> {
    let l1 = tree.list_mut(d_loc).id();
    let l2 = tree.list_mut(h_loc).id();
    tree.swap_lists(d_loc, h_loc);
    return swap_modify_states(l1, l2, |_old: &DroppableState, new: &DroppableState| {
        new.clone()
    });
}
