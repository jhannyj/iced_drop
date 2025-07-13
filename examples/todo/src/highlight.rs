//! Highlighting of droppable elements and drop zones
use iced::Rectangle;

use crate::tree::{TreeData, TreeElement, TreeLocation};

/// The state of the drag and drop highlight.
/// None means no highlight
#[derive(Clone, Default)]
pub struct Highlight {
    pub dragging: Option<(TreeLocation, Rectangle)>,
    pub hovered: Option<TreeLocation>,
}

/// Describes some ['Droppable'] that is to be highlighted
pub trait Highlightable {
    fn set_highlight(&mut self, highlight: bool);
}

/// Set the droppable to be highlighted
pub fn dragged(
    info: &Highlight,
    loc: TreeLocation,
    bounds: Rectangle,
) -> Highlight {
    Highlight {
        dragging: Some((loc, bounds)),
        ..info.clone()
    }
}

/// Determine if the current zone should be de-highlighted and if there is a new zone to be highlighted
pub fn zones_found(
    info: &Highlight,
    zones: &Vec<(TreeLocation, Rectangle)>,
) -> Highlight {
    let mut new_info = info.clone();

    if zones.is_empty() {
        new_info.hovered = None;
    }

    if let Some((_, bounds)) = info.dragging {
        let mut split_zones: [Vec<(TreeLocation, Rectangle)>; 2] =
            [vec![], vec![]];
        for zone in zones {
            let is_task = match zone.0.element() {
                TreeElement::Todo(_) => true,
                _ => false,
            };

            if is_task {
                split_zones[0].push(zone.clone());
            } else {
                split_zones[1].push(zone.clone());
            }
        }
        let valid_zones = if split_zones[0].is_empty() {
            &split_zones[1]
        } else {
            &split_zones[0]
        };
        if let Some((id, _)) = bigggest_intersect_area(valid_zones, &bounds) {
            new_info.hovered = Some(id.clone());
        }
    }
    new_info
}

/// De-highlight everything
pub fn dropped() -> Highlight {
    Highlight::default()
}

pub fn should_update_droppable(
    old_info: &Highlight,
    new_info: &Highlight,
    loc: &TreeLocation,
) -> bool {
    match &old_info.dragging {
        Some((d_id, _)) => *d_id == *loc,
        None => {
            if new_info.dragging.is_some() {
                true
            } else {
                false
            }
        }
    }
}

pub fn zone_update(old_info: &Highlight, new_info: &Highlight) -> ZoneUpdate {
    match &old_info.hovered {
        Some(o_id) => match &new_info.hovered {
            Some(n_id) => {
                if *o_id != *n_id {
                    ZoneUpdate::Replace
                } else {
                    ZoneUpdate::None
                }
            }
            None => ZoneUpdate::RemoveHighlight,
        },
        None => match &new_info.hovered {
            Some(_) => ZoneUpdate::Highlight,
            None => ZoneUpdate::None,
        },
    }
}

pub fn set_hovered(tree: &mut TreeData, info: &Highlight, highlight: bool) {
    if let Some(loc) = info.hovered.as_ref() {
        match loc.element() {
            &TreeElement::Slot => {
                tree.slot_mut(loc.slot()).set_highlight(highlight)
            }
            &TreeElement::List => tree.list_mut(&loc).set_highlight(highlight),
            &TreeElement::Todo(_) => {
                if let Some(task) = tree.todo_mut(&loc) {
                    task.set_highlight(highlight);
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ZoneUpdate {
    RemoveHighlight,
    Highlight,
    Replace,
    None,
}

impl ZoneUpdate {
    pub fn update(
        &self,
        tree: &mut TreeData,
        old_info: &Highlight,
        new_info: &Highlight,
    ) {
        match self {
            ZoneUpdate::RemoveHighlight => set_hovered(tree, old_info, false),
            ZoneUpdate::Highlight => set_hovered(tree, new_info, true),
            ZoneUpdate::Replace => {
                set_hovered(tree, old_info, false);
                set_hovered(tree, new_info, true);
            }
            ZoneUpdate::None => (),
        }
    }
}

/// Returns the id and area of the zone with the biggest intersection with the droppable rectangle
fn bigggest_intersect_area<'a>(
    zones: &'a Vec<(TreeLocation, Rectangle)>,
    droppable: &Rectangle,
) -> Option<(&'a TreeLocation, f32)> {
    zones
        .iter()
        .map(|(id, rect)| {
            (
                id,
                rect.intersection(&droppable)
                    .unwrap_or(Rectangle::default()),
            )
        })
        .map(|(id, rect)| (id, rect.area()))
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
}
