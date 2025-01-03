use std::{cell::RefCell, rc::Rc};

use critic::{
    dto::{
        self, CriteriaGroup, DeleteTitle, DeleteTitleCriteria, NewTitle, NewTitleCriteria, Title,
        UpdateTitle,
    },
    prelude::Connection,
    CriticData, DbConnection,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{modal_input_single_line, popup_area, theme, AppTab};

#[derive(Debug, Default, Clone)]
enum Mode {
    #[default]
    Title,
    Group {
        title_id: i32,
    },
    NewTitle {
        state: Input,
    },
    EditTitle {
        id: i32,
        state: Input,
    },
    DeleteTitle {
        id: i32,
    },
    EditGroups {
        title_id: i32,
        id: i32,
        cur: Vec<bool>,
        edit: Vec<bool>,
        all_groups: Vec<dto::CriteriaGroup>,
        state: RefCell<ListState>,
    },
}

#[derive(Debug, Clone)]
pub struct TitleWidget {
    db: Rc<RefCell<Connection>>,
    mode: Mode,
    titles: Vec<dto::Title>,
    groups: Vec<dto::CriteriaGroup>,
    titles_state: RefCell<ListState>,
    group_state: RefCell<ListState>,
}

impl TitleWidget {
    pub fn new(db: Rc<RefCell<Connection>>) -> Self {
        let (titles, groups) = {
            let titles = all_titles(&db.borrow());
            let first_id: i32 = titles.first().map(|x| x.id).or(Some(i32::MAX)).unwrap();
            let groups = groups_by_title(&db.borrow(), first_id);
            (titles, groups)
        };

        let titles_state: RefCell<ListState> = RefCell::default();
        let group_state: RefCell<ListState> = RefCell::default();
        titles_state.borrow_mut().select_first();
        group_state.borrow_mut().select_first();

        Self {
            db,
            mode: Mode::default(),
            titles,
            groups,
            titles_state,
            group_state,
        }
    }
}

impl AppTab for TitleWidget {
    fn render(&self, area: Rect, frame: &mut Frame) {
        let areas = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let (g_selected, g_unselected) = {
            match self.mode {
                Mode::Group { title_id: _ } => (theme::HIGHLIGHT, theme::DEFAULT),
                _ => (theme::DEFAULT, theme::HIGHLIGHT),
            }
        };

        let titles_items = self
            .titles
            .iter()
            .map(|x| ListItem::new(x.name.as_str()))
            .collect::<Vec<ListItem>>();

        let titles_block = Block::default()
            .borders(Borders::ALL)
            .fg(g_unselected)
            .title("Title");

        let titles = List::new(titles_items)
            .block(titles_block)
            .fg(theme::DEFAULT)
            .highlight_style(theme::HIGHLIGHT)
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

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

        let mut titles_state: ListState = self.titles_state.borrow().clone();
        frame.render_stateful_widget(titles, areas[0], &mut titles_state);
        *self.titles_state.borrow_mut() = titles_state;

        let mut group_state: ListState = self.group_state.borrow().clone();
        frame.render_stateful_widget(groups, areas[1], &mut group_state);
        *self.group_state.borrow_mut() = group_state;

        match &self.mode {
            Mode::NewTitle { state } => {
                modal_input_single_line("Add Title", area, &state, frame);
            }
            Mode::EditTitle { state, .. } => {
                modal_input_single_line("Edit Title", area, &state, frame);
            }
            Mode::DeleteTitle { .. } => {
                let area = popup_area(area, 50, 50);
                frame.render_widget(Clear::default(), area);

                let text = vec![
                    Line::from("Are your sure you want to delete the Title?").centered(),
                    Line::from(vec!["[Y]es".blue().bold(), "[N]o".blue().bold()]).centered(),
                ];

                frame.render_widget(
                    Paragraph::new(text)
                        .block(Block::default().borders(Borders::ALL).title("Delete Title")),
                    area,
                );
            }
            Mode::EditGroups {
                edit,
                all_groups: groups,
                state,
                ..
            } => {
                let area = popup_area(area, 50, 80);
                frame.render_widget(Clear::default(), area);

                let list_items: Vec<ListItem> = groups
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

    fn render_footer(&self, area: Rect, frame: &mut Frame) {
        let help = match &self.mode {
            Mode::Group { .. } | Mode::EditGroups { .. } => {
                Paragraph::new(Line::from(vec![" Edit ".into(), "<e>".blue().bold()]))
                    .right_aligned()
            }
            _ => Paragraph::new(
                Line::from(vec![
                    " [^a] ".blue().bold(),
                    " Add ".into(),
                    "[^d]".blue().bold(),
                    " Delete ".into(),
                ])
                .left_aligned(),
            ),
        };
        frame.render_widget(help, area);
    }

    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        match &mut self.mode {
            Mode::Title => {
                let title_id = self.titles_state.borrow().selected();
                match (evt.code, evt.modifiers) {
                    (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                        self.mode = Mode::NewTitle {
                            state: Input::default(),
                        };
                    }
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        let id = {
                            let idx = self.titles_state.borrow().selected().unwrap();
                            self.titles[idx].id
                        };
                        self.mode = Mode::DeleteTitle { id };
                    }
                    (KeyCode::Char('e'), _) => {
                        let (id, value) = {
                            let idx = self.titles_state.borrow().selected().unwrap();
                            (self.titles[idx].id, self.titles[idx].name.as_str())
                        };

                        self.mode = Mode::EditTitle {
                            id,
                            state: Input::new(value.to_string()),
                        };
                    }
                    (KeyCode::Up | KeyCode::Char('w'), _) => {
                        self.titles_state.borrow_mut().select_previous();
                    }
                    (KeyCode::Down | KeyCode::Char('s'), _) => {
                        self.titles_state.borrow_mut().select_next();
                    }
                    (KeyCode::Right | KeyCode::Char('d'), _) => {
                        if !self.titles.is_empty() {
                            let title_id = self.titles[title_id.unwrap()].id;
                            self.mode = Mode::Group { title_id };
                        }
                    }
                    _ => {}
                }

                let post_title_id = self.titles_state.borrow().selected();
                if title_id != post_title_id && post_title_id.is_some() {
                    let db = self.db.borrow();
                    let db = &*db;
                    let idx = post_title_id.unwrap();

                    if idx < self.titles.len() {
                        let title = &self.titles[idx];
                        self.groups = groups_by_title(&db, title.id);
                        self.group_state.borrow_mut().select_first();
                    }
                }
            }
            Mode::Group { title_id } => match evt.code {
                KeyCode::Up | KeyCode::Char('w') => {
                    self.group_state.borrow_mut().select_previous();
                }
                KeyCode::Down | KeyCode::Char('s') => {
                    self.group_state.borrow_mut().select_next();
                }
                KeyCode::Left | KeyCode::Char('a') => {
                    self.mode = Mode::Title;
                }
                KeyCode::Char('e') => {
                    let id = self.titles_state.borrow().selected().unwrap() as i32;

                    let db = self.db.borrow();
                    let db = &*db;

                    let all_groups = db.all_groups().expect("SQL failed to connect");
                    let cur = all_groups
                        .iter()
                        .map(|x| self.groups.contains(&x))
                        .collect::<Vec<bool>>();
                    let edit = cur.clone();

                    let mut state = ListState::default();
                    state.select_first();                    

                    self.mode = Mode::EditGroups {
                        title_id: *title_id,
                        id,
                        cur,
                        edit,
                        all_groups,
                        state: RefCell::new(state),
                    };
                }
                _ => {}
            },
            Mode::EditTitle { id, state } => match (evt.code, evt.modifiers) {
                (KeyCode::Esc, _) => {
                    self.mode = Mode::Title;
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let name = state.value();

                    let request = UpdateTitle {
                        id: *id,
                        name: name.to_string(),
                    };

                    conn.save(&request)?;

                    self.titles = all_titles(&conn);
                    self.groups = groups_by_title(&conn, *id);
                    let idx = self.titles.iter().position(|x| x.id == *id);
                    *self.titles_state.borrow_mut().selected_mut() = idx;
                    self.mode = Mode::Title;
                }
                _ => {
                    state.handle_event(&Event::Key(*evt));
                    return Ok(true);
                }
            },
            Mode::NewTitle { state } => match (evt.code, evt.modifiers) {
                (KeyCode::Esc, _) => {
                    self.mode = Mode::Title;
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let name = state.value();
                    let request = NewTitle {
                        name: name.to_string(),
                    };

                    if let Ok(id) = conn.save(&request) {
                        self.titles = all_titles(&conn);
                        self.groups = groups_by_title(&conn, id as i32);
                        let idx = self.groups.iter().position(|x| x.id == id as i32);
                        *self.group_state.borrow_mut().selected_mut() = idx;
                        self.group_state.borrow_mut().select_first();
                        self.mode = Mode::Title;
                    }
                }
                _ => {
                    state.handle_event(&Event::Key(*evt));
                    return Ok(true);
                }
            },
            Mode::DeleteTitle { id } => match evt.code {
                KeyCode::Char('y') => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    let request = DeleteTitle { id: *id };

                    let idx = self.titles_state.borrow().selected().unwrap();
                    conn.save(&request)?;
                    self.titles = all_titles(&conn);
                    if idx < self.titles.len() {
                        let id = self.titles[idx].id;
                        self.groups = groups_by_title(&conn, id);
                    } else if !self.titles.is_empty() {
                        self.titles_state.borrow_mut().select_last();
                        let id = self.titles.last().unwrap().id;
                        self.groups = groups_by_title(&conn, id);
                    } else {
                        self.groups.clear();
                    }
                    self.group_state.borrow_mut().select_first();
                    self.mode = Mode::Title;
                }
                KeyCode::Esc | KeyCode::Char('n') => {
                    self.mode = Mode::Title;
                }
                _ => {}
            },
            Mode::EditGroups {
                id,
                state,
                title_id,
                cur,
                edit,
                all_groups: groups,
                ..
            } => match (evt.code, evt.modifiers) {
                (KeyCode::Esc, _) => {
                    self.mode = Mode::Group {
                        title_id: *title_id,
                    };
                }
                (KeyCode::Enter | KeyCode::Char(' '), _) => {
                    if let Some(idx) = state.borrow_mut().selected() {
                        edit[idx] = !edit[idx];
                    }
                }
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    let mut db = self.db.borrow_mut();
                    let conn = &mut *db;

                    for (y, g) in cur
                        .iter()
                        .zip(edit)
                        .zip(groups)
                        .filter(|((x, y), _)| *x != *y)
                        .map(|((_, y), g)| (*y, g))
                    {
                        if y {
                            let req = NewTitleCriteria {
                                title: *title_id,
                                criteria: g.id,
                            };

                            conn.save(&req)?;
                        } else {
                            let req = DeleteTitleCriteria {
                                title: *title_id,
                                criteria: g.id,
                            };

                            conn.save(&req)?;
                        }
                    }

                    let groups = groups_by_title(&conn, *title_id);
                    if let Some(idx) = groups.iter().position(|x| x.id == *id) {
                        let g = &groups[idx];
                        *self.group_state.borrow_mut().selected_mut() = Some(idx);
                        self.mode = Mode::Group { title_id: g.id };
                    } else {
                        self.group_state.borrow_mut().select_first();
                        if groups.is_empty() {
                            self.mode = Mode::Group { title_id: i32::MAX };
                        } else {
                            self.mode = Mode::Group {
                                title_id: groups[0].id,
                            };
                        }
                    }
                    self.groups = groups;
                }
                (KeyCode::Up | KeyCode::Char('w'), _) => {
                    state.borrow_mut().select_previous();
                }
                (KeyCode::Down | KeyCode::Char('s'), _) => {
                    state.borrow_mut().select_next();
                }
                _ => {}
            },
        }

        Ok(false)
    }
}

fn all_titles(conn: &Connection) -> Vec<Title> {
    if let Ok(titles) = conn.all_titles() {
        titles
    } else {
        vec![]
    }
}

fn groups_by_title(conn: &Connection, id: i32) -> Vec<CriteriaGroup> {
    if let Ok(groups) = conn.groups_by_title(id) {
        groups
    } else {
        vec![]
    }
}
