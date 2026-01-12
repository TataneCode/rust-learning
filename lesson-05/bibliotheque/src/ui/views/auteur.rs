use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, ScrollView, TextView};
use cursive::Cursive;

use crate::models::Auteur;
use super::SharedBibliotheque;

pub fn formulaire_ajouter_auteur(siv: &mut Cursive, biblio: SharedBibliotheque) {
    let form = LinearLayout::vertical()
        .child(TextView::new("Remplissez les informations de l'auteur:"))
        .child(DummyView)
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("ID: ").fixed_width(15))
                .child(EditView::new().with_name("id").fixed_width(30)),
        )
        .child(DummyView)
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Prénom: ").fixed_width(15))
                .child(EditView::new().with_name("prenom").fixed_width(30)),
        )
        .child(DummyView)
        .child(
            LinearLayout::horizontal()
                .child(TextView::new("Nom: ").fixed_width(15))
                .child(EditView::new().with_name("nom").fixed_width(30)),
        );

    siv.add_layer(
        Dialog::around(form)
            .title("✍️  Ajouter un auteur")
            .button("Ajouter", move |s| {
                let id = s
                    .call_on_name("id", |v: &mut EditView| v.get_content())
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(0);

                let prenom = s
                    .call_on_name("prenom", |v: &mut EditView| v.get_content())
                    .unwrap()
                    .to_string();

                let nom = s
                    .call_on_name("nom", |v: &mut EditView| v.get_content())
                    .unwrap()
                    .to_string();

                if prenom.is_empty() || nom.is_empty() {
                    s.add_layer(
                        Dialog::info("Le prénom et le nom ne peuvent pas être vides!")
                            .title("⚠️  Erreur"),
                    );
                    return;
                }

                let auteur = Auteur::new(id, prenom, nom);
                let mut b = biblio.lock().unwrap();
                b.ajouter_auteur(auteur);

                s.pop_layer();
                s.add_layer(Dialog::info("Auteur ajouté avec succès!").title("✅ Succès"));
            })
            .button("Annuler", |s| {
                s.pop_layer();
            }),
    );
}

pub fn afficher_liste_auteurs(siv: &mut Cursive, biblio: SharedBibliotheque) {
    let b = biblio.lock().unwrap();

    let mut list = LinearLayout::vertical();

    if b.get_auteurs().is_empty() {
        list.add_child(TextView::new("Aucun auteur dans la bibliothèque"));
    } else {
        for auteur in b.get_auteurs() {
            // En-tête auteur
            let header = format!(
                "#{} - {} {} ({} livre(s))",
                auteur.id,
                auteur.prenom,
                auteur.nom,
                auteur.livres.len()
            );
            list.add_child(TextView::new(header));

            // Liste des livres de l'auteur
            if !auteur.livres.is_empty() {
                for &livre_id in &auteur.livres {
                    match b.get_livres().iter().find(|l| l.id == livre_id) {
                        Some(livre) => {
                            let livre_info = format!("   └─ {} ({})", livre.titre, livre.annee);
                            list.add_child(TextView::new(livre_info));
                        }
                        None => {
                            list.add_child(TextView::new(format!(
                                "   └─ Livre ID {} (non trouvé)",
                                livre_id
                            )));
                        }
                    }
                }
            }

            list.add_child(DummyView);
        }
    }

    siv.add_layer(
        Dialog::around(ScrollView::new(list).fixed_size((80, 20)))
            .title("👥 Liste des auteurs")
            .button("Fermer", |s| {
                s.pop_layer();
            }),
    );
}
