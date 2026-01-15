use super::text_input::TextInput;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

pub struct Form {
    pub fields: Vec<TextInput>,
    pub focused_field: usize,
}

impl Form {
    pub fn new(labels: Vec<String>) -> Self {
        let fields = labels
            .into_iter()
            .map(|label| TextInput::new(label))
            .collect();

        Form {
            fields,
            focused_field: 0,
        }
    }

    pub fn with_values(labels: Vec<String>, values: Vec<String>) -> Self {
        let fields = labels
            .into_iter()
            .zip(values.into_iter())
            .map(|(label, value)| TextInput::with_value(label, value))
            .collect();

        Form {
            fields,
            focused_field: 0,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Tab => {
                self.focus_next();
                true
            }
            KeyCode::BackTab => {
                self.focus_prev();
                true
            }
            KeyCode::Down => {
                self.focus_next();
                true
            }
            KeyCode::Up => {
                self.focus_prev();
                true
            }
            _ => {
                if let Some(field) = self.fields.get_mut(self.focused_field) {
                    field.handle_key_event(key);
                }
                false
            }
        }
    }

    pub fn focus_next(&mut self) {
        if !self.fields.is_empty() {
            self.focused_field = (self.focused_field + 1) % self.fields.len();
        }
    }

    pub fn focus_prev(&mut self) {
        if !self.fields.is_empty() {
            if self.focused_field == 0 {
                self.focused_field = self.fields.len() - 1;
            } else {
                self.focused_field -= 1;
            }
        }
    }

    pub fn values(&self) -> Vec<String> {
        self.fields.iter().map(|f| f.value().to_string()).collect()
    }

    pub fn get_value(&self, index: usize) -> Option<&str> {
        self.fields.get(index).map(|f| f.value())
    }

    pub fn set_values(&mut self, values: Vec<String>) {
        for (i, value) in values.into_iter().enumerate() {
            if let Some(field) = self.fields.get_mut(i) {
                field.set_value(value);
            }
        }
    }

    pub fn clear(&mut self) {
        for field in &mut self.fields {
            field.clear();
        }
        self.focused_field = 0;
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        // Validation basique: vérifier que les champs ne sont pas vides
        let errors: Vec<String> = self
            .fields
            .iter()
            .enumerate()
            .filter_map(|(i, field)| {
                if field.value().trim().is_empty() {
                    Some(format!("Le champ '{}' ne peut pas être vide", field.label))
                } else {
                    None
                }
            })
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Widget for &mut Form {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let num_fields = self.fields.len();
        if num_fields == 0 {
            return;
        }

        // Créer un layout vertical avec un espace pour chaque champ
        let constraints: Vec<Constraint> = self
            .fields
            .iter()
            .map(|_| Constraint::Length(3))
            .collect();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        // Rendre chaque champ
        for (idx, field) in self.fields.iter_mut().enumerate() {
            field.focused = idx == self.focused_field;
            if let Some(&chunk) = chunks.get(idx) {
                field.render(chunk, buf);
            }
        }
    }
}
