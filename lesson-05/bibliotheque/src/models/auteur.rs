pub use super::Livre;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auteur {
    pub id: u32,
    pub prenom: String,
    pub nom: String,
    pub livres: Vec<Livre>,
}

impl Auteur {
    pub fn new(id: u32, prenom: String, nom: String) -> Self {
        Auteur {
            id,
            prenom,
            nom,
            livres: Vec::new(),
        }
    }

    pub fn add_livre(&mut self, livre: Livre) {
        self.livres.push(livre);
    }

    pub fn afficher(&self) {
        println!("Auteur #{} - {} {}", self.id, self.prenom, self.nom);
        if self.livres.is_empty() {
            println!("  Aucun livre");
        } else {
            println!("  Livres:");
            for livre in &self.livres {
                println!("    - {} ({})", livre.titre, livre.annee);
            }
        }
    }
}