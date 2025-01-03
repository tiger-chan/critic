use super::{theme, AppTab};
use critic::{dto, prelude::*};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Constraint,
    prelude::Rect,
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Frame,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct TopWidget {
    db: Rc<RefCell<Connection>>,
    rows: Vec<dto::TopRow>,
    page: usize,
    criteria: String,
    state: RefCell<TableState>,
}

impl TopWidget {
    pub fn new(db: Rc<RefCell<Connection>>) -> Self {
        let rows = top_rows(&*db.borrow(), "", 0);
        let mut state = TableState::default();
        state.select_first();
        Self {
            db,
            rows,
            page: 0,
            criteria: "".to_string(),
            state: RefCell::new(state),
        }
    }
}

impl AppTab for TopWidget {
    fn render(&self, area: Rect, frame: &mut Frame) {
        let rows = self.rows.iter().map(|x| {
            Row::new::<Vec<Text>>(vec![
                x.entry.as_str().into(),
                x.group.as_str().into(),
                x.elo.to_string().into(),
            ])
        });
        let columns = Constraint::from_ratios([(3, 8), (3, 8), (2, 8)]);
        let criteria = Line::from(vec![
            Span::from("Criteria: "),
            Span::from(self.criteria.as_str()),
        ]);
        let table = Table::new(rows, columns)
            .header(
                Row::new(vec!["Title", "Criteria", "ELO"])
                    .style(theme::HIGHLIGHT)
                    .bottom_margin(1),
            )
            .column_spacing(1)
            .style(theme::DEFAULT)
            .row_highlight_style(theme::HIGHLIGHT)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title_bottom(criteria),
            );

        let state = &mut *self.state.borrow_mut();
        frame.render_stateful_widget(table, area, state);
    }

    fn render_footer(&self, area: Rect, frame: &mut ratatui::Frame) {
        let help = Paragraph::new(
            Line::from(vec![
                " [↑↓/WS]".blue().bold(),
                " Navigate".into(),
                " [←→/AD]".blue().bold(),
                " Page".into(),
                " [Enter/Space]".blue().bold(),
                " Filter".into(),
            ])
            .left_aligned(),
        );
        frame.render_widget(help, area);
    }

    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        match evt.code {
            KeyCode::Up | KeyCode::Char('w') => {
                self.state.borrow_mut().select_previous();
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.state.borrow_mut().select_next();
            }
            KeyCode::Left | KeyCode::Char('a') => {
                let db = self.db.borrow();
                let db = &*db;
                let page = self.page.saturating_sub(1);
                let rows = top_rows(&db, &self.criteria, page);
                if !rows.is_empty() {
                    self.rows = rows;
                    self.page = page;
                    self.state.borrow_mut().select_first();
                }
            }
            KeyCode::Right | KeyCode::Char('d') => {
                let db = self.db.borrow();
                let db = &*db;
                let page = self.page.saturating_add(1);
                let rows = top_rows(&db, &self.criteria, page);
                if !rows.is_empty() {
                    self.rows = rows;
                    self.page = page;
                    self.state.borrow_mut().select_first();
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let db = self.db.borrow();
                let db = &*db;
                if self.criteria.is_empty() {
                    if let Some(idx) = self.state.borrow().selected() {
                        self.criteria = self.rows[idx].group.clone();
                    }
                } else {
                    self.criteria = "".to_string();
                }
                self.page = 0;
                self.state.borrow_mut().select_first();
                self.rows = top_rows(&db, &self.criteria, self.page);
            }
            _ => {}
        }
        Ok(false)
    }
}

fn top_rows(conn: &Connection, group: &str, page: usize) -> Vec<dto::TopRow> {
    if let Ok(rows) = conn.top(group, 30, page) {
        rows
    } else {
        vec![]
    }
}
