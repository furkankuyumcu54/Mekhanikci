use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget};

pub struct StatusWidget {
    pub message: String,
    pub style: Style,
}

impl StatusWidget {
    pub fn new() -> Self {
        Self {
            message: "Ready  |  Type a prompt and press Enter to generate  |  Tab: focus  |  Esc: quit".into(),
            style: Style::default().fg(Color::DarkGray),
        }
    }

    pub fn set_ready(&mut self) {
        self.message = "Ready  |  Tab: focus  |  PgUp/PgDn: scroll  |  Esc: quit".into();
        self.style = Style::default().fg(Color::DarkGray);
    }

    pub fn set_loading(&mut self, msg: impl Into<String>) {
        self.message = msg.into();
        self.style = Style::default().fg(Color::Yellow);
    }

    pub fn set_error(&mut self, msg: impl Into<String>) {
        self.message = msg.into();
        self.style = Style::default().fg(Color::Red);
    }

    pub fn set_success(&mut self, msg: impl Into<String>) {
        self.message = msg.into();
        self.style = Style::default().fg(Color::Green);
    }
}

impl Default for StatusWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &StatusWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(self.message.as_str()).style(self.style);
        paragraph.render(area, buf);
    }
}
