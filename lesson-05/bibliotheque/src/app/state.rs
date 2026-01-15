use crate::services::Bibliotheque;
use std::sync::{Arc, Mutex};

pub type SharedBibliotheque = Arc<Mutex<Bibliotheque>>;

// État pour les formulaires
#[derive(Default)]
pub struct FormState {
    pub fields: Vec<String>,
    pub focused_field: usize,
}

// État pour les listes
#[derive(Default)]
pub struct ListState {
    pub selected: usize,
    pub scroll_offset: usize,
}

// État pour les messages/dialogs
pub struct MessageState {
    pub title: String,
    pub message: String,
    pub is_error: bool,
}

// Différents types d'écrans de l'application
pub enum Screen {
    MainMenu(ListState),
    AddBook(FormState),
    ListBooks(ListState),
    BorrowBook(FormState),
    ReturnBook(FormState),
    AddAuthor(FormState),
    ListAuthors(ListState),
    Message(MessageState),
}

impl Screen {
    pub fn main_menu() -> Self {
        Screen::MainMenu(ListState::default())
    }

    pub fn add_book() -> Self {
        Screen::AddBook(FormState {
            fields: vec![String::new(); 4], // ID, Titre, Auteur ID, Année
            focused_field: 0,
        })
    }

    pub fn list_books() -> Self {
        Screen::ListBooks(ListState::default())
    }

    pub fn borrow_book() -> Self {
        Screen::BorrowBook(FormState {
            fields: vec![String::new(); 1], // ID du livre
            focused_field: 0,
        })
    }

    pub fn return_book() -> Self {
        Screen::ReturnBook(FormState {
            fields: vec![String::new(); 1], // ID du livre
            focused_field: 0,
        })
    }

    pub fn add_author() -> Self {
        Screen::AddAuthor(FormState {
            fields: vec![String::new(); 3], // ID, Prénom, Nom
            focused_field: 0,
        })
    }

    pub fn list_authors() -> Self {
        Screen::ListAuthors(ListState::default())
    }

    pub fn message(title: String, message: String, is_error: bool) -> Self {
        Screen::Message(MessageState {
            title,
            message,
            is_error,
        })
    }
}
