mod models;
mod services;

use models::{Auteur, Livre};
use services::Bibliotheque;
use std::io::{self, Write};

fn afficher_menu() {
    println!("\n=== Menu ===");
    println!("1. Ajouter un livre");
    println!("2. Lister les livres");
    println!("3. Emprunter un livre");
    println!("4. Retourner un livre");
    println!("5. Ajouter un auteur");
    println!("6. Lister les auteurs");
    println!("7. Sauvegarder");
    println!("8. Charger");
    println!("9. Quitter");
    print!("Votre choix: ");
    io::stdout().flush().unwrap();
}

fn lire_ligne() -> String {
    // MUTABILITÉ: `input` est déclaré mutable avec `mut` car read_line()
    // a besoin de modifier la String pour y ajouter l'entrée utilisateur
    let mut input = String::new();

    // OWNERSHIP: `read_line` emprunte `input` de façon mutable (&mut)
    // sans prendre l'ownership, donc on peut continuer à utiliser `input` après
    io::stdin().read_line(&mut input).unwrap();

    // OWNERSHIP: `trim()` retourne une référence &str, puis `to_string()`
    // crée une nouvelle String dont l'ownership est transféré au caller
    input.trim().to_string()
}

// MUTABILITÉ: &mut Bibliotheque car on va ajouter un livre (modification)
fn ajouter_livre_interactif(biblio: &mut Bibliotheque) {
    print!("ID: ");
    io::stdout().flush().unwrap();
    let id: u32 = lire_ligne().parse().unwrap_or(0);

    print!("Titre: ");
    io::stdout().flush().unwrap();
    let titre = lire_ligne();

    print!("Auteur id: ");
    io::stdout().flush().unwrap();
    let auteur_id: u32 = lire_ligne().parse().unwrap_or(0);

    print!("Année: ");
    io::stdout().flush().unwrap();
    let annee: u32 = lire_ligne().parse().unwrap_or(0);

    // OWNERSHIP: Livre::new() prend ownership des Strings (titre, auteur)
    // et retourne un Livre dont l'ownership est transféré à `livre`
    let livre = Livre::new(id, titre, auteur_id, annee);

    // OWNERSHIP: `livre` est MOVE dans ajouter_livre()
    // Après cet appel, `livre` n'est plus accessible (ownership transféré)
    // MUTABILITÉ: ajouter_livre prend &mut self, donc biblio doit être mut
    biblio.ajouter_livre(livre);
    match biblio.associer_livre_auteur(id, auteur_id) {
        Ok(()) => println!("✓ Livre associé à l'auteur"),
        Err(e) => eprintln!("✗ Erreur: {}", e),
    }
}

// MUTABILITÉ: &mut Bibliotheque car on modifie l'état d'un livre
fn emprunter_livre_interactif(biblio: &mut Bibliotheque) {
    print!("ID du livre à emprunter: ");
    io::stdout().flush().unwrap();
    let id: u32 = lire_ligne().parse().unwrap_or(0);

    // MUTABILITÉ: emprunter_livre prend &mut self car il modifie
    // l'état interne (le champ `emprunte` d'un Livre)
    match biblio.emprunter_livre(id) {
        Ok(_) => println!("Livre emprunté avec succès"),
        // OWNERSHIP: String `e` est MOVE hors du Result
        // Le {} dans println utilise Display qui emprunte e (&e implicite)
        Err(e) => println!("Erreur: {}", e),
    }
}

// MUTABILITÉ: &mut Bibliotheque car on modifie l'état d'un livre
fn retourner_livre_interactif(biblio: &mut Bibliotheque) {
    print!("ID du livre à retourner: ");
    io::stdout().flush().unwrap();
    let id: u32 = lire_ligne().parse().unwrap_or(0);

    match biblio.retourner_livre(id) {
        Ok(_) => println!("Livre retourné avec succès"),
        Err(e) => println!("Erreur: {}", e),
    }
}

fn ajouter_auteur_interactif(biblio: &mut Bibliotheque) {
    print!("ID: ");
    io::stdout().flush().unwrap();
    let id: u32 = lire_ligne().parse().unwrap_or(0);

    print!("Prénom: ");
    io::stdout().flush().unwrap();
    let prenom = lire_ligne();

    print!("Nom: ");
    io::stdout().flush().unwrap();
    let nom = lire_ligne();

    let auteur = Auteur::new(id, prenom, nom);
    biblio.ajouter_auteur(auteur);
}

fn sauvegarder_interactif(biblio: &Bibliotheque, fichier: &str) {
    if let Err(e) = biblio.sauvegarder(fichier) {
        println!("Erreur lors de la sauvegarde: {}", e);
    }
}

// MUTABILITÉ: &mut Bibliotheque car on va remplacer tout son contenu
// LIFETIME: fichier a un lifetime 'static (string literal depuis main)
fn charger_interactif(biblio: &mut Bibliotheque, fichier: &str) {
    match Bibliotheque::charger(fichier) {
        Ok(b) => {
            // OWNERSHIP: l'ancienne Bibliotheque est DROP (libérée de la mémoire)
            // `b` est MOVE dans `*biblio` via déréférencement
            // On remplace le contenu pointé par la référence mutable
            *biblio = b;
        }
        Err(e) => println!("Erreur lors du chargement: {}", e),
    }
}

fn main() {
    // MUTABILITÉ: `biblio` doit être mutable car on va modifier son état
    // (ajouter/emprunter/retourner des livres)
    let mut biblio = Bibliotheque::new();

    // OWNERSHIP: String literal &str stockée dans le binaire (lifetime 'static)
    // Pas d'allocation heap nécessaire, vit pendant toute l'exécution
    let fichier_sauvegarde = "bibliotheque.json";

    loop {
        afficher_menu();
        // OWNERSHIP: lire_ligne() retourne une String nouvellement créée
        // dont l'ownership est transféré à `choix`
        let choix = lire_ligne();

        match choix.as_str() {
            "1" => {
                ajouter_livre_interactif(&mut biblio);
            }
            "2" => {
                biblio.lister_livres();
            }
            "3" => {
                emprunter_livre_interactif(&mut biblio);
            }
            "4" => {
                retourner_livre_interactif(&mut biblio);
            }
            "5" => {
                ajouter_auteur_interactif(&mut biblio);
            }
            "6" => {
                biblio.lister_auteurs();
            }
            "7" => {
                sauvegarder_interactif(&biblio, fichier_sauvegarde);
            }
            "8" => {
                charger_interactif(&mut biblio, fichier_sauvegarde);
            }
            "9" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide"),
        }
    }
}
