use std::{cell::RefCell, rc::Rc};

use critic::{dto::Contest, prelude::*};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::{theme, AppTab};

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Selected {
    #[default]
    None,
    Left,
    Right,
    Skip,
    Equals,
}

#[derive(Debug)]
pub struct RateWidget {
    contest: Contest,
    selected: Selected,
    db: Rc<RefCell<Connection>>,
}

#[allow(unused)]
fn save_match(
    conn: &mut Connection,
    contest: &critic::dto::Contest,
    score: f32,
) -> Result<(), critic::DbError> {
    let result = MatchResult {
        score,
        criteria_group: contest.criterion.group,
        a: contest.a.id,
        b: contest.b.id,
        criterion: contest.criterion.id,
        elo_change: critic::elo::calc_change(contest.a.elo, contest.b.elo, score),
    };
    conn.save(&result).map(|_| ())
}

impl RateWidget {
    pub fn new(db: Rc<RefCell<Connection>>) -> Self {
        let contest = {
            if let Ok(contest) = db.borrow().next_contest() {
                contest
            } else {
                Contest::default()
            }
        };

        Self {
            contest,
            db,
            selected: Selected::None,
        }
    }
}

impl AppTab for RateWidget {
    fn render(&self, area: Rect, frame: &mut ratatui::Frame) {
        let title = Line::from(vec![
            self.contest.criterion.group_name.as_str().bold(),
            " - ".into(),
            self.contest.criterion.name.as_str().into(),
        ])
        .alignment(Alignment::Center);
        frame.render_widget(Block::bordered().title(title), area);

        let centered_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
            ])
            .split(area);

        let card_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(30),
                Constraint::Length(5),
                Constraint::Length(30),
                Constraint::Fill(1),
            ])
            .split(centered_area[2]);

        let alt_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(30),
                Constraint::Fill(1),
            ]);

        let styles = [
            Style::default()
                .fg(theme::HIGHLIGHT)
                .add_modifier(Modifier::BOLD),
            Style::default().fg(theme::DEFAULT),
        ];

        let a_str = self.contest.a.name.as_str();
        let a_style = if self.selected == Selected::Left {
            0
        } else {
            1
        };

        let b_str = self.contest.b.name.as_str();
        let b_style = if self.selected == Selected::Right {
            0
        } else {
            1
        };

        let skiped = if self.selected == Selected::Skip {
            0
        } else {
            1
        };
        let equals = if self.selected == Selected::Equals {
            0
        } else {
            1
        };

        frame.render_widget(
            Paragraph::new(Line::from(vec!["Skip".into()]))
                .block(Block::default().borders(Borders::ALL).style(styles[skiped]))
                .alignment(Alignment::Center),
            alt_area.split(centered_area[1])[1],
        );

        frame.render_widget(
            Paragraph::new(Line::from(vec!["Equal".into()]))
                .block(Block::default().borders(Borders::ALL).style(styles[equals]))
                .alignment(Alignment::Center),
            alt_area.split(centered_area[3])[1],
        );

        frame.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(a_str, styles[a_style])]))
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(styles[a_style]),
                )
                .alignment(Alignment::Center),
            card_area[1],
        );

        frame.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(b_str, styles[b_style])]))
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(styles[b_style]),
                )
                .alignment(Alignment::Center),
            card_area[3],
        );
    }

    fn render_footer(&self, area: Rect, frame: &mut ratatui::Frame) {
        let help = Paragraph::new(
            Line::from(vec![
                " [↑↓←→/WASD]".blue().bold(),
                " Select".into(),
                " [Enter/Space]".blue().bold(),
                " Submit".into(),
            ])
            .left_aligned(),
        );
        frame.render_widget(help, area);
    }

    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        match evt.code {
            KeyCode::Up | KeyCode::Char('w') => {
                self.selected = Selected::Skip;
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.selected = Selected::Equals;
            }
            KeyCode::Left | KeyCode::Char('a') => {
                self.selected = Selected::Left;
            }
            KeyCode::Right | KeyCode::Char('d') => {
                self.selected = Selected::Right;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.selected != Selected::None {
                    let mut db = self.db.borrow_mut();
                    let db = &mut *db;
                    match self.selected {
                        Selected::Left => save_match(db, &self.contest, 1.0)?,
                        Selected::Right => save_match(db, &self.contest, 0.0)?,
                        Selected::Equals => save_match(db, &self.contest, 0.5)?,
                        _ => {}
                    }

                    self.contest = db.next_contest()?;
                    self.selected = Selected::None;
                }
            }
            _ => {}
        }

        Ok(false)
    }
}
