use crate::models::{Livre, Auteur};
use std::fs;
use std::io;

pub struct Bibliotheque {
    livres: Vec<Livre>,
    auteurs: Vec<Auteur>,
}

impl Bibliotheque {
    pub fn new() -> Self {
        Bibliotheque {
            livres: Vec::new(),
            auteurs: Vec::new(),
        }
    }
    
    // MUTABILITÉ: &mut self car on modifie le Vec interne
    // OWNERSHIP: `livre` est MOVE dans cette fonction (pas de &)
    // puis MOVE dans le Vec via push - très efficace, pas de copie!
    pub fn ajouter_livre(&mut self, livre: Livre) {
        self.livres.push(livre);
        println!("Livre ajouté avec succès");
    }
    
    // LIFETIME: &self a un lifetime implicite qui garantit que la référence
    // est valide pendant tout l'appel de la fonction
    // OWNERSHIP: Emprunt immuable - on lit sans modifier ni prendre ownership
    pub fn lister_livres(&self) {
        if self.livres.is_empty() {
            println!("Aucun livre dans la bibliothèque");
            return;
        }

        println!("\n=== Bibliothèque ===");
        // OWNERSHIP: &self.livres crée un itérateur de références (&Livre)
        // On emprunte chaque livre sans en prendre l'ownership
        // Si on écrivait `for livre in self.livres`, ça MOVE les livres hors du Vec!
        for livre in &self.livres {
            // LIFETIME: `livre` a le type &Livre avec un lifetime lié à &self
            livre.afficher();
        }
    }
    
    // MUTABILITÉ: &mut self car on va modifier un Livre dans le Vec
    pub fn emprunter_livre(&mut self, id: u32) -> Result<(), String> {
        // MUTABILITÉ: iter_mut() retourne un itérateur de &mut Livre
        // Ceci est crucial car on doit modifier le champ `emprunte`
        // LIFETIME: les &mut Livre ont un lifetime lié à &mut self
        match self.livres.iter_mut().find(|l| l.id == id) {
            // OWNERSHIP: `livre` est de type &mut Livre (référence mutable)
            // On peut modifier ses champs sans prendre ownership du Livre
            Some(livre) => {
                if livre.emprunte {
                    Err(String::from("Ce livre est déjà emprunté"))
                } else {
                    // MUTABILITÉ: On modifie le champ via la référence mutable
                    livre.emprunte = true;
                    Ok(())
                }
            }
            None => Err(String::from("Livre non trouvé")),
        }
    }
    
    // Même pattern que emprunter_livre
    // MUTABILITÉ: &mut self + iter_mut() pour modifier un Livre
    pub fn retourner_livre(&mut self, id: u32) -> Result<(), String> {
        match self.livres.iter_mut().find(|l| l.id == id) {
            Some(livre) => {
                if !livre.emprunte {
                    Err(String::from("Ce livre n'est pas emprunté"))
                } else {
                    livre.emprunte = false;
                    Ok(())
                }
            }
            None => Err(String::from("Livre non trouvé")),
        }
    }

    pub fn ajouter_auteur(&mut self, auteur: Auteur) {
        self.auteurs.push(auteur);
        println!("Auteur ajouté avec succès");
    }

    pub fn lister_auteurs(&self) {
        if self.auteurs.is_empty() {
            println!("Aucun auteur dans la bibliothèque");
            return;
        }

        println!("\n=== Auteurs ===");
        for auteur in &self.auteurs {
            auteur.afficher();
        }
    }

    pub fn trouver_auteur_mut(&mut self, id: u32) -> Option<&mut Auteur> {
        self.auteurs.iter_mut().find(|a| a.id == id)
    }

    pub fn associer_livre_auteur(&mut self, livre_id: u32, auteur_id: u32) -> Result<(), String> {
        let livre = self.livres.iter()
            .find(|l| l.id == livre_id)
            .ok_or_else(|| String::from("Livre non trouvé"))?
            .clone();

        let auteur = self.trouver_auteur_mut(auteur_id)
            .ok_or_else(|| String::from("Auteur non trouvé"))?;

        auteur.add_livre(livre);
        Ok(())
    }

    pub fn sauvegarder(&self, fichier: &str) -> io::Result<()> {
        // OWNERSHIP: to_string_pretty emprunte &self.livres et retourne une
        // nouvelle String dont on prend ownership
        let json = serde_json::to_string_pretty(&self.auteurs)?;

        // OWNERSHIP: `json` est MOVE dans fs::write (String implements AsRef<[u8]>)
        fs::write(fichier, json)?;
        println!("Bibliothèque sauvegardée dans {}", fichier);
        Ok(())
    }

    // LIFETIME: Même principe que sauvegarder pour &str
    // Pas de &self car c'est une fonction associée (constructeur alternatif)
    pub fn charger(fichier: &str) -> io::Result<Self> {
        let contenu = fs::read_to_string(fichier)?;
        let auteurs: Vec<Auteur> = serde_json::from_str(&contenu)?;
        let livres: Vec<Livre> = auteurs
            .iter()
            .flat_map(|auteur| auteur.livres.clone())
            .collect();

        println!("Bibliothèque chargée depuis {}", fichier);
        Ok(Bibliotheque {
            livres,
            auteurs,
        })
    }
}