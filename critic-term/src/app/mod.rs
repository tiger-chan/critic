mod group;
mod rate;

use std::{cell::RefCell, rc::Rc};

use critic::{prelude::Connection, DbConnection};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use group::GroupWidget;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
    style::{palette::tailwind, Stylize},
    text::Line,
    widgets::{Block, Tabs},
    DefaultTerminal, Frame,
};
use rate::RateWidget;

#[allow(dead_code)]
/// Helper function to create a centered rect using up certain percentage of the available rect `r`
pub(super) fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    use ratatui::layout::Flex;
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum ActiveScreen {
    Rate,
    Group,
    Title,
    Top,
    Exiting,
}

impl ActiveScreen {
    pub fn rate(db: Rc<RefCell<Connection>>) -> (ActiveScreen, Box<dyn AppTab>) {
        (ActiveScreen::Rate, Box::new(RateWidget::new(db)))
    }

    pub fn group(db: Rc<RefCell<Connection>>) -> (ActiveScreen, Box<dyn AppTab>) {
        (ActiveScreen::Group, Box::new(GroupWidget::new(db)))
    }
}

pub(super) trait AppTab: std::fmt::Debug {
    fn render(&self, area: Rect, frame: &mut Frame);
    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<bool, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct App {
    tab: (ActiveScreen, Box<dyn AppTab>),
    db: Rc<RefCell<Connection>>,
    db_name: String,
}

impl App {
    pub fn new<T: AsRef<str>>(db: T) -> Self {
        let db_name = db.as_ref().to_string();
        let db = Rc::new(RefCell::new(
            Connection::open_category(&db_name).expect("A valid db is requred"),
        ));
        Self {
            tab: ActiveScreen::rate(db.clone()),
            db,
            db_name,
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
        self.render(frame);
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
        if !(self.tab.1.handle_key_events(&evt)?) {
            match (evt.code, evt.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.exit(),
                (KeyCode::Char('q'), _) => self.exit(),
                (KeyCode::Char('1'), _) => {
                    if self.tab.0 != ActiveScreen::Rate {
                        self.tab = ActiveScreen::rate(self.db.clone());
                    }
                }
                (KeyCode::Char('2'), _) => {
                    if self.tab.0 != ActiveScreen::Group {
                        self.tab = ActiveScreen::group(self.db.clone());
                    }
                }
                (KeyCode::Char('3'), _) => {}
                (KeyCode::Char('4'), _) => {}
                _ => {}
            }
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.tab.0 = ActiveScreen::Exiting;
    }

    fn render(&self, frame: &mut Frame) {
        use Constraint::{Length, Min};
        let area = frame.area();
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        {
            frame.render_widget(
                Line::from(vec![self.db_name.as_str().blue().bold(), " Critic".bold()]),
                title_area,
            );
        }

        {
            let highlight_style = (tailwind::BLUE.c900, tailwind::SLATE.c200);
            let tabs = Tabs::new(vec!["Rate", "Group", "Title", "Top"])
                .highlight_style(highlight_style)
                .select(self.tab.0 as usize)
                .padding("", "")
                .divider(" ");
            frame.render_widget(tabs, tabs_area);
        }

        {
            // Active tab screen
            frame.render_widget(Block::bordered(), inner_area);
            self.tab.1.render(inner_area, frame);
        }

        {
            let help = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]).centered();

            frame.render_widget(help, footer_area);
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
