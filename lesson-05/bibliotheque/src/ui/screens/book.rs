use crate::app::state::{FormState, ListState, SharedBibliotheque};
use crate::ui::widgets::Form;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

// Formulaire d'ajout de livre
pub struct AddBookScreen {
    form: Form,
}

impl AddBookScreen {
    pub fn new(state: &FormState) -> Self {
        let labels = vec![
            "ID".to_string(),
            "Titre".to_string(),
            "Auteur ID".to_string(),
            "Année".to_string(),
        ];
        let form = Form::with_values(labels, state.fields.clone());
        AddBookScreen { form }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<BookAction> {
        if key.code == KeyCode::Enter && !key.modifiers.contains(crossterm::event::KeyModifiers::SHIFT) {
            // Vérifier si on n'est pas en train d'éditer un champ
            let values = self.form.values();
            if !values.iter().any(|v| v.trim().is_empty()) {
                return Some(BookAction::SubmitAdd(values));
            }
        }

        self.form.handle_key_event(key);
        None
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(15), Constraint::Length(1)])
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("📚 Ajouter un livre")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(chunks[0]);
        block.render(chunks[0], buf);

        self.form.render(inner, buf);

        // Aide
        let help = Paragraph::new(Line::from(vec![
            Span::styled("Tab", Style::default().fg(Color::Yellow)),
            Span::raw(":Champ suivant "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(":Ajouter "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(":Annuler"),
        ]))
        .alignment(Alignment::Center);

        help.render(chunks[1], buf);
    }
}

// Liste des livres
pub struct ListBooksScreen<'a> {
    biblio: &'a SharedBibliotheque,
    state: &'a mut ListState,
}

impl<'a> ListBooksScreen<'a> {
    pub fn new(biblio: &'a SharedBibliotheque, state: &'a mut ListState) -> Self {
        ListBooksScreen { biblio, state }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        let b = self.biblio.lock().unwrap();
        let count = b.get_livres().len();
        drop(b);

        if count == 0 {
            return;
        }

        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.state.selected = (self.state.selected + 1) % count;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.selected == 0 {
                    self.state.selected = count - 1;
                } else {
                    self.state.selected -= 1;
                }
            }
            KeyCode::PageDown => {
                self.state.selected = (self.state.selected + 10).min(count - 1);
            }
            KeyCode::PageUp => {
                self.state.selected = self.state.selected.saturating_sub(10);
            }
            _ => {}
        }

        // Ajuster le scroll_offset pour garder l'élément sélectionné visible
        const VISIBLE_LINES: usize = 15;

        if self.state.selected >= self.state.scroll_offset + VISIBLE_LINES {
            // Scroll down
            self.state.scroll_offset = self.state.selected.saturating_sub(VISIBLE_LINES - 1);
        } else if self.state.selected < self.state.scroll_offset {
            // Scroll up
            self.state.scroll_offset = self.state.selected;
        }
    }
}

impl<'a> Widget for ListBooksScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(1)])
            .split(area);

        let b = self.biblio.lock().unwrap();
        let livres = b.get_livres();

        let items: Vec<ListItem> = if livres.is_empty() {
            vec![ListItem::new(Line::from(Span::styled(
                "Aucun livre dans la bibliothèque",
                Style::default().fg(Color::DarkGray),
            )))]
        } else {
            livres
                .iter()
                .enumerate()
                .map(|(i, livre)| {
                    let status_icon = if livre.emprunte { "●" } else { "○" };
                    let status_color = if livre.emprunte { Color::Red } else { Color::Green };
                    let status_text = if livre.emprunte { "Emprunté" } else { "Disponible" };

                    let line = Line::from(vec![
                        Span::raw(format!("#{} - {} ({}) - Auteur ID: {} ", livre.id, livre.titre, livre.annee, livre.auteur_id)),
                        Span::styled(status_icon, Style::default().fg(status_color)),
                        Span::styled(format!(" {}", status_text), Style::default().fg(status_color)),
                    ]);

                    let style = if i == self.state.selected {
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };

                    ListItem::new(line).style(style)
                })
                .collect()
        };

        // Appliquer le scroll en skippant les premiers items
        let visible_items: Vec<ListItem> = items
            .into_iter()
            .skip(self.state.scroll_offset)
            .collect();

        let list = List::new(visible_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("📖 Liste des livres")
                .border_style(Style::default().fg(Color::Blue)),
        );

        list.render(chunks[0], buf);

        // Aide
        let help = Paragraph::new(Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
            Span::raw(":Navigate "),
            Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
            Span::raw(":Scroll "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(":Retour"),
        ]))
        .alignment(Alignment::Center);

        help.render(chunks[1], buf);
    }
}

// Dialog d'emprunt de livre
pub struct BorrowBookScreen {
    form: Form,
}

impl BorrowBookScreen {
    pub fn new(state: &FormState) -> Self {
        let labels = vec!["ID du livre à emprunter".to_string()];
        let form = Form::with_values(labels, state.fields.clone());
        BorrowBookScreen { form }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<BookAction> {
        if key.code == KeyCode::Enter {
            let values = self.form.values();
            if !values[0].trim().is_empty() {
                return Some(BookAction::SubmitBorrow(values[0].clone()));
            }
        }

        self.form.handle_key_event(key);
        None
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(8), Constraint::Length(1)])
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("✋ Emprunter un livre")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(chunks[0]);
        block.render(chunks[0], buf);

        self.form.render(inner, buf);

        // Aide
        let help = Paragraph::new(Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(":Emprunter "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(":Annuler"),
        ]))
        .alignment(Alignment::Center);

        help.render(chunks[1], buf);
    }
}

// Dialog de retour de livre
pub struct ReturnBookScreen {
    form: Form,
}

impl ReturnBookScreen {
    pub fn new(state: &FormState) -> Self {
        let labels = vec!["ID du livre à retourner".to_string()];
        let form = Form::with_values(labels, state.fields.clone());
        ReturnBookScreen { form }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<BookAction> {
        if key.code == KeyCode::Enter {
            let values = self.form.values();
            if !values[0].trim().is_empty() {
                return Some(BookAction::SubmitReturn(values[0].clone()));
            }
        }

        self.form.handle_key_event(key);
        None
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(8), Constraint::Length(1)])
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("📥 Retourner un livre")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(chunks[0]);
        block.render(chunks[0], buf);

        self.form.render(inner, buf);

        // Aide
        let help = Paragraph::new(Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(":Retourner "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(":Annuler"),
        ]))
        .alignment(Alignment::Center);

        help.render(chunks[1], buf);
    }
}

pub enum BookAction {
    SubmitAdd(Vec<String>),
    SubmitBorrow(String),
    SubmitReturn(String),
}
