use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auteur {
    pub id: u32,
    pub prenom: String,
    pub nom: String,
    pub livres: Vec<u32>,
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

    pub fn add_livre(&mut self, livre_id: u32) {
        self.livres.push(livre_id);
    }
}