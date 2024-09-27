#![feature(get_many_mut)]
#![feature(hash_raw_entry)]

use std::time::Instant;

use highlight::{should_update_droppable, zone_update, Highlight, Highlightable, ZoneUpdate};
use iced::{
    advanced::widget::Id,
    widget::{column, container, text, text_input},
    Element, Length, Point, Rectangle, Task,
};
use iced_drop::find_zones;
use iced_drop::widget::droppable::State as DroppableState;
use operation::swap_modify_states;
use tree::{List, Slot, Todo, TreeData, TreeElement, TreeLocation};

mod highlight;
mod operation;
mod theme;
mod tree;

const HEADER_HEIGHT: f32 = 80.0;
const DOUBLE_CLICK_TIME: u128 = 500;

fn main() -> iced::Result {
    iced::application(TodoBoard::title, TodoBoard::update, TodoBoard::view)
        .theme(TodoBoard::theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    // To-do editing
    EditTodo(TreeLocation, iced::widget::text_input::Id),
    UpdateTodo(TreeLocation, String),
    StopEditingTodo,

    // To-do creation
    UpdateTodoWriter(TreeLocation, String),
    WriteTodo(TreeLocation),

    // Drag/drop to-dos
    DragTodo(TreeLocation, Point, Rectangle),
    HandleTodoZones(Vec<(Id, Rectangle)>),
    #[allow(dead_code)]
    DropTodo(TreeLocation, Point, Rectangle),
    TodoDropCanceled,

    // Drag/drop lists
    #[allow(dead_code)]
    DragList(TreeLocation, Point, Rectangle),
    HandleListZones(Vec<(Id, Rectangle)>),
    #[allow(dead_code)]
    DropList(TreeLocation, Point, Rectangle),
    ListDropCanceled,
}

struct TodoBoard {
    tree: TreeData,
    clicked: (TreeLocation, Instant),
    editing: Option<TreeLocation>,
    todos_highlight: highlight::Highlight,
    lists_highlight: highlight::Highlight,
}

impl Default for TodoBoard {
    fn default() -> Self {
        Self {
            tree: TreeData::new(vec![
                Slot::new(List::new("Todo", vec![Todo::new("Fix bugs")])),
                Slot::new(List::new("Doing", vec![Todo::new("Write code")])),
                Slot::new(List::new("Done", vec![Todo::new("Drag and drop")])),
            ]),
            clicked: (tree::NULL_TODO_LOC, Instant::now()),
            editing: None,
            todos_highlight: Highlight::default(),
            lists_highlight: Highlight::default(),
        }
    }
}

impl TodoBoard {
    fn title(&self) -> String {
        "Todo".to_string()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinFrappe
    }

    fn view(&self) -> Element<'_, Message> {
        let header = container(text("TODO Board").size(30).style(theme::text::title))
            .padding(10.0)
            .width(Length::Fill)
            .height(Length::Fixed(HEADER_HEIGHT))
            .style(theme::container::title);
        container(
            column![header, self.tree.view()]
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::background)
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditTodo(t_loc, ti_id) => {
                self.stop_editing();

                let (clicked, time) = &self.clicked;
                if *clicked == t_loc && time.elapsed().as_millis() < DOUBLE_CLICK_TIME {
                    if let Some(todo) = self.tree.todo_mut(&t_loc) {
                        todo.editing = true;
                        self.editing = Some(t_loc);
                        return text_input::focus(ti_id);
                    }
                }
                self.clicked = (t_loc, Instant::now());
            }
            Message::UpdateTodo(t_loc, content) => {
                if let Some(todo) = self.tree.todo_mut(&t_loc) {
                    todo.content = content;
                }
            }
            Message::StopEditingTodo => {
                self.stop_editing();
            }
            // To-do drag/drop
            Message::DragTodo(t_loc, __, t_bounds) => {
                let new_highlight =
                    highlight::dragged(&self.todos_highlight, t_loc.clone(), t_bounds);
                if should_update_droppable(&self.todos_highlight, &new_highlight, &t_loc) {
                    if let Some(todo) = self.tree.todo_mut(&t_loc) {
                        todo.set_highlight(true)
                    }
                }
                self.todos_highlight = new_highlight;
                return find_zones(
                    Message::HandleTodoZones,
                    move |zone_bounds| zone_bounds.intersects(&t_bounds),
                    Some(self.tree.todo_options(&t_loc)),
                    None,
                );
            }
            Message::HandleTodoZones(zones) => {
                let new_highlight =
                    highlight::zones_found(&self.todos_highlight, &map_zones(&self.tree, zones));
                zone_update(&self.todos_highlight, &new_highlight).update(
                    &mut self.tree,
                    &self.todos_highlight,
                    &new_highlight,
                );
                self.todos_highlight = new_highlight;
            }
            Message::DropTodo(t_loc, _, _) => {
                if let Some(h_loc) = &self.todos_highlight.hovered {
                    match h_loc.element() {
                        TreeElement::List => todo_dropped_on_list(&mut self.tree, &t_loc, &h_loc),
                        TreeElement::Todo(_) => {
                            todo_dropped_on_todo(&mut self.tree, &t_loc, &h_loc)
                        }
                        _ => (),
                    }
                } else {
                    self.tree.list_mut(&t_loc).remove(&t_loc);
                }
                self.todos_highlight = highlight::dropped();
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
                self.todos_highlight = highlight::dropped();
            }
            Message::UpdateTodoWriter(l_loc, new_str) => {
                self.stop_editing();
                self.tree.list_mut(&l_loc).todo_adder.text = new_str;
            }
            Message::WriteTodo(l_loc) => {
                let (text, id) = {
                    let adder = &mut self.tree.list_mut(&l_loc).todo_adder;
                    let text = adder.text.clone();
                    adder.text.clear();
                    (text, adder.id())
                };
                if text.is_empty() {
                    return Task::none();
                }
                let todo = Todo::new(&text);
                self.tree.list_mut(&l_loc).push(todo);
                return text_input::focus(id);
            }
            Message::TodoDropCanceled => {
                if let Some(d_loc) = &self.todos_highlight.dragging {
                    if let Some(todo) = self.tree.todo_mut(&d_loc.0) {
                        todo.set_highlight(false);
                        highlight::set_hovered(&mut self.tree, &self.todos_highlight, false);
                    }
                }
                self.todos_highlight = highlight::dropped();
            }
            Message::ListDropCanceled => {
                if let Some(d_loc) = &self.lists_highlight.dragging {
                    self.tree.list_mut(&d_loc.0).set_highlight(false);
                    self.tree.slot_mut(d_loc.0.slot()).set_highlight(false);
                }
                self.lists_highlight = highlight::dropped();
            }
        }
        Task::none()
    }
}

