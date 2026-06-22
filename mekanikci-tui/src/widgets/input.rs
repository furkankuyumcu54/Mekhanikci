use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

pub struct InputWidget {
    pub input: String,
    pub cursor: usize,
    pub focused: bool,
    pub disabled: bool,
}

impl InputWidget {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            focused: false,
            disabled: false,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.input.remove(self.cursor);
        }
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor = 0;
    }

    pub fn take_input(&mut self) -> String {
        let text = std::mem::take(&mut self.input);
        self.cursor = 0;
        text
    }
}

impl Default for InputWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &InputWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = if self.disabled {
            " Input (disabled) "
        } else {
            " Input "
        };
        let border_style = if self.disabled {
            Style::default().fg(Color::DarkGray)
        } else if self.focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);
        let inner = block.inner(area);
        block.render(area, buf);

        let text = format!(">{}", self.input);
        Paragraph::new(text).render(inner, buf);
    }
}
