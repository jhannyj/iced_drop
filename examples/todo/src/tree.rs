use std::sync::atomic::AtomicUsize;

use iced::advanced::widget::Id;

use iced::widget::tooltip;
use iced::{
    alignment,
    widget::{button, column, container, horizontal_space, row, text, text_input},
    Center, Element, Length, Size,
};
use iced_drop::droppable;

use crate::{highlight::Highlightable, theme, Message};

pub const NULL_TODO_LOC: TreeLocation = TreeLocation {
    slot: 0,
    element: TreeElement::Slot,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeLocation {
    slot: usize,
    element: TreeElement,
}

impl TreeLocation {
    fn new(slot: usize, element: TreeElement) -> Self {
        Self { slot, element }
    }

    pub fn element(&self) -> &TreeElement {
        &self.element
    }

    pub fn slot(&self) -> usize {
        self.slot
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeElement {
    Slot,
    List,
    Todo(usize),
}

pub struct ElementAdder {
    pub text: String,
    id: iced::widget::text_input::Id,
}

/// Contains items organized by slots, and lists
pub struct TreeData {
    slots: Vec<Slot>,
}

impl TreeData {
    pub fn new(slots: Vec<Slot>) -> Self {
        Self { slots }
    }
    /// Convert the tree into an element that iced can render
    pub fn view(&self) -> Element<Message> {
        let children = self.slots.iter().enumerate().map(|(i, slot)| slot.view(i));
        row(children)
            .spacing(10.0)
            .padding(20.0)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn find(&self, id: &Id) -> Option<TreeLocation> {
        for (i, slot) in self.slots.iter().enumerate() {
            if slot.id == *id {
                return Some(TreeLocation::new(i, TreeElement::Slot));
            }
            if slot.list.id == *id {
                return Some(TreeLocation::new(i, TreeElement::List));
            }
            for (j, list) in slot.list.todos.iter().enumerate() {
                if list.id == *id {
                    return Some(TreeLocation::new(i, TreeElement::Todo(j)));
                }
            }
        }
        None
    }

    pub fn slot_mut(&mut self, index: usize) -> &mut Slot {
        self.slots.get_mut(index).unwrap()
    }

    pub fn list_mut(&mut self, location: &TreeLocation) -> &mut List {
        let i = location.slot;
        match location.element {
            TreeElement::Slot => &mut self.slots[i].list,
            TreeElement::List => &mut self.slots[i].list,
            TreeElement::Todo(_) => &mut self.slots[i].list,
        }
    }

    pub fn todo(&self, location: &TreeLocation) -> Option<&Todo> {
        let i = location.slot;
        match location.element {
            TreeElement::Slot => None,
            TreeElement::List => None,
            TreeElement::Todo(j) => Some(&self.slots[i].list.todos[j]),
        }
    }

    pub fn todo_mut(&mut self, location: &TreeLocation) -> Option<&mut Todo> {
        let i = location.slot;
        match location.element {
            TreeElement::Slot => None,
            TreeElement::List => None,
            TreeElement::Todo(j) => Some(&mut self.slots[i].list.todos[j]),
        }
    }

    pub fn swap_lists(&mut self, l1: &TreeLocation, l2: &TreeLocation) {
        let [s1, s2] = if let Ok(slots) = self.slots.get_many_mut([l1.slot, l2.slot]) {
            slots
        } else {
            return;
        };
        std::mem::swap(&mut s1.list, &mut s2.list);
    }

    /// Returns the widget Id of all the widgets wich a item can be dropped on
    pub fn todo_options(&self, t_loc: &TreeLocation) -> Vec<Id> {
        let todo_id = if let Some(todo) = self.todo(t_loc) {
            todo.id.clone()
        } else {
            return vec![];
        };
        self.slots
            .iter()
            .map(|slot| {
                slot.list.todos.iter().filter_map(|todo| {
                    if todo.id != todo_id {
                        Some(todo.id.clone())
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .chain(self.slots.iter().map(|slot| slot.list.id.clone()))
            .collect()
    }

    /// Returns the widget Id of all the widgets wich a list can be dropped on
    pub fn list_options(&self) -> Vec<Id> {
        self.slots.iter().map(|slot| slot.id.clone()).collect()
    }
}

static NEXT_SLOT: AtomicUsize = AtomicUsize::new(0);

/// Some slot that a list can be dragged into
pub struct Slot {
    id: Id,
    list: List,
    c_id: iced::widget::container::Id,
    highlight: bool,
}

impl Highlightable for Slot {
    fn set_highlight(&mut self, highlight: bool) {
        self.highlight = highlight;
    }
}

impl Slot {
    /// Create a new slot with a list
    pub fn new(list: List) -> Self {
        let id = NEXT_SLOT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let c_id = iced::widget::container::Id::new(format!("slot_{}", id));
        Self {
            id: Id::from(c_id.clone()),
            c_id,
            list,
            highlight: false,
        }
    }

    /// Convert the slot into an element that iced can render
    fn view(&self, index: usize) -> Element<Message> {
        container(self.list.view(index))
            .id(self.c_id.clone())
            .style(if self.highlight {
                theme::container::active_slot
            } else {
                container::transparent
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(3.5)
            .into()
    }
}

impl ElementAdder {
    pub fn new(id: usize) -> Self {
        Self {
            id: iced::widget::text_input::Id::new(format!("todo_adder_{}", id)),
            text: String::new(),
        }
    }

    pub fn id(&self) -> iced::widget::text_input::Id {
        self.id.clone()
    }
}

static NEXT_LIST: AtomicUsize = AtomicUsize::new(0);

/// Some list that contains to-do tasks and can be dragged into a slot.
/// Tasks can also be dragged into a list.
pub struct List {
    pub todo_adder: ElementAdder,
    id: Id,
    title: String,
    todos: Vec<Todo>,
    highlight: bool,
}

impl Highlightable for List {
    fn set_highlight(&mut self, highlight: bool) {
        self.highlight = highlight;
    }
}

impl List {
    /// Create a new list with a title
    pub fn new(title: &str, todos: Vec<Todo>) -> Self {
        let id = NEXT_LIST.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            id: Id::new(format!("list_{}", id)),
            title: title.to_string(),
            todo_adder: ElementAdder::new(id),
            todos,
            highlight: false,
        }
    }

    pub fn remove(&mut self, loc: &TreeLocation) -> Option<Todo> {
        if let TreeElement::Todo(i) = loc.element() {
            Some(self.todos.remove(*i))
        } else {
            None
        }
    }

    pub fn push(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn insert(&mut self, todo: Todo, index: usize) {
        self.todos.insert(index, todo);
    }

    pub fn move_todo(&mut self, from: &TreeLocation, to: &TreeLocation) {
        if let (TreeElement::Todo(i), TreeElement::Todo(j)) = (from.element(), to.element()) {
            let insert_index = if i < j { j - 1 } else { *j };
            let todo = self.todos.remove(*i);
            self.todos.insert(insert_index, todo);
        }
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    /// Convert the list into an element that iced can render
    fn view(&self, slot_index: usize) -> Element<Message> {
        let name = text(self.title.clone())
            .size(20)
            .style(theme::text::list_name);
        let location = TreeLocation::new(slot_index, TreeElement::List);
        let todos = column(
            self.todos
                .iter()
                .enumerate()
                .map(|(i, todo)| todo.view(TreeLocation::new(slot_index, TreeElement::Todo(i)))),
        )
        .spacing(10.0)
        .width(Length::Fill)
        .height(Length::Shrink)
        .push(self.adder(location));
        let content = container(column![name, todos].spacing(20.0))
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(10.0)
            .style(if self.highlight {
                theme::container::active_list
            } else {
                theme::container::list
            });
        droppable(content)
            .id(self.id.clone())
            .on_click(Message::StopEditingTodo)
            .on_drop(move |p, r| Message::DropList(location, p, r))
            .on_drag(move |p, r| Message::DragList(location, p, r))
            .on_cancel(Message::ListDropCanceled)
            .drag_hide(true)
            .into()
    }

    fn adder(&self, location: TreeLocation) -> Element<Message> {
        let input = text_input("Add task...", self.todo_adder.text.as_str())
            .id(self.todo_adder.id.clone())
            .on_input(move |new_str| Message::UpdateTodoWriter(location, new_str))
            .on_submit(Message::WriteTodo(location))
            .style(theme::text_input::element_adder)
            .size(14.0)
            .width(Length::Fill);
        let spacing = horizontal_space().width(Length::Fixed(10.0));
        let add_btn = tooltip(
            button(text("+").align_y(Center).align_x(Center))
                .on_press(Message::WriteTodo(location))
                .style(theme::button::adder)
                .width(Length::Fixed(30.0)),
            "Add task",
            tooltip::Position::FollowCursor,
        )
        .style(theme::container::adder_tooltip);
        row![input, spacing, add_btn].width(Length::Fill).into()
    }
}

static NEXT_TODO: AtomicUsize = AtomicUsize::new(0);

/// Some to-do task that can be dragged into a list
#[derive(Debug)]
pub struct Todo {
    pub content: String,
    pub editing: bool,
    id: Id,
    t_id: iced::widget::text_input::Id,
    highlight: bool,
}

impl Highlightable for Todo {
    fn set_highlight(&mut self, highlight: bool) {
        self.highlight = highlight;
    }
}

impl Todo {
    /// Create a new to-do task with some content
    pub fn new(content: &str) -> Self {
        let id = NEXT_TODO.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            id: Id::new(format!("todo_{}", id)),
            t_id: iced::widget::text_input::Id::new(format!("todo_input_{}", id)),
            content: content.to_string(),
            highlight: false,
            editing: false,
        }
    }

    /// Convert the task into an element that iced can render
    fn view(&self, location: TreeLocation) -> Element<Message> {
        let txt = text(&self.content)
            .size(15)
            .style(theme::text::todo)
            .align_y(Center);
        let content = container(txt)
            .align_y(alignment::Vertical::Center)
            .padding(10.0)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(if self.highlight {
                theme::container::active_todo
            } else {
                theme::container::todo
            });
        let element = if !self.editing {
            droppable(content)
                .id(self.id.clone())
                .on_click(Message::EditTodo(location, self.t_id.clone()))
                .on_drop(move |p, r| Message::DropTodo(location, p, r))
                .on_drag(move |p, r| Message::DragTodo(location, p, r))
                .on_cancel(Message::TodoDropCanceled)
                .drag_hide(true)
                .drag_size(Size::ZERO)
                .into()
        } else {
            text_input("", &self.content)
                .id(self.t_id.clone())
                .padding(10.0)
                .size(15)
                .on_input(move |new_str| Message::UpdateTodo(location, new_str))
                .on_submit(Message::StopEditingTodo)
                .into()
        };
        element
    }
}