impl TodoBoard {
    fn stop_editing(&mut self) {
        if let Some(loc) = self.editing {
            if let Some(todo) = self.tree.todo_mut(&loc) {
                todo.editing = false;
                self.editing = None;
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

fn todo_dropped_on_list(tree: &mut TreeData, d_loc: &TreeLocation, h_loc: &TreeLocation) {
    if let Some(todo) = tree.todo_mut(d_loc) {
        todo.set_highlight(false);
        let todo = {
            let list = tree.list_mut(h_loc);
            list.set_highlight(false);
            if d_loc.slot() == h_loc.slot() {
                return;
            }
            if let Some(todo) = tree.list_mut(d_loc).remove(d_loc) {
                todo
            } else {
                return;
            }
        };
        tree.list_mut(h_loc).push(todo);
    }
}

fn todo_dropped_on_todo(tree: &mut TreeData, d_loc: &TreeLocation, h_loc: &TreeLocation) {
    if let Some(d_todo) = tree.todo_mut(d_loc) {
        d_todo.set_highlight(false);
        let h_todo = tree.todo_mut(h_loc);
        if let Some(todo) = h_todo {
            todo.set_highlight(false);
        } else {
            return;
        }
    }

    if d_loc.slot() != h_loc.slot() {
        if let TreeElement::Todo(i) = h_loc.element() {
            let todo = tree.list_mut(d_loc).remove(d_loc).unwrap();
            tree.list_mut(h_loc).insert(todo, *i);
        }
    } else {
        tree.list_mut(d_loc).move_todo(d_loc, h_loc);
    }
}

fn move_list_to_zone(
    tree: &mut TreeData,
    d_loc: &TreeLocation,
    h_loc: &TreeLocation,
) -> Task<Message> {
    let l1 = tree.list_mut(d_loc).id();
    let l2 = tree.list_mut(h_loc).id();
    tree.swap_lists(d_loc, h_loc);
    return swap_modify_states(l1, l2, |_old: &DroppableState, new: &DroppableState| {
        new.clone()
    });
}
