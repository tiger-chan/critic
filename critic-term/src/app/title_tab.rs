use std::{cell::RefCell, rc::Rc};

use critic::{
    dto::{self, CriteriaGroup, Title},
    prelude::Connection,
    CriticData,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use super::{theme, AppTab};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Mode {
    #[default]
    Title,
    Group {
        title_id: i32,
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
    }

    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        match self.mode {
            Mode::Title => {
                let title_id = self.titles_state.borrow().selected();
                match evt.code {
                    KeyCode::Up => {
                        self.titles_state.borrow_mut().select_previous();
                    }
                    KeyCode::Down => {
                        self.titles_state.borrow_mut().select_next();
                    }
                    KeyCode::Right => {
                        if !self.titles.is_empty() {
                            let title_id = self.titles[title_id.unwrap()].id;
                            self.mode = Mode::Group { title_id };
                        }
                    }
                    _ => {}
                }

                let post_title_id = self.group_state.borrow().selected();
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
            Mode::Group { title_id: _ } => match evt.code {
                KeyCode::Up => {
                    self.group_state.borrow_mut().select_previous();
                }
                KeyCode::Down => {
                    self.group_state.borrow_mut().select_next();
                }
                KeyCode::Left => {
                    self.mode = Mode::Title;
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
