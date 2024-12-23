use critic::{dto::Contest, prelude::*};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{palette::tailwind, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::AppTab;

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Selected {
    #[default]
    None,
    Left,
    Right,
    Skip,
    Equals,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct RateWidget {
    contest: Contest,
    selected: Selected,
    db: String,
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
    pub fn new<T: AsRef<str>>(db: T) -> Self {
        let contest = {
            if let Ok(conn) = Connection::open_category(db.as_ref()) {
                if let Ok(contest) = conn.next_contest() {
                    contest
                } else {
                    Contest::default()
                }
            } else {
                Contest::default()
            }
        };

        Self {
            contest,
            db: db.as_ref().to_string(),
            selected: Selected::None,
        }
    }
}

impl AppTab for RateWidget {
    fn render(&self, block: Block, area: Rect, buf: &mut Buffer) {
        let title = Line::from(vec![
            self.contest.criterion.group_name.as_str().bold(),
            " - ".into(),
            self.contest.criterion.name.as_str().into(),
        ])
        .alignment(Alignment::Center);
        Paragraph::default()
            .block(block.title(title))
            .render(area, buf);

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
                .fg(tailwind::YELLOW.c200)
                .add_modifier(Modifier::BOLD),
            Style::default().fg(tailwind::WHITE),
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

        Paragraph::new(Line::from(vec!["Skip".into()]))
            .block(Block::default().borders(Borders::ALL).style(styles[skiped]))
            .alignment(Alignment::Center)
            .render(alt_area.split(centered_area[1])[1], buf);

        Paragraph::new(Line::from(vec!["Equal".into()]))
            .block(Block::default().borders(Borders::ALL).style(styles[equals]))
            .alignment(Alignment::Center)
            .render(alt_area.split(centered_area[3])[1], buf);

        Paragraph::new(Line::from(vec![Span::styled(a_str, styles[a_style])]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(styles[a_style]),
            )
            .alignment(Alignment::Center)
            .render(card_area[1], buf);

        Paragraph::new(Line::from(vec![Span::styled(b_str, styles[b_style])]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(styles[b_style]),
            )
            .alignment(Alignment::Center)
            .render(card_area[3], buf);
    }

    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        match evt.code {
            KeyCode::Up => {
                self.selected = Selected::Skip;
            }
            KeyCode::Down => {
                self.selected = Selected::Equals;
            }
            KeyCode::Left => {
                self.selected = Selected::Left;
            }
            KeyCode::Right => {
                self.selected = Selected::Right;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let mut conn =
                    Connection::open_category(self.db.as_str()).expect("Db should be available");

                match self.selected {
                    Selected::Left => save_match(&mut conn, &self.contest, 1.0)?,
                    Selected::Right => save_match(&mut conn, &self.contest, 0.0)?,
                    Selected::Equals => save_match(&mut conn, &self.contest, 0.5)?,
                    _ => {}
                }

                self.contest = conn.next_contest()?;
                self.selected = Selected::None;
            }
            _ => {}
        }

        Ok(())
    }
}
