pub mod state;

use crate::models::{Auteur, Livre};
use crate::ui::screens::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use state::{Screen, SharedBibliotheque};

pub struct App {
    pub biblio: SharedBibliotheque,
    pub screen_stack: Vec<Screen>,
    pub should_quit: bool,
}

impl App {
    pub fn new(biblio: SharedBibliotheque) -> Self {
        App {
            biblio,
            screen_stack: vec![Screen::main_menu()],
            should_quit: false,
        }
    }

    pub fn current_screen(&self) -> &Screen {
        self.screen_stack
            .last()
            .expect("Screen stack should never be empty")
    }

    pub fn current_screen_mut(&mut self) -> &mut Screen {
        self.screen_stack
            .last_mut()
            .expect("Screen stack should never be empty")
    }

    pub fn push_screen(&mut self, screen: Screen) {
        self.screen_stack.push(screen);
    }

    pub fn pop_screen(&mut self) {
        if self.screen_stack.len() > 1 {
            self.screen_stack.pop();
        }
    }

    pub fn replace_screen(&mut self, screen: Screen) {
        if !self.screen_stack.is_empty() {
            self.screen_stack.pop();
        }
        self.screen_stack.push(screen);
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // Cloner la référence Arc avant le match pour éviter les conflits d'emprunt
        let biblio = self.biblio.clone();

        let current = self.screen_stack.last_mut().expect("Screen stack should never be empty");

        match current {
            Screen::MainMenu(state) => {
                let menu = MainMenu::new(state);
                frame.render_widget(menu, area);
            }
            Screen::AddBook(state) => {
                let mut screen = AddBookScreen::new(state);
                screen.render(area, frame.buffer_mut());
            }
            Screen::ListBooks(state) => {
                let screen = ListBooksScreen::new(&biblio, state);
                frame.render_widget(screen, area);
            }
            Screen::BorrowBook(state) => {
                let mut screen = BorrowBookScreen::new(state);
                screen.render(area, frame.buffer_mut());
            }
            Screen::ReturnBook(state) => {
                let mut screen = ReturnBookScreen::new(state);
                screen.render(area, frame.buffer_mut());
            }
            Screen::AddAuthor(state) => {
                let mut screen = AddAuthorScreen::new(state);
                screen.render(area, frame.buffer_mut());
            }
            Screen::ListAuthors(state) => {
                let screen = ListAuthorsScreen::new(&biblio, state);
                frame.render_widget(screen, area);
            }
            Screen::Message(state) => {
                let screen = MessageScreen::new(state);
                frame.render_widget(screen, area);
            }
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> std::io::Result<()> {
        // Quitter avec Ctrl+C
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return Ok(());
        }

        // Gestion Esc pour retour arrière (sauf pour les messages)
        if key.code == KeyCode::Esc {
            match self.current_screen() {
                Screen::Message(_) => {
                    self.pop_screen();
                }
                _ => {
                    if self.screen_stack.len() > 1 {
                        self.pop_screen();
                    } else {
                        self.should_quit = true;
                    }
                }
            }
            return Ok(());
        }

        // Enter pour fermer les messages
        if key.code == KeyCode::Enter {
            if let Screen::Message(_) = self.current_screen() {
                self.pop_screen();
                return Ok(());
            }
        }

        // Cloner la référence Arc avant le match pour éviter les conflits d'emprunt
        let biblio = self.biblio.clone();

        // Dispatcher selon l'écran actuel
        let current = self.screen_stack.last_mut().expect("Screen stack should never be empty");

        match current {
            Screen::MainMenu(state) => {
                let mut menu = MainMenu::new(state);
                match menu.handle_key_event(key) {
                    MenuAction::AddAuthor => self.push_screen(Screen::add_author()),
                    MenuAction::AddBook => self.push_screen(Screen::add_book()),
                    MenuAction::ListBooks => self.push_screen(Screen::list_books()),
                    MenuAction::BorrowBook => self.push_screen(Screen::borrow_book()),
                    MenuAction::ReturnBook => self.push_screen(Screen::return_book()),
                    MenuAction::ListAuthors => self.push_screen(Screen::list_authors()),
                    MenuAction::Save => self.handle_save(),
                    MenuAction::Load => self.handle_load(),
                    MenuAction::Quit => self.should_quit = true,
                    MenuAction::None => {}
                }
            }
            Screen::AddBook(state) => {
                let mut screen = AddBookScreen::new(state);
                if let Some(BookAction::SubmitAdd(values)) = screen.handle_key_event(key) {
                    self.handle_add_book(values);
                }
            }
            Screen::ListBooks(state) => {
                let mut screen = ListBooksScreen::new(&biblio, state);
                screen.handle_key_event(key);
            }
            Screen::BorrowBook(state) => {
                let mut screen = BorrowBookScreen::new(state);
                if let Some(BookAction::SubmitBorrow(id_str)) = screen.handle_key_event(key) {
                    self.handle_borrow_book(id_str);
                }
            }
            Screen::ReturnBook(state) => {
                let mut screen = ReturnBookScreen::new(state);
                if let Some(BookAction::SubmitReturn(id_str)) = screen.handle_key_event(key) {
                    self.handle_return_book(id_str);
                }
            }
            Screen::AddAuthor(state) => {
                let mut screen = AddAuthorScreen::new(state);
                if let Some(AuthorAction::SubmitAdd(values)) = screen.handle_key_event(key) {
                    self.handle_add_author(values);
                }
            }
            Screen::ListAuthors(state) => {
                let mut screen = ListAuthorsScreen::new(&biblio, state);
                screen.handle_key_event(key);
            }
            Screen::Message(_) => {
                // Géré plus haut (Esc ou Enter pour fermer)
            }
        }

        Ok(())
    }

    fn handle_add_book(&mut self, values: Vec<String>) {
        let id = values[0].parse::<u32>().unwrap_or(0);
        let titre = values[1].clone();
        let auteur_id = values[2].parse::<u32>().unwrap_or(0);
        let annee = values[3].parse::<u32>().unwrap_or(0);

        if titre.is_empty() {
            self.push_screen(Screen::message(
                "Erreur".to_string(),
                "Le titre ne peut pas être vide!".to_string(),
                true,
            ));
            return;
        }

        let livre = Livre::new(id, titre, auteur_id, annee);
        let mut b = self.biblio.lock().unwrap();
        b.ajouter_livre(livre);

        match b.associer_livre_auteur(id, auteur_id) {
            Ok(()) => {
                drop(b);
                self.pop_screen();
                self.push_screen(Screen::message(
                    "Succès".to_string(),
                    "Livre ajouté et associé avec succès!".to_string(),
                    false,
                ));
            }
            Err(e) => {
                drop(b);
                self.pop_screen();
                self.push_screen(Screen::message(
                    "Attention".to_string(),
                    format!("Livre ajouté mais: {}", e),
                    false,
                ));
            }
        }
    }

    fn handle_borrow_book(&mut self, id_str: String) {
        let id = match id_str.parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                self.push_screen(Screen::message(
                    "Erreur".to_string(),
                    "ID invalide".to_string(),
                    true,
                ));
                return;
            }
        };

