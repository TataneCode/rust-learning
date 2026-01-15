use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};
use tui_input::Input;

pub struct TextInput {
    pub input: Input,
    pub label: String,
    pub focused: bool,
}

impl TextInput {
    pub fn new(label: String) -> Self {
        TextInput {
            input: Input::default(),
            label,
            focused: false,
        }
    }

    pub fn with_value(label: String, value: String) -> Self {
        TextInput {
            input: Input::default().with_value(value),
            label,
            focused: false,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.input.handle(tui_input::InputRequest::InsertChar(c));
            }
            KeyCode::Backspace => {
                self.input.handle(tui_input::InputRequest::DeletePrevChar);
            }
            KeyCode::Delete => {
                self.input.handle(tui_input::InputRequest::DeleteNextChar);
            }
            KeyCode::Left => {
                self.input.handle(tui_input::InputRequest::GoToPrevChar);
            }
            KeyCode::Right => {
                self.input.handle(tui_input::InputRequest::GoToNextChar);
            }
            KeyCode::Home => {
                self.input.handle(tui_input::InputRequest::GoToStart);
            }
            KeyCode::End => {
                self.input.handle(tui_input::InputRequest::GoToEnd);
            }
            _ => {}
        }
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    pub fn set_value(&mut self, value: String) {
        self.input = Input::default().with_value(value);
    }

    pub fn clear(&mut self) {
        self.input = Input::default();
    }
}

impl Widget for &TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.label.as_str())
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, buf);

        // Afficher le texte avec curseur si focus
        let text = if self.focused {
            let cursor_pos = self.input.cursor();
            let value = self.input.value();

            // Insertion du curseur
            if cursor_pos < value.len() {
                format!("{}█{}", &value[..cursor_pos], &value[cursor_pos..])
            } else {
                format!("{}█", value)
            }
        } else {
            self.input.value().to_string()
        };

        let paragraph = Paragraph::new(Line::from(text))
            .style(Style::default().fg(Color::White));

        paragraph.render(inner, buf);
    }
}
