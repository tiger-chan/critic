use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

use super::AppTab;

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct WelcomeWidget {}

impl AppTab for WelcomeWidget {
    fn render(&self, block: Block, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome Widget")
            .block(block)
            .render(area, buf);
    }

    fn handle_key_events(&mut self, evt: &KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        match evt.code {
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Left => {}
            KeyCode::Right => {}
            _ => {}
        }

        Ok(())
    }
}
