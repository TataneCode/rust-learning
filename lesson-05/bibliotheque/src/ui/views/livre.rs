use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, ScrollView, TextView};
use cursive::Cursive;

use crate::models::Livre;
use super::SharedBibliotheque;

pub fn formulaire_ajouter_livre(siv: &mut Cursive, biblio: SharedBibliotheque) {
    let form = LinearLayout::vertical()
        .child(TextView::new("Remplissez les informations du livre:"))
        .child(DummyView)
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("ID: ").fixed_width(15))
                .child(EditView::new().with_name("id").fixed_width(30)),
        )
        .child(DummyView)
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Titre: ").fixed_width(15))
                .child(EditView::new().with_name("titre").fixed_width(30)),
        )
        .child(DummyView)
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Auteur ID: ").fixed_width(15))
                .child(EditView::new().with_name("auteur_id").fixed_width(30)),
        )
        .child(DummyView)
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Année: ").fixed_width(15))
                .child(EditView::new().with_name("annee").fixed_width(30)),
        );

    siv.add_layer(
        Dialog::around(form)
            .title("📚 Ajouter un livre")
            .button("Ajouter", move |s| {
                let id = s
                    .call_on_name("id", |v: &mut EditView| v.get_content())
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(0);

                let titre = s
                    .call_on_name("titre", |v: &mut EditView| v.get_content())
                    .unwrap()
                    .to_string();

                let auteur_id = s
                    .call_on_name("auteur_id", |v: &mut EditView| v.get_content())
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(0);

                let annee = s
                    .call_on_name("annee", |v: &mut EditView| v.get_content())
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(0);

                if titre.is_empty() {
                    s.add_layer(Dialog::info("Le titre ne peut pas être vide!").title("⚠️  Erreur"));
                    return;
                }

                let livre = Livre::new(id, titre, auteur_id, annee);
                let mut b = biblio.lock().unwrap();
                b.ajouter_livre(livre);

                match b.associer_livre_auteur(id, auteur_id) {
                    Ok(()) => {
                        s.pop_layer();
                        s.add_layer(
                            Dialog::info("Livre ajouté et associé avec succès!").title("✅ Succès"),
                        );
                    }
                    Err(e) => {
                        s.pop_layer();
                        s.add_layer(
                            Dialog::info(format!("Livre ajouté mais: {}", e))
                                .title("⚠️  Attention"),
                        );
                    }
                }
            })
            .button("Annuler", |s| {
                s.pop_layer();
            }),
    );
}

pub fn afficher_liste_livres(siv: &mut Cursive, biblio: SharedBibliotheque) {
    let b = biblio.lock().unwrap();

    let mut list = LinearLayout::vertical();

    if b.get_livres().is_empty() {
        list.add_child(TextView::new("Aucun livre dans la bibliothèque"));
    } else {
        for livre in b.get_livres() {
            let status = if livre.emprunte {
                "● Emprunté"
            } else {
                "○ Disponible"
            };
            let info = format!(
                "#{} - {} ({}) - Auteur ID: {} - {}",
                livre.id, livre.titre, livre.annee, livre.auteur_id, status
            );
            list.add_child(TextView::new(info));
            list.add_child(DummyView);
        }
    }

    siv.add_layer(
        Dialog::around(ScrollView::new(list).fixed_size((80, 20)))
            .title("📖 Liste des livres")
            .button("Fermer", |s| {
                s.pop_layer();
            }),
    );
}

pub fn dialogue_emprunter_livre(siv: &mut Cursive, biblio: SharedBibliotheque) {
    siv.add_layer(
        Dialog::new()
            .title("✋ Emprunter un livre")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Entrez l'ID du livre à emprunter:"))
                    .child(DummyView)
                    .child(EditView::new().with_name("emprunt_id").fixed_width(20)),
            )
            .button("Emprunter", move |s| {
                let id = s
                    .call_on_name("emprunt_id", |v: &mut EditView| v.get_content())
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(0);

                let mut b = biblio.lock().unwrap();
                match b.emprunter_livre(id) {
                    Ok(_) => {
                        s.pop_layer();
                        s.add_layer(Dialog::info("Livre emprunté avec succès!").title("✅ Succès"));
                    }
                    Err(e) => {
                        s.add_layer(Dialog::info(format!("Erreur: {}", e)).title("⚠️  Erreur"));
                    }
                }
            })
            .button("Annuler", |s| {
                s.pop_layer();
            }),
    );
}

pub fn dialogue_retourner_livre(siv: &mut Cursive, biblio: SharedBibliotheque) {
    siv.add_layer(
        Dialog::new()
            .title("📥 Retourner un livre")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Entrez l'ID du livre à retourner:"))
                    .child(DummyView)
                    .child(EditView::new().with_name("retour_id").fixed_width(20)),
            )
            .button("Retourner", move |s| {
                let id = s
                    .call_on_name("retour_id", |v: &mut EditView| v.get_content())
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(0);

                let mut b = biblio.lock().unwrap();
                match b.retourner_livre(id) {
                    Ok(_) => {
                        s.pop_layer();
                        s.add_layer(Dialog::info("Livre retourné avec succès!").title("✅ Succès"));
                    }
                    Err(e) => {
                        s.add_layer(Dialog::info(format!("Erreur: {}", e)).title("⚠️  Erreur"));
                    }
                }
            })
            .button("Annuler", |s| {
                s.pop_layer();
            }),
    );
}