        let mut b = self.biblio.lock().unwrap();
        match b.emprunter_livre(id) {
            Ok(_) => {
                drop(b);
                self.pop_screen();
                self.push_screen(Screen::message(
                    "Succès".to_string(),
                    "Livre emprunté avec succès!".to_string(),
                    false,
                ));
            }
            Err(e) => {
                drop(b);
                self.push_screen(Screen::message(
                    "Erreur".to_string(),
                    format!("Erreur: {}", e),
                    true,
                ));
            }
        }
    }

    fn handle_return_book(&mut self, id_str: String) {
        let id = match id_str.parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                self.push_screen(Screen::message(
                    "Erreur".to_string(),
                    "ID invalide".to_string(),
                    true,
                ));
                return;
            }
        };

        let mut b = self.biblio.lock().unwrap();
        match b.retourner_livre(id) {
            Ok(_) => {
                drop(b);
                self.pop_screen();
                self.push_screen(Screen::message(
                    "Succès".to_string(),
                    "Livre retourné avec succès!".to_string(),
                    false,
                ));
            }
            Err(e) => {
                drop(b);
                self.push_screen(Screen::message(
                    "Erreur".to_string(),
                    format!("Erreur: {}", e),
                    true,
                ));
            }
        }
    }

    fn handle_add_author(&mut self, values: Vec<String>) {
        let id = values[0].parse::<u32>().unwrap_or(0);
        let prenom = values[1].clone();
        let nom = values[2].clone();

        if prenom.is_empty() || nom.is_empty() {
            self.push_screen(Screen::message(
                "Erreur".to_string(),
                "Le prénom et le nom ne peuvent pas être vides!".to_string(),
                true,
            ));
            return;
        }

        let auteur = Auteur::new(id, prenom, nom);
        let mut b = self.biblio.lock().unwrap();
        b.ajouter_auteur(auteur);
        drop(b);

        self.pop_screen();
        self.push_screen(Screen::message(
            "Succès".to_string(),
            "Auteur ajouté avec succès!".to_string(),
            false,
        ));
    }

    fn handle_save(&mut self) {
        let b = self.biblio.lock().unwrap();
        match b.sauvegarder("bibliotheque.json") {
            Ok(_) => {
                drop(b);
                self.push_screen(Screen::message(
                    "Succès".to_string(),
                    "Bibliothèque sauvegardée dans bibliotheque.json".to_string(),
                    false,
                ));
            }
            Err(e) => {
                drop(b);
                self.push_screen(Screen::message(
                    "Erreur".to_string(),
                    format!("Erreur lors de la sauvegarde: {}", e),
                    true,
                ));
            }
        }
    }

    fn handle_load(&mut self) {
        use crate::services::Bibliotheque;

        match Bibliotheque::charger("bibliotheque.json") {
            Ok(nouvelle_biblio) => {
                let mut b = self.biblio.lock().unwrap();
                *b = nouvelle_biblio;
                drop(b);
                self.push_screen(Screen::message(
                    "Succès".to_string(),
                    "Bibliothèque chargée depuis bibliotheque.json".to_string(),
                    false,
                ));
            }
            Err(e) => {
                self.push_screen(Screen::message(
                    "Erreur".to_string(),
                    format!("Erreur lors du chargement: {}", e),
                    true,
                ));
            }
        }
    }
}

