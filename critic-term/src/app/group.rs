use std::{cell::RefCell, i32, rc::Rc};

use critic::prelude::*;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use tui_input::{backend::crossterm::EventHandler, Input};

use super::AppTab;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Mode {
    #[default]
    Group,
    Criteria,
    EditGroup {
        id: i32,
    },
    EditCriteria {
        group_id: i32,
        id: i32,
    },
}

pub(super) fn edit_popup_area(area: Rect, percent_x: u16, size_y: u16) -> Rect {
    use ratatui::layout::Flex;
    let vertical = Layout::vertical([Constraint::Length(size_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn all_groups(conn: &Connection) -> Vec<CriteriaGroup> {
    if let Ok(groups) = conn.all_groups() {
        groups
    } else {
        vec![]
    }
}

fn criteria(conn: &Connection, id: i32) -> Vec<CriteriaGroupItem> {
    if let Ok(criteria) = conn.criteria(id) {
        criteria
    } else {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct GroupWidget {
    db: Rc<RefCell<Connection>>,
    groups: Vec<CriteriaGroup>,
    criteria: Vec<CriteriaGroupItem>,
    group_state: RefCell<ListState>,
    criteria_state: RefCell<ListState>,
    mode: Mode,
    input_state: Input,
}

impl GroupWidget {
    pub fn new(db: Rc<RefCell<Connection>>) -> Self {
        let (groups, first_group) = {
            let groups = all_groups(&db.borrow());
            let first_id: i32 = groups.first().map(|x| x.id).or(Some(i32::MAX)).unwrap();
            let criteria = criteria(&db.borrow(), first_id);
            (groups, criteria)
        };

        let group_state: RefCell<ListState> = RefCell::default();
        let criteria_state: RefCell<ListState> = RefCell::default();
        group_state.borrow_mut().select_first();
        criteria_state.borrow_mut().select_first();

        Self {
            db,
            groups,
            criteria: first_group,
            group_state,
            criteria_state,
            mode: Mode::Group,
            input_state: Input::default(),
        }
    }
}

impl AppTab for GroupWidget {
    fn render(&self, area: Rect, frame: &mut ratatui::Frame) {
        let areas = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let group_items = self
            .groups
            .iter()
            .map(|x| ListItem::new(x.name.as_str()))
            .collect::<Vec<ListItem>>();

        let groups = List::new(group_items)
            .block(Block::default().borders(Borders::ALL).title("Groups"))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        let criteria_items = self
            .criteria
            .iter()
            .map(|x| ListItem::new(x.name.as_str()))
            .collect::<Vec<ListItem>>();

        let criteria = List::new(criteria_items)
            .block(Block::default().borders(Borders::ALL).title("Criteria"))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        let mut group_state: ListState = self.group_state.borrow().clone();

        frame.render_stateful_widget(groups, areas[0], &mut group_state);
        *self.group_state.borrow_mut() = group_state;

        let mut criteria_state: ListState = self.criteria_state.borrow().clone();
        frame.render_stateful_widget(criteria, areas[1], &mut criteria_state);
        *self.criteria_state.borrow_mut() = criteria_state;

        match self.mode {
            Mode::EditGroup { id: _ } | Mode::EditCriteria { group_id: _, id: _ } => {
                let block = Block::bordered()
                    .title("Edit".bold().into_centered_line())
                    .title_bottom(
                        Line::from(vec![
                            "Save ".into(),
                            "<CTRL-S> ".blue().bold(),
                            "Back ".into(),
                            "<ESC>".blue().bold(),
                        ])
                        .centered(),
                    );
                let area = edit_popup_area(area, 60, 3);
                frame.render_widget(Clear::default(), area);
                let input = Paragraph::new(self.input_state.value()).block(block);
                {
                    let width = area.width.max(3) - 3; // Keep 2 for borders and 1 for cursor
                    let scroll = self.input_state.visual_scroll(width as usize);
                    frame.set_cursor_position((
                        area.x + 1 + (self.input_state.visual_cursor().max(scroll) - scroll) as u16,
                        area.y + 1,
                    ));
                }
                frame.render_widget(input, area);
            }
            _ => {}
        }
    }

    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        match self.mode {
            Mode::Group => {
                let group_id = self.group_state.borrow().selected();
                match evt.code {
                    KeyCode::Up => {
                        self.group_state.borrow_mut().select_previous();
                    }
                    KeyCode::Down => {
                        self.group_state.borrow_mut().select_next();
                    }
                    KeyCode::Right => {
                        self.mode = Mode::Criteria;
                    }
                    KeyCode::Char('e') => {
                        if !self.groups.is_empty() {
                            let (id, value) = {
                                let idx = self.group_state.borrow().selected().unwrap();
                                (self.groups[idx].id, self.groups[idx].name.as_str())
                            };

                            self.mode = Mode::EditGroup { id };
                            self.input_state = Input::new(value.to_string());
                        }
                    }
                    _ => {}
                }

                let post_group_id = self.group_state.borrow().selected();
                if group_id != post_group_id && post_group_id.is_some() {
                    let db = self.db.borrow();
                    let db = &*db;
                    let idx = post_group_id.unwrap();
                    if idx < self.groups.len() {
                        let group = &self.groups[idx];
                        self.criteria = criteria(&db, group.id);
                        self.criteria_state.borrow_mut().select_first();
                    }
                }
            }
            Mode::Criteria => match evt.code {
                KeyCode::Up => {
                    self.criteria_state.borrow_mut().select_previous();
                }
                KeyCode::Down => {
                    self.criteria_state.borrow_mut().select_next();
                }
                KeyCode::Left => {
                    self.mode = Mode::Group;
                }
                KeyCode::Char('e') => {
                    let group_id = {
                        let idx = self.group_state.borrow().selected().unwrap();
                        self.groups[idx].id
                    };
                    let (id, value) = {
                        let idx = self.criteria_state.borrow().selected().unwrap();
                        (self.criteria[idx].id, self.criteria[idx].name.as_str())
                    };

                    self.mode = Mode::EditCriteria { group_id, id };
                    self.input_state = Input::new(value.to_string());
                }
                _ => {}
            },
            Mode::EditGroup { id } => match (evt.code, evt.modifiers) {
                (KeyCode::Esc, _) => {
                    self.mode = Mode::Group;
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let name = self.input_state.value();

                    let request = UpdateCriteriaGroup {
                        id,
                        name: name.to_string(),
                    };

                    conn.save(&request)?;

                    self.groups = all_groups(&conn);
                    self.criteria = criteria(&conn, id);
                    let idx = self.groups.iter().position(|x| x.id == id);
                    *self.group_state.borrow_mut().selected_mut() = idx;
                    self.mode = Mode::Group;
                }
                _ => {
                    self.input_state.handle_event(&Event::Key(*evt));
                }
            },
            Mode::EditCriteria { group_id, id } => match (evt.code, evt.modifiers) {
                (KeyCode::Esc, _) => {
                    self.mode = Mode::Criteria;
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let name = self.input_state.value();
                    let request = UpdateCriterion {
                        id,
                        name: name.to_string(),
                    };

                    conn.save(&request)?;

                    self.criteria = criteria(&conn, group_id);
                    let idx = self.criteria.iter().position(|x| x.id == id);
                    *self.criteria_state.borrow_mut().selected_mut() = idx;
                    self.mode = Mode::Criteria;
                }
                _ => {
                    self.input_state.handle_event(&Event::Key(*evt));
                }
            },
        }

        Ok(())
    }
}
