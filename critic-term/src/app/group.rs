use std::cell::RefCell;

use critic::prelude::*;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use super::AppTab;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Mode {
    #[default]
    Group,
    Criteria,
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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupWidget {
    db: String,
    groups: Vec<CriteriaGroup>,
    criteria: Vec<CriteriaGroupItem>,
    group_state: RefCell<ListState>,
    criteria_state: RefCell<ListState>,
    mode: Mode,
}

impl GroupWidget {
    pub fn new<T: AsRef<str>>(db: T) -> Self {
        let (groups, first_group) = if let Ok(conn) = Connection::open_category(db.as_ref()) {
            let groups = all_groups(&conn);
            let first_id: i32 = groups.first().map(|x| x.id).or(Some(i32::MAX)).unwrap();
            let criteria = criteria(&conn, first_id);
            (groups, criteria)
        } else {
            (vec![], vec![])
        };

        let group_state: RefCell<ListState> = RefCell::default();
        group_state.borrow_mut().select_first();

        Self {
            db: db.as_ref().to_string(),
            groups,
            criteria: first_group,
            group_state,
            criteria_state: RefCell::default(),
            mode: Mode::Group,
        }
    }
}

impl AppTab for GroupWidget {
    fn render(&self, block: Block, area: Rect, buf: &mut Buffer) {
        Paragraph::default().block(block).render(area, buf);

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
        StatefulWidget::render(groups, areas[0], buf, &mut group_state);
        *self.group_state.borrow_mut() = group_state;

        let mut criteria_state: ListState = self.criteria_state.borrow().clone();
        StatefulWidget::render(criteria, areas[1], buf, &mut criteria_state);
        *self.criteria_state.borrow_mut() = criteria_state;
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
                    _ => {}
                }

                let post_group_id = self.group_state.borrow().selected();
                if group_id != post_group_id && post_group_id.is_some() {
                    if let Ok(conn) = Connection::open_category(self.db.as_str()) {
                        let idx = post_group_id.unwrap();
                        if idx < self.groups.len() {
                            let group = &self.groups[idx];
                            self.criteria = criteria(&conn, group.id);
                            self.criteria_state.borrow_mut().select_first();
                        }
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
                _ => {}
            },
        }

        Ok(())
    }
}
