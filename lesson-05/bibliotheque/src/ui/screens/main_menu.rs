use crate::app::state::ListState;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

const MENU_ITEMS: &[(&str, &str)] = &[
    ("add_author", "✍️  Ajouter un auteur"),
    ("add_book", "📚 Ajouter un livre"),
    ("list_books", "📖 Lister les livres"),
    ("borrow_book", "✋ Emprunter un livre"),
    ("return_book", "📥 Retourner un livre"),
    ("list_authors", "👥 Lister les auteurs"),
    ("save", "💾 Sauvegarder"),
    ("load", "📂 Charger"),
    ("quit", "X - Quitter"),
];

pub enum MenuAction {
    AddAuthor,
    AddBook,
    ListBooks,
    BorrowBook,
    ReturnBook,
    ListAuthors,
    Save,
    Load,
    Quit,
    None,
}

pub struct MainMenu<'a> {
    state: &'a mut ListState,
}

impl<'a> MainMenu<'a> {
    pub fn new(state: &'a mut ListState) -> Self {
        MainMenu { state }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> MenuAction {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.state.selected = (self.state.selected + 1) % MENU_ITEMS.len();
                MenuAction::None
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.selected == 0 {
                    self.state.selected = MENU_ITEMS.len() - 1;
                } else {
                    self.state.selected -= 1;
                }
                MenuAction::None
            }
            KeyCode::Enter => {
                match MENU_ITEMS[self.state.selected].0 {
                    "add_author" => MenuAction::AddAuthor,
                    "add_book" => MenuAction::AddBook,
                    "list_books" => MenuAction::ListBooks,
                    "borrow_book" => MenuAction::BorrowBook,
                    "return_book" => MenuAction::ReturnBook,
                    "list_authors" => MenuAction::ListAuthors,
                    "save" => MenuAction::Save,
                    "load" => MenuAction::Load,
                    "quit" => MenuAction::Quit,
                    _ => MenuAction::None,
                }
            }
            _ => MenuAction::None,
        }
    }
}

impl<'a> Widget for MainMenu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Layout principal: titre + menu + aide
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(1),
            ])
            .split(area);

        // Titre
        let title = Paragraph::new(Line::from(vec![
            Span::styled("🏛️  ", Style::default().fg(Color::Cyan)),
            Span::styled(
                "Bibliothèque",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
        title.render(chunks[0], buf);

        // Menu
        let items: Vec<ListItem> = MENU_ITEMS
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let content = if i == self.state.selected {
                    Line::from(vec![
                        Span::styled("> ", Style::default().fg(Color::Cyan)),
                        Span::styled(
                            item.1,
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ])
                } else {
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(item.1, Style::default().fg(Color::White)),
                    ])
                };
                ListItem::new(content)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Menu Principal")
                .border_style(Style::default().fg(Color::Blue)),
        );

        list.render(chunks[1], buf);

        // Aide
        let help = Paragraph::new(Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
            Span::raw(":Navigate "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(":Select "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(":Quit"),
        ]))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

        help.render(chunks[2], buf);
    }
}
