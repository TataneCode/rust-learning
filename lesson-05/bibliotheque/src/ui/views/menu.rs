use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;

use super::SharedBibliotheque;
use super::{
    afficher_liste_auteurs, afficher_liste_livres, charger_bibliotheque,
    dialogue_emprunter_livre, dialogue_retourner_livre, formulaire_ajouter_auteur,
    formulaire_ajouter_livre, sauvegarder_bibliotheque,
};

// Enum représentant les options du menu
#[derive(Clone, Copy)]
enum MenuOption {
    AjouterLivre,
    ListerLivres,
    EmprunterLivre,
    RetournerLivre,
    AjouterAuteur,
    ListerAuteurs,
    Sauvegarder,
    Charger,
    Quitter,
}

pub fn afficher_menu_principal(siv: &mut Cursive, biblio: SharedBibliotheque) {
    let mut menu = SelectView::new().h_align(HAlign::Center);

    menu.add_item("📚 Ajouter un livre", MenuOption::AjouterLivre);
    menu.add_item("📖 Lister les livres", MenuOption::ListerLivres);
    menu.add_item("✋ Emprunter un livre", MenuOption::EmprunterLivre);
    menu.add_item("📥 Retourner un livre", MenuOption::RetournerLivre);
    menu.add_item("✍️  Ajouter un auteur", MenuOption::AjouterAuteur);
    menu.add_item("👥 Lister les auteurs", MenuOption::ListerAuteurs);
    menu.add_item("💾 Sauvegarder", MenuOption::Sauvegarder);
    menu.add_item("📂 Charger", MenuOption::Charger);
    menu.add_item("🚪 Quitter", MenuOption::Quitter);

    let b = biblio.clone();
    menu.set_on_submit(move |s, option| match option {
        MenuOption::AjouterLivre => formulaire_ajouter_livre(s, b.clone()),
        MenuOption::ListerLivres => afficher_liste_livres(s, b.clone()),
        MenuOption::EmprunterLivre => dialogue_emprunter_livre(s, b.clone()),
        MenuOption::RetournerLivre => dialogue_retourner_livre(s, b.clone()),
        MenuOption::AjouterAuteur => formulaire_ajouter_auteur(s, b.clone()),
        MenuOption::ListerAuteurs => afficher_liste_auteurs(s, b.clone()),
        MenuOption::Sauvegarder => sauvegarder_bibliotheque(s, b.clone()),
        MenuOption::Charger => charger_bibliotheque(s, b.clone()),
        MenuOption::Quitter => s.quit(),
    });

    siv.add_layer(
        Dialog::around(menu.scrollable().fixed_size((50, 12)))
            .title("🏛️  Bibliothèque - Menu Principal")
            .button("Quitter", |s| s.quit()),
    );
}
