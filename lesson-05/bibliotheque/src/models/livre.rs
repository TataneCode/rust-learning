use serde::{Serialize, Deserialize};

// OWNERSHIP: Clone permet de dupliquer un Livre (copie profonde des Strings)
// Utile quand on veut créer une copie indépendante sans transférer l'ownership
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Livre {
    pub id: u32,

    // OWNERSHIP: String possède ses données sur le heap
    // Quand un Livre est drop, ses Strings sont aussi drop (libérés)
    pub titre: String,
    pub auteur_id: u32,

    pub annee: u32,

    // MUTABILITÉ: Ce champ sera modifié lors de l'emprunt/retour
    // même si on a une référence &mut vers le Livre
    #[serde(default)]
    pub emprunte: bool,
}

impl Livre {
    // OWNERSHIP: Cette fonction prend ownership des Strings passées en paramètres
    // et les MOVE dans la structure Livre retournée
    pub fn new(id: u32, titre: String, auteur_id: u32, annee: u32) -> Self {
        Livre {
            id,
            // OWNERSHIP: titre est MOVE dans la struct
            // Les Strings ne sont pas copiées, juste déplacées (efficace!)
            titre,
            auteur_id,
            annee,
            emprunte: false,
        }
    }

    // LIFETIME: &self a un lifetime implicite 'a: fn afficher<'a>(&'a self)
    // La référence est valide pendant l'appel de la fonction
    // OWNERSHIP: Emprunt immuable, on ne modifie pas et on ne prend pas l'ownership
    pub fn afficher(&self) {
        let statut = if self.emprunte { "Emprunté" } else { "Disponible" };

        // OWNERSHIP: self.titre et self.auteur sont empruntés (&String)
        // puis coercés en &str pour println! - pas de copie, juste des références
        println!("#{} - {} par {} ({}) - {}",
                 self.id, self.titre, self.auteur_id, self.annee, statut);
    }
}