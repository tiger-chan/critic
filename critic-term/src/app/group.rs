use std::{cell::RefCell, i32, rc::Rc};

use critic::{
    dto::{GroupAddToTiles, NewTitleCriteria},
    prelude::*,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{modal_input_single_line, popup_area, theme, AppTab};

#[derive(Debug, Default, PartialEq, Clone)]
enum Mode {
    #[default]
    Group,
    Criteria {
        group_id: i32,
    },
    EditGroup {
        id: i32,
    },
    EditCriteria {
        group_id: i32,
        id: i32,
    },
    NewGroup,
    NewCriteria {
        group_id: i32,
    },
    DeleteGroup {
        id: i32,
    },
    DeleteCriteria {
        group_id: i32,
        id: i32,
    },
    PushGroupToAll {
        id: i32,
    },
    EditGroupsForTitles {
        id: i32,
        curr: Vec<bool>,
        edit: Vec<bool>,
        titles: Vec<Title>,
        state: RefCell<ListState>,
    },
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

        let (g_selected, g_unselected) = {
            match self.mode {
                Mode::Group | Mode::EditGroup { id: _ } | Mode::NewGroup => {
                    (theme::HIGHLIGHT, theme::DEFAULT)
                }
                _ => (theme::DEFAULT, theme::HIGHLIGHT),
            }
        };

        let group_items = self
            .groups
            .iter()
            .map(|x| ListItem::new(x.name.as_str()))
            .collect::<Vec<ListItem>>();

        let group_block = Block::default()
            .borders(Borders::ALL)
            .fg(g_selected)
            .title("Groups");

        let groups = List::new(group_items)
            .block(group_block)
            .fg(theme::DEFAULT)
            .highlight_style(theme::HIGHLIGHT)
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        let criteria_items = self
            .criteria
            .iter()
            .map(|x| ListItem::new(x.name.as_str()))
            .collect::<Vec<ListItem>>();

        let criteria_block = Block::default()
            .borders(Borders::ALL)
            .fg(g_unselected)
            .title("Criteria");

        let criteria = List::new(criteria_items)
            .block(criteria_block)
            .fg(theme::DEFAULT)
            .highlight_style(theme::HIGHLIGHT)
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        let mut group_state: ListState = self.group_state.borrow().clone();

        frame.render_stateful_widget(groups, areas[0], &mut group_state);
        *self.group_state.borrow_mut() = group_state;

        let mut criteria_state: ListState = self.criteria_state.borrow().clone();
        frame.render_stateful_widget(criteria, areas[1], &mut criteria_state);
        *self.criteria_state.borrow_mut() = criteria_state;

        match &self.mode {
            Mode::NewGroup => {
                modal_input_single_line("New Group", area, &self.input_state, frame);
            }
            Mode::NewCriteria { group_id: _ } => {
                modal_input_single_line("New Criterion", area, &self.input_state, frame);
            }
            Mode::EditGroup { id: _ } => {
                modal_input_single_line("Edit Group", area, &self.input_state, frame);
            }
            Mode::EditCriteria { group_id: _, id: _ } => {
                modal_input_single_line("Edit Criterion", area, &self.input_state, frame);
            }
            Mode::DeleteGroup { id: _ } => {
                let area = popup_area(area, 50, 50);
                frame.render_widget(Clear::default(), area);

                let text = vec![
                    Line::from("Are your sure you want to delete the group?").centered(),
                    Line::from(vec![
                        Span::styled("[Y]es", theme::HINT),
                        Span::styled("[N]o", theme::HINT),
                    ])
                    .centered(),
                ];

                frame.render_widget(
                    Paragraph::new(text)
                        .block(Block::default().borders(Borders::ALL).title("Delete Group")),
                    area,
                );
            }
            Mode::DeleteCriteria { group_id: _, id: _ } => {
                let area = popup_area(area, 50, 50);
                frame.render_widget(Clear::default(), area);
                let text = vec![
                    Line::from("Are your sure you want to delete the group?").centered(),
                    Line::from(vec![
                        Span::styled("[Y]es", theme::HINT),
                        Span::styled("[N]o", theme::HINT),
                    ])
                    .centered(),
                ];

                frame.render_widget(
                    Paragraph::new(text).block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Delete Criterion"),
                    ),
                    area,
                );
            }
            Mode::PushGroupToAll { id: _ } => {
                let area = popup_area(area, 50, 50);
                frame.render_widget(Clear::default(), area);
                let text = vec![
                    Line::from("Are your sure you want to push the group to all titles?")
                        .centered(),
                    Line::from(vec![
                        Span::styled("[Y]es", theme::HINT),
                        Span::styled("[N]o", theme::HINT),
                    ])
                    .centered(),
                ];

                frame.render_widget(
                    Paragraph::new(text).block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Add Group to All Titles"),
                    ),
                    area,
                );
            }
            Mode::EditGroupsForTitles {
                edit,
                titles,
                state,
                ..
            } => {
                let area = popup_area(area, 50, 80);
                frame.render_widget(Clear::default(), area);

                let list_items: Vec<ListItem> = titles
                    .iter()
                    .zip(edit)
                    .map(|(x, y)| {
                        ListItem::new(if *y {
                            format!("[x] {}", x.name)
                        } else {
                            format!("[ ] {}", x.name)
                        })
                    })
                    .collect();

                let block = Block::default()
                    .borders(Borders::ALL)
                    .fg(theme::HIGHLIGHT)
                    .title("Groups");

                let list = List::new(list_items)
                    .block(block)
                    .fg(theme::DEFAULT)
                    .highlight_style(theme::HIGHLIGHT);

                let mut tmp_state: ListState = state.borrow().clone();
                frame.render_stateful_widget(list, area, &mut tmp_state);
                *state.borrow_mut() = tmp_state;
            }
            _ => {}
        }
    }

    fn render_footer(&self, area: Rect, frame: &mut ratatui::Frame) {
        let help = Paragraph::new(
            Line::from(vec![
                Span::styled(" [^a]", theme::HINT),
                " Add".into(),
                Span::styled(" [e]", theme::HINT),
                " Edit".into(),
                Span::styled(" [^e]", theme::HINT),
                " Edit Titles".into(),
                Span::styled(" [^p]", theme::HINT),
                " Push".into(),
                Span::styled(" [^d]", theme::HINT),
                " Delete ".into(),
            ])
            .left_aligned(),
        );
        frame.render_widget(help, area);
    }

    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        match &mut self.mode {
            Mode::Group => {
                let group_id = self.group_state.borrow().selected();
                match (evt.code, evt.modifiers) {
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        let id = {
                            let idx = self.group_state.borrow().selected().unwrap();
                            self.groups[idx].id
                        };
                        self.mode = Mode::DeleteGroup { id };
                    }
                    (KeyCode::Up | KeyCode::Char('w'), _) => {
                        self.group_state.borrow_mut().select_previous();
                    }
                    (KeyCode::Down | KeyCode::Char('s'), _) => {
                        self.group_state.borrow_mut().select_next();
                    }
                    (KeyCode::Right | KeyCode::Char('d'), _) => {
                        if !self.groups.is_empty() {
                            let group_id = {
                                let idx = self.group_state.borrow().selected().unwrap();
                                self.groups[idx].id
                            };
                            self.mode = Mode::Criteria { group_id };
                        }
                    }
                    (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                        self.input_state = Input::default();
                        self.mode = Mode::NewGroup;
                    }
                    (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                        if !self.groups.is_empty() {
                            let id = {
                                let idx = self.group_state.borrow().selected().unwrap();
                                self.groups[idx].id
                            };
                            self.mode = Mode::PushGroupToAll { id };
                        }
                    }
                    (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                        if !self.groups.is_empty() {
                            let id = {
                                let idx = self.group_state.borrow().selected().unwrap();
                                self.groups[idx].id
                            };
                            let mut db = self.db.borrow_mut();
                            let conn = &mut *db;

                            let titles = conn
                                .all_titles()
                                .expect("Only invalid if the connection is bad");

                            let titles_in_group = conn
                                .titles_in_group(id)
                                .expect("Only invalid if the connection is bad");

                            let curr = titles
                                .iter()
                                .map(|x| titles_in_group.contains(x))
                                .collect::<Vec<bool>>();

                            let mut state = ListState::default();
                            state.select_first();

                            self.mode = Mode::EditGroupsForTitles {
                                id,
                                edit: curr.clone(),
                                curr,
                                titles,
                                state: RefCell::new(state),
                            }
                        }
                    }
                    (KeyCode::Char('e'), _) => {
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
            Mode::Criteria { group_id } => match (evt.code, evt.modifiers) {
                (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                    self.input_state = Input::default();
                    self.mode = Mode::NewCriteria {
                        group_id: *group_id,
                    };
                }
                (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    let id = {
                        let idx = self.criteria_state.borrow().selected().unwrap();
                        self.criteria[idx].id
                    };
                    self.mode = Mode::DeleteCriteria {
                        group_id: *group_id,
                        id,
                    };
                }
                (KeyCode::Up | KeyCode::Char('w'), _) => {
                    self.criteria_state.borrow_mut().select_previous();
                }
                (KeyCode::Down | KeyCode::Char('s'), _) => {
                    self.criteria_state.borrow_mut().select_next();
                }
                (KeyCode::Left | KeyCode::Char('a'), _) => {
                    self.mode = Mode::Group;
                }
                (KeyCode::Char('e'), _) => {
                    if !self.criteria.is_empty() {
                        let (id, value) = {
                            let idx = self.criteria_state.borrow().selected().unwrap();
                            (self.criteria[idx].id, self.criteria[idx].name.as_str())
                        };

                        self.mode = Mode::EditCriteria {
                            group_id: *group_id,
                            id,
                        };
                        self.input_state = Input::new(value.to_string());
                    }
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
                        id: *id,
                        name: name.to_string(),
                    };

                    conn.save(&request)?;

                    self.groups = all_groups(&conn);
                    self.criteria = criteria(&conn, *id);
                    let idx = self.groups.iter().position(|x| &x.id == id);
                    *self.group_state.borrow_mut().selected_mut() = idx;
                    self.mode = Mode::Group;
                }
                _ => {
                    self.input_state.handle_event(&Event::Key(*evt));
                    return Ok(true);
                }
            },
            Mode::EditCriteria { group_id, id } => match (evt.code, evt.modifiers) {
                (KeyCode::Esc, _) => {
                    self.mode = Mode::Criteria {
                        group_id: *group_id,
                    };
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let name = self.input_state.value();
                    let request = UpdateCriterion {
                        id: *id,
                        name: name.to_string(),
                    };

                    conn.save(&request)?;

                    self.criteria = criteria(&conn, *group_id);
                    let idx = self.criteria.iter().position(|x| &x.id == id);
                    *self.criteria_state.borrow_mut().selected_mut() = idx;
                    self.mode = Mode::Criteria {
                        group_id: *group_id,
                    };
                }
                _ => {
                    self.input_state.handle_event(&Event::Key(*evt));
                    return Ok(true);
                }
            },
            Mode::NewGroup => match (evt.code, evt.modifiers) {
                (KeyCode::Esc, _) => {
                    self.mode = Mode::Group;
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let name = self.input_state.value();
                    let request = NewCriteriaGroup {
                        name: name.to_string(),
                    };

                    if let Ok(id) = conn.save(&request) {
                        self.groups = all_groups(&conn);
                        self.criteria = criteria(&conn, id as i32);
                        let idx = self.groups.iter().position(|x| x.id == id as i32);
                        *self.group_state.borrow_mut().selected_mut() = idx;
                        self.criteria_state.borrow_mut().select_first();
                        self.mode = Mode::Group;
                    }
                }
                _ => {
                    self.input_state.handle_event(&Event::Key(*evt));
                    return Ok(true);
                }
            },
            Mode::NewCriteria { group_id } => match (evt.code, evt.modifiers) {
                (KeyCode::Esc, _) => {
                    self.mode = Mode::Criteria {
                        group_id: *group_id,
                    };
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let name = self.input_state.value();
                    let request = NewCriterion {
                        group: *group_id,
                        name: name.to_string(),
                    };

                    if let Ok(_) = conn.save(&request) {
                        self.criteria = criteria(&conn, *group_id);
                        let idx = self.criteria.iter().position(|x| x.name == name);
                        *self.criteria_state.borrow_mut().selected_mut() = idx;
                        self.mode = Mode::Group;
                    }
                }
                _ => {
                    self.input_state.handle_event(&Event::Key(*evt));
                    return Ok(true);
                }
            },
            Mode::DeleteGroup { id } => match evt.code {
                KeyCode::Char('y') => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let request = DeleteCriteriaGroup { id: *id };

                    let idx = self.group_state.borrow().selected().unwrap();
                    conn.save(&request)?;
                    self.groups = all_groups(&conn);
                    if idx < self.groups.len() {
                        let id = self.groups[idx].id;
                        self.criteria = criteria(&conn, id);
                    } else if !self.groups.is_empty() {
                        self.group_state.borrow_mut().select_last();
                        let id = self.groups.last().unwrap().id;
                        self.criteria = criteria(&conn, id);
                    } else {
                        self.criteria.clear();
                    }
                    self.criteria_state.borrow_mut().select_first();
                    self.mode = Mode::Group;
                }
                KeyCode::Esc | KeyCode::Char('n') => {
                    self.mode = Mode::Group;
                }
                _ => {}
            },
            Mode::DeleteCriteria { group_id, id } => match evt.code {
                KeyCode::Char('y') => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let request = DeleteCriterion { id: *id };

                    let idx = self.criteria_state.borrow().selected().unwrap();
                    if let Ok(_) = conn.save(&request) {
                        self.criteria = criteria(&conn, *group_id);
                        if idx >= self.criteria.len() {
                            self.criteria_state.borrow_mut().select_last();
                        }
                        self.mode = Mode::Criteria {
                            group_id: *group_id,
                        };
                    }
                }
                KeyCode::Esc | KeyCode::Char('n') => {
                    self.mode = Mode::Group;
                }
                _ => {}
            },
            Mode::PushGroupToAll { id } => match evt.code {
                KeyCode::Char('y') => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let request = GroupAddToTiles { id: *id };
                    conn.save(&request)?;
                    self.mode = Mode::Group;
                }
                KeyCode::Esc | KeyCode::Char('n') => {
                    self.mode = Mode::Group;
                }
                _ => {}
            },
            Mode::EditGroupsForTitles {
                id,
                state,
                curr,
                edit,
                titles,
                ..
            } => match (evt.code, evt.modifiers) {
                (KeyCode::Esc, _) => {
                    self.mode = Mode::Group;
                }
                (KeyCode::Enter | KeyCode::Char(' '), _) => {
                    if let Some(idx) = state.borrow_mut().selected() {
                        edit[idx] = !edit[idx];
                    }
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    for (y, t) in curr
                        .iter()
                        .zip(edit)
                        .zip(titles)
                        .filter(|((x, y), _)| x != y)
                        .map(|((_, y), t)| (*y, t))
                    {
                        if y {
                            let req = NewTitleCriteria {
                                criteria: *id,
                                title: t.id,
                            };

                            conn.save(&req)?;
                        } else {
                            let req = DeleteTitleCriteria {
                                criteria: *id,
                                title: t.id,
                            };

                            conn.save(&req)?;
                        }
                    }

                    self.mode = Mode::Group;
                }
                (KeyCode::Up | KeyCode::Char('w'), _) => {
                    state.borrow_mut().select_previous();
                }
                (KeyCode::Down | KeyCode::Char('s'), _) => {
                    state.borrow_mut().select_next();
                }
                (KeyCode::PageUp, _) => {
                    state.borrow_mut().scroll_up_by(10);
                }
                (KeyCode::PageDown, _) => {
                    state.borrow_mut().scroll_down_by(10);
                }
                _ => {}
            },
        }

        Ok(false)
    }
}
