use std::sync::atomic::AtomicUsize;

use iced::advanced::widget::Id;

use iced::widget::tooltip;
use iced::{
    alignment,
    widget::{button, column, container, horizontal_space, row, text, text_input},
    Element, Length, Size,
};
use iced_drop::droppable;

use crate::{highlight::Highlightable, theme, Message};

pub const NULL_TASK_LOC: TreeLocation = TreeLocation {
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
    Task(usize),
}

pub struct ElementAdder {
    pub text: String,
    id: iced::widget::text_input::Id,
}

/// Contains tasks organized by slots, and lists
pub struct TreeData {
    slots: Vec<Slot>,
}

impl TreeData {
    pub fn new(slots: Vec<Slot>) -> Self {
        Self { slots }
    }
    /// Convert the tree into an element that iced can render
    pub fn view(&self) -> Element<Message, theme::Board, iced::Renderer> {
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
            for (j, list) in slot.list.tasks.iter().enumerate() {
                if list.id == *id {
                    return Some(TreeLocation::new(i, TreeElement::Task(j)));
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
            TreeElement::Task(_) => &mut self.slots[i].list,
        }
    }

    pub fn task(&self, location: &TreeLocation) -> Option<&Task> {
        let i = location.slot;
        match location.element {
            TreeElement::Slot => None,
            TreeElement::List => None,
            TreeElement::Task(j) => Some(&self.slots[i].list.tasks[j]),
        }
    }

    pub fn task_mut(&mut self, location: &TreeLocation) -> Option<&mut Task> {
        let i = location.slot;
        match location.element {
            TreeElement::Slot => None,
            TreeElement::List => None,
            TreeElement::Task(j) => Some(&mut self.slots[i].list.tasks[j]),
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

    /// Returns the widget Id of all the widgets wich a task can be dropped on
    pub fn task_options(&self, t_loc: &TreeLocation) -> Vec<Id> {
        let task_id = if let Some(task) = self.task(t_loc) {
            task.id.clone()
        } else {
            return vec![];
        };
        self.slots
            .iter()
            .map(|slot| {
                slot.list.tasks.iter().filter_map(|task| {
                    if task.id != task_id {
                        Some(task.id.clone())
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
    fn view(&self, index: usize) -> Element<Message, theme::Board, iced::Renderer> {
        container(self.list.view(index))
            .id(self.c_id.clone())
            .style(if self.highlight {
                theme::Container::ActiveSlot
            } else {
                theme::Container::Default
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
            id: iced::widget::text_input::Id::new(format!("task_adder_{}", id)),
            text: String::new(),
        }
    }

    pub fn id(&self) -> iced::widget::text_input::Id {
        self.id.clone()
    }
}

static NEXT_LIST: AtomicUsize = AtomicUsize::new(0);

/// Some list that contains tasks and can be dragged into a slot. Tasks can also be dragged into a list.
pub struct List {
    pub task_adder: ElementAdder,
    id: Id,
    title: String,
    tasks: Vec<Task>,
    highlight: bool,
}

impl Highlightable for List {
    fn set_highlight(&mut self, highlight: bool) {
        self.highlight = highlight;
    }
}

impl List {
    /// Create a new list with a title
    pub fn new(title: &str, tasks: Vec<Task>) -> Self {
        let id = NEXT_LIST.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            id: Id::new(format!("list_{}", id)),
            title: title.to_string(),
            task_adder: ElementAdder::new(id),
            tasks,
            highlight: false,
        }
    }

    pub fn remove_task(&mut self, loc: &TreeLocation) -> Option<Task> {
        if let TreeElement::Task(i) = loc.element() {
            Some(self.tasks.remove(*i))
        } else {
            None
        }
    }

    pub fn push_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn inser_task(&mut self, task: Task, index: usize) {
        self.tasks.insert(index, task);
    }

    pub fn move_task(&mut self, from: &TreeLocation, to: &TreeLocation) {
        if let (TreeElement::Task(i), TreeElement::Task(j)) = (from.element(), to.element()) {
            let insert_index = if i < j { j - 1 } else { *j };
            let task = self.tasks.remove(*i);
            self.tasks.insert(insert_index, task);
        }
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    /// Convert the list into an element that iced can render
    fn view(&self, slot_index: usize) -> Element<Message, theme::Board, iced::Renderer> {
        let name = text(self.title.clone())
            .size(20)
            .style(theme::Text::ListName);
        let location = TreeLocation::new(slot_index, TreeElement::List);
        let tasks = column(
            self.tasks
                .iter()
                .enumerate()
                .map(|(i, task)| task.view(TreeLocation::new(slot_index, TreeElement::Task(i)))),
        )
        .spacing(10.0)
        .width(Length::Fill)
        .height(Length::Shrink)
        .push(self.task_adder(location));
        let content = container(column![name, tasks].spacing(20.0))
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(10.0)
            .style(if self.highlight {
                theme::Container::ActiveList
            } else {
                theme::Container::List
            });
        droppable(content)
            .id(self.id.clone())
            .on_click(Message::StopEditingTask)
            .on_drop(move |p, r| Message::DropList(location, p, r))
            .on_drag(move |p, r| Message::DragList(location, p, r))
            .on_cancel(Message::ListDropCanceled)
            .drag_hide(true)
            .into()
    }

    fn task_adder(&self, location: TreeLocation) -> Element<Message, theme::Board, iced::Renderer> {
        let input = text_input("Add task...", self.task_adder.text.as_str())
            .id(self.task_adder.id.clone())
            .on_input(move |new_str| Message::UpdateTaskWriter(location, new_str))
            .on_submit(Message::WriteTask(location))
            .style(theme::TextInput::ElementAdder)
            .size(14.0)
            .width(Length::Fill);
        let spacing = horizontal_space(Length::Fixed(10.0));
        let add_btn = tooltip(
            button(
                text("+")
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .on_press(Message::WriteTask(location))
            .style(theme::Button::Adder)
            .width(Length::Fixed(30.0)),
            "Add task",
            tooltip::Position::FollowCursor,
        )
        .style(theme::Container::TaskAdderTooltip);
        row![input, spacing, add_btn].width(Length::Fill).into()
    }
}

static NEXT_TASK: AtomicUsize = AtomicUsize::new(0);

/// Some task that can be dragged into a list
#[derive(Debug)]
pub struct Task {
    pub content: String,
    pub editing: bool,
    id: Id,
    t_id: iced::widget::text_input::Id,
    highlight: bool,
}

impl Highlightable for Task {
    fn set_highlight(&mut self, highlight: bool) {
        self.highlight = highlight;
    }
}

impl Task {
    /// Create a new task with some content
    pub fn new(content: &str) -> Self {
        let id = NEXT_TASK.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            id: Id::new(format!("task_{}", id)),
            t_id: iced::widget::text_input::Id::new(format!("task_input_{}", id)),
            content: content.to_string(),
            highlight: false,
            editing: false,
        }
    }

    /// Convert the task into an element that iced can render
    fn view(&self, location: TreeLocation) -> Element<Message, theme::Board, iced::Renderer> {
        let txt: iced::advanced::widget::Text<'_, theme::Board, iced::Renderer> =
            text(&self.content)
                .size(15)
                .style(theme::Text::Task)
                .vertical_alignment(alignment::Vertical::Center);
        let content = container(txt)
            .align_y(alignment::Vertical::Center)
            .padding(10.0)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(if self.highlight {
                theme::Container::ActiveTask
            } else {
                theme::Container::Task
            });
        let element = if !self.editing {
            droppable(content)
                .id(self.id.clone())
                .on_click(Message::EditTask(location, self.t_id.clone()))
                .on_drop(move |p, r| Message::DropTask(location, p, r))
                .on_drag(move |p, r| Message::DragTask(location, p, r))
                .on_cancel(Message::TaskDropCanceled)
                .drag_hide(true)
                .drag_size(Size::ZERO)
                .into()
        } else {
            text_input("", &self.content)
                .id(self.t_id.clone())
                .padding(10.0)
                .size(15)
                .on_input(move |new_str| Message::UpdateTask(location, new_str))
                .on_submit(Message::StopEditingTask)
                .into()
        };
        element
    }
}
