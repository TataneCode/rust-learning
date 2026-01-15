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

// Formulaire d'ajout d'auteur
pub struct AddAuthorScreen {
    form: Form,
}

impl AddAuthorScreen {
    pub fn new(state: &FormState) -> Self {
        let labels = vec!["ID".to_string(), "Prénom".to_string(), "Nom".to_string()];
        let form = Form::with_values(labels, state.fields.clone());
        AddAuthorScreen { form }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<AuthorAction> {
        if key.code == KeyCode::Enter
            && !key
                .modifiers
                .contains(crossterm::event::KeyModifiers::SHIFT)
        {
            let values = self.form.values();
            if !values.iter().any(|v| v.trim().is_empty()) {
                return Some(AuthorAction::SubmitAdd(values));
            }
        }

        self.form.handle_key_event(key);
        None
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(12), Constraint::Length(1)])
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("✍️  Ajouter un auteur")
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

// Liste des auteurs
pub struct ListAuthorsScreen<'a> {
    biblio: &'a SharedBibliotheque,
    state: &'a mut ListState,
}

impl<'a> ListAuthorsScreen<'a> {
    pub fn new(biblio: &'a SharedBibliotheque, state: &'a mut ListState) -> Self {
        ListAuthorsScreen { biblio, state }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        let b = self.biblio.lock().unwrap();

        // Compter le nombre total d'items affichés (auteurs + livres + lignes vides)
        let mut total_items = 0;
        for auteur in b.get_auteurs() {
            total_items += 1; // En-tête auteur
            total_items += auteur.livres.len(); // Livres
            total_items += 1; // Ligne vide
        }
        drop(b);

        if total_items == 0 {
            return;
        }

        let old_selected = self.state.selected;

        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.state.selected = (self.state.selected + 1) % total_items;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.selected == 0 {
                    self.state.selected = total_items - 1;
                } else {
                    self.state.selected -= 1;
                }
            }
            KeyCode::PageDown => {
                self.state.selected = (self.state.selected + 10).min(total_items - 1);
            }
            KeyCode::PageUp => {
                self.state.selected = self.state.selected.saturating_sub(10);
            }
            _ => {}
        }

        // Ajuster le scroll_offset pour garder l'élément sélectionné visible
        // On affiche environ 15 lignes à la fois (estimation)
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

impl<'a> Widget for ListAuthorsScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(1)])
            .split(area);

        let b = self.biblio.lock().unwrap();
        let auteurs = b.get_auteurs();

        let mut items: Vec<ListItem> = Vec::new();
        let mut item_index = 0;
        let selected = self.state.selected;

        if auteurs.is_empty() {
            items.push(ListItem::new(Line::from(Span::styled(
                "Aucun auteur dans la bibliothèque",
                Style::default().fg(Color::DarkGray),
            ))));
        } else {
            for auteur in auteurs {
                // En-tête auteur avec highlight si sélectionné
                let is_selected = item_index == selected;
                let header_style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                };

                let prefix = if is_selected { "> " } else { "  " };
                let header = Line::from(vec![
                    Span::styled(prefix, header_style),
                    Span::styled(
                        format!(
                            "#{} - {} {} ({} livre(s))",
                            auteur.id, auteur.prenom, auteur.nom, auteur.livres.len()
                        ),
                        header_style,
                    ),
                ]);
                items.push(ListItem::new(header));
                item_index += 1;

                // Liste des livres de l'auteur
                if !auteur.livres.is_empty() {
                    for &livre_id in &auteur.livres {
                        let is_selected = item_index == selected;
                        let livre_style = if is_selected {
                            Style::default().fg(Color::White).bg(Color::DarkGray)
                        } else {
                            Style::default().fg(Color::White)
                        };

                        let prefix = if is_selected { "> " } else { "  " };

                        match b.get_livres().iter().find(|l| l.id == livre_id) {
                            Some(livre) => {
                                let livre_line = Line::from(vec![
                                    Span::styled(prefix, livre_style),
                                    Span::raw(" └─ "),
                                    Span::styled(
                                        format!("{} ({})", livre.titre, livre.annee),
                                        livre_style,
                                    ),
                                ]);
                                items.push(ListItem::new(livre_line));
                            }
                            None => {
                                let livre_line = Line::from(vec![
                                    Span::styled(prefix, livre_style),
                                    Span::raw(" └─ "),
                                    Span::styled(
                                        format!("Livre ID {} (non trouvé)", livre_id),
                                        Style::default().fg(Color::Red),
                                    ),
                                ]);
                                items.push(ListItem::new(livre_line));
                            }
                        }
                        item_index += 1;
                    }
                }

                // Ligne vide entre auteurs
                items.push(ListItem::new(Line::from("")));
                item_index += 1;
            }
        }

        // Appliquer le scroll en skippant les premiers items
        let visible_items: Vec<ListItem> = items
            .into_iter()
            .skip(self.state.scroll_offset)
            .collect();

        let list = List::new(visible_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("👥 Liste des auteurs")
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

pub enum AuthorAction {
    SubmitAdd(Vec<String>),
}
