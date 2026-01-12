use cursive::views::Dialog;
use cursive::Cursive;

use crate::services::Bibliotheque;
use super::SharedBibliotheque;

pub fn sauvegarder_bibliotheque(siv: &mut Cursive, biblio: SharedBibliotheque) {
    let b = biblio.lock().unwrap();
    match b.sauvegarder("bibliotheque.json") {
        Ok(_) => {
            siv.add_layer(
                Dialog::info("Bibliothèque sauvegardée dans bibliotheque.json").title("✅ Succès"),
            );
        }
        Err(e) => {
            siv.add_layer(
                Dialog::info(format!("Erreur lors de la sauvegarde: {}", e)).title("⚠️  Erreur"),
            );
        }
    }
}

pub fn charger_bibliotheque(siv: &mut Cursive, biblio: SharedBibliotheque) {
    match Bibliotheque::charger("bibliotheque.json") {
        Ok(nouvelle_biblio) => {
            let mut b = biblio.lock().unwrap();
            *b = nouvelle_biblio;
            siv.add_layer(
                Dialog::info("Bibliothèque chargée depuis bibliotheque.json").title("✅ Succès"),
            );
        }
        Err(e) => {
            siv.add_layer(
                Dialog::info(format!("Erreur lors du chargement: {}", e)).title("⚠️  Erreur"),
            );
        }
    }
}
