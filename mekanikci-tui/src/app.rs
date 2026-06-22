use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::Context;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget};
use ratatui::{Frame, Terminal};

use mekanikci_core::backend::{CadBackend, OpenSCADBackend};
use mekanikci_core::design::DesignSpec;
use mekanikci_llm::client::OllamaClient;
use mekanikci_llm::parser::parse_conveyor_spec;
use mekanikci_llm::prompt::PromptManager;
use mekanikci_llm::validation::validate_conveyor_spec;

use crate::widgets::chat::{ChatWidget, MessageRole};
use crate::widgets::input::InputWidget;
use crate::widgets::status::StatusWidget;

pub struct App {
    pub running: bool,
    pub chat: ChatWidget,
    pub input: InputWidget,
    pub status: StatusWidget,
    processing: bool,
    pending_prompt: String,
}

impl App {
    pub fn new() -> Self {
        let mut input = InputWidget::new();
        input.focused = true;
        Self {
            running: true,
            chat: ChatWidget::new(),
            input,
            status: StatusWidget::new(),
            processing: false,
            pending_prompt: String::new(),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> anyhow::Result<()> {
        while self.running {
            terminal.draw(|frame| self.ui(frame))?;

            if self.processing {
                self.input.disabled = true;
                self.status.set_loading("Generating...");
                // Redraw to show "Generating..." state before blocking
                let _ = terminal.draw(|frame| self.ui(frame));

                self.process_pipeline();

                self.input.disabled = false;
                self.processing = false;
                // Draw again to show the result
                terminal.draw(|frame| self.ui(frame))?;
            }

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        if self.processing {
            return;
        }

        if self.input.focused {
            match key.code {
                KeyCode::Esc => self.running = false,
                KeyCode::Enter => self.submit(),
                KeyCode::Backspace => self.input.delete_char(),
                KeyCode::Char(c) if key.modifiers == KeyModifiers::CONTROL && c == 'u' => {
                    self.input.clear();
                }
                KeyCode::Char(c) if !c.is_control() => {
                    self.input.insert_char(c);
                }
                KeyCode::Tab => self.focus_input(false),
                KeyCode::PageUp => self.chat.scroll_up(),
                KeyCode::PageDown => self.chat.scroll_down(),
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Esc => self.running = false,
                KeyCode::Tab => self.focus_input(true),
                KeyCode::PageUp => self.chat.scroll_up(),
                KeyCode::PageDown => self.chat.scroll_down(),
                _ => {}
            }
        }
    }

    fn focus_input(&mut self, focused: bool) {
        self.input.focused = focused;
        self.chat.focused = !focused;
    }

    fn submit(&mut self) {
        let text = self.input.take_input();
        if text.is_empty() {
            return;
        }

        self.chat.add_message(&text, MessageRole::User);
        self.pending_prompt = text;
        self.processing = true;
    }

    fn process_pipeline(&mut self) {
        let prompt = std::mem::take(&mut self.pending_prompt);
        let start = Instant::now();

        let result = self.run_pipeline(&prompt);

        match result {
            Ok(summary) => {
                let elapsed = start.elapsed();
                self.status
                    .set_success(format!("Done in {:.1}s", elapsed.as_secs_f64()));
                self.chat.add_message(summary, MessageRole::System);
            }
            Err(e) => {
                let msg = format!("{e:#}");
                self.status.set_error(&msg);
                self.chat.add_message(msg, MessageRole::Error);
            }
        }
    }

    fn run_pipeline(&self, prompt: &str) -> anyhow::Result<String> {
        let full_prompt = PromptManager::build_prompt(prompt);

        let client = OllamaClient::new("http://127.0.0.1:11434", "qwen3.5:4b", 0.0);
        let json = client
            .generate(&full_prompt)
            .context("LLM call failed")?;

        let spec = parse_conveyor_spec(&json).context("Failed to parse JSON")?;

        if let Err(errors) = validate_conveyor_spec(&spec) {
            let details: Vec<String> = errors
                .iter()
                .map(|e| format!("  {} \u{2014} {}", e.field, e.message))
                .collect();
            anyhow::bail!("Validation failed:\n{}", details.join("\n"));
        }

        let cad = spec.to_cad_model().context("CAD generation failed")?;

        let output = OpenSCADBackend
            .render(&cad, Path::new("./output"))
            .context("OpenSCAD rendering failed")?;

        let scad = output
            .scad_path
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        let stl = output
            .stl_path
            .map(|p| p.display().to_string())
            .unwrap_or_default();

        Ok(format!(
            "Conveyor generated:
  Length: {}mm  Belt: {}mm
  Motor: {:?} ({:?})
  SCAD: {scad}
  STL:  {stl}",
            spec.length_mm, spec.belt_width_mm, spec.motor.frame, spec.motor.mount,
        ))
    }

    fn ui(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(frame.area());

        frame.render_widget(self.top_bar(), chunks[0]);
        frame.render_widget(&self.chat, chunks[1]);
        frame.render_widget(&self.input, chunks[2]);
        frame.render_widget(&self.status, chunks[3]);

        if self.input.focused && !self.input.disabled {
            let x = chunks[2].x + 2 + self.input.cursor as u16;
            let y = chunks[2].y + 1;
            frame.set_cursor_position((x, y));
        }
    }

    fn top_bar(&self) -> impl Widget + '_ {
        Paragraph::new(Line::from(" Mekhanikçi v0.1"))
            .style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
