use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

const SCROLL_STEP: usize = 3;

#[derive(Clone, PartialEq)]
pub enum MessageRole {
    User,
    System,
    Error,
}

#[derive(Clone)]
pub struct ChatMessage {
    pub text: String,
    pub role: MessageRole,
}

#[derive(Default)]
pub struct ChatWidget {
    pub messages: Vec<ChatMessage>,
    pub scroll_offset: usize,
    pub focused: bool,
}

impl ChatWidget {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            scroll_offset: 0,
            focused: false,
        }
    }

    pub fn add_message(&mut self, text: impl Into<String>, role: MessageRole) {
        self.messages.push(ChatMessage {
            text: text.into(),
            role,
        });
        self.scroll_offset = 0;
    }

    pub fn scroll_up(&mut self) {
        let max = self.messages.len().saturating_sub(1);
        self.scroll_offset = (self.scroll_offset + SCROLL_STEP).min(max);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(SCROLL_STEP);
    }

    fn is_scrolled(&self) -> bool {
        self.scroll_offset > 0
    }
}

impl Widget for &ChatWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let scroll_indicator = if self.is_scrolled() {
            format!(" \u{2191}{} ", self.scroll_offset)
        } else {
            String::new()
        };
        let title = format!(" Chat ({}){} ", self.messages.len(), scroll_indicator);

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);
        let inner = block.inner(area);
        block.render(area, buf);

        let mut text = Text::default();
        for msg in &self.messages {
            let (prefix, color) = match msg.role {
                MessageRole::User => ("You: ", Color::Cyan),
                MessageRole::System => ("  \u{2713} ", Color::Green),
                MessageRole::Error => ("  \u{2717} ", Color::Red),
            };
            text.lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(Color::DarkGray)),
                Span::styled(&msg.text, Style::default().fg(color)),
            ]));
        }

        let scroll = (self.scroll_offset as u16).min(
            self.messages.len().saturating_sub(1) as u16,
        );
        Paragraph::new(text).scroll((scroll, 0)).render(inner, buf);
    }
}
