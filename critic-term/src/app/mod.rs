mod group;
mod rate;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use group::GroupWidget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Stylize},
    text::Line,
    widgets::{Block, Tabs, Widget},
    DefaultTerminal, Frame,
};
use rate::RateWidget;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum ActiveScreen {
    Rate,
    Group,
    Title,
    Top,
    Exiting,
}

impl ActiveScreen {
    pub fn rate<T: AsRef<str>>(db: T) -> (ActiveScreen, Box<dyn AppTab>) {
        (ActiveScreen::Rate, Box::new(RateWidget::new(db)))
    }

    pub fn group<T: AsRef<str>>(db: T) -> (ActiveScreen, Box<dyn AppTab>) {
        (ActiveScreen::Group, Box::new(GroupWidget::new(db)))
    }
}

pub(super) trait AppTab: std::fmt::Debug {
    fn render(&self, block: Block, area: Rect, buf: &mut Buffer);
    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct App {
    tab: (ActiveScreen, Box<dyn AppTab>),
    db: String,
}

impl App {
    pub fn new<T: AsRef<str>>(db: T) -> Self {
        Self {
            tab: ActiveScreen::rate(db.as_ref()),
            db: String::default(),
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.tab.0 != ActiveScreen::Exiting {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, evt: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        match (evt.code, evt.modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.exit(),
            (KeyCode::Char('q'), _) => self.exit(),
            (KeyCode::Char('1'), _) => {
                if self.tab.0 != ActiveScreen::Rate {
                    self.tab = ActiveScreen::rate(&self.db);
                }
            }
            (KeyCode::Char('2'), _) => {
                if self.tab.0 != ActiveScreen::Group {
                    self.tab = ActiveScreen::group(&self.db);
                }
            }
            (KeyCode::Char('3'), _) => {}
            (KeyCode::Char('4'), _) => {}
            _ => {}
        }

        self.tab.1.handle_key_events(&evt)?;
        Ok(())
    }

    fn exit(&mut self) {
        self.tab.0 = ActiveScreen::Exiting;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        {
            "Critic".bold().render(title_area, buf);
        }

        {
            let highlight_style = (tailwind::BLUE.c900, tailwind::SLATE.c200);
            Tabs::new(vec!["Rate", "Group", "Title", "Top"])
                .highlight_style(highlight_style)
                .select(self.tab.0 as usize)
                .padding("", "")
                .divider(" ")
                .render(tabs_area, buf);
        }

        {
            // Active tab screen
            let block = Block::bordered();
            self.tab.1.render(block, inner_area, buf);
        }

        {
            Line::from(vec![
                " Load ".into(),
                " <L> ".blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ])
            .centered()
            .render(footer_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_event() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = App::new("Test.db");
        app.handle_key_event(KeyCode::Char('q').into())?;
        assert_eq!(app.tab.0, ActiveScreen::Exiting);

        Ok(())
    }
}
