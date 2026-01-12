mod models;
mod services;
mod ui;

use services::Bibliotheque;
use std::sync::{Arc, Mutex};
use ui::{afficher_menu_principal, setup_theme};

fn main() {
    let mut siv = cursive::default();

    // Configuration du thème avec couleurs douces
    siv.set_theme(setup_theme());

    // Création de la bibliothèque partagée
    // Arc<Mutex<T>> permet le partage thread-safe:
    // - Arc (Atomic Reference Counted) = smart pointer avec compteur de références
    // - Mutex = garantit l'accès exclusif lors des modifications
    let biblio = Arc::new(Mutex::new(Bibliotheque::new()));

    // Affichage du menu principal
    afficher_menu_principal(&mut siv, biblio);

    // Lancement de l'application
    siv.run();
}
