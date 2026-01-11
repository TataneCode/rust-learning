# Leçon 5 : Sérialisation avec Serde, Structs et Modularisation

## Introduction

Cette leçon couvre trois concepts essentiels pour structurer des applications Rust professionnelles :
- Les **structs** pour modéliser vos données
- **Serde** pour la sérialisation/désérialisation (JSON, TOML, etc.)
- La **modularisation** pour organiser votre code en plusieurs fichiers

## 1. Les Structs en Profondeur

### Définition et Utilisation de Base

```rust
// Struct classique
struct Utilisateur {
    nom: String,
    email: String,
    age: u8,
    actif: bool,
}

// Création d'une instance
let user = Utilisateur {
    nom: String::from("Alice"),
    email: String::from("alice@example.com"),
    age: 30,
    actif: true,
};

println!("Nom: {}", user.nom);
```

### Structs avec Méthodes

```rust
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    // Méthode associée (constructeur)
    fn new(largeur: u32, hauteur: u32) -> Self {
        Rectangle { largeur, hauteur }
    }
    
    // Méthode d'instance
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }
    
    // Méthode mutante
    fn doubler(&mut self) {
        self.largeur *= 2;
        self.hauteur *= 2;
    }
}

// Utilisation
let mut rect = Rectangle::new(10, 20);
println!("Aire: {}", rect.aire());
rect.doubler();
```

### Tuple Structs et Unit Structs

```rust
// Tuple struct
struct Couleur(u8, u8, u8);
let rouge = Couleur(255, 0, 0);

// Unit struct (sans champs)
struct Marqueur;
```

## 2. Serde : Sérialisation et Désérialisation

### Installation

Ajoutez à votre `Cargo.toml` :

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Dérive de Base

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    nom: String,
    email: String,
    telephone: Option<String>,
}

fn main() {
    // Sérialisation vers JSON
    let contact = Contact {
        nom: String::from("Marie"),
        email: String::from("marie@example.com"),
        telephone: Some(String::from("0123456789")),
    };
    
    let json = serde_json::to_string(&contact).unwrap();
    println!("JSON: {}", json);
    
    // Désérialisation depuis JSON
    let json_str = r#"{"nom":"Pierre","email":"pierre@example.com","telephone":null}"#;
    let contact2: Contact = serde_json::from_str(json_str).unwrap();
    println!("{:?}", contact2);
}
```

### Personnalisation avec Attributs Serde

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Produit {
    #[serde(rename = "productId")]
    id: u32,
    
    #[serde(rename = "productName")]
    nom: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    
    #[serde(default)]
    stock: u32,
}
```

**Attributs utiles :**
- `#[serde(rename = "...")]` : change le nom dans le JSON
- `#[serde(skip_serializing_if = "...")]` : omet le champ selon condition
- `#[serde(default)]` : utilise la valeur par défaut si absent
- `#[serde(skip)]` : ignore complètement le champ

### Gestion des Erreurs avec Result

```rust
use serde_json::Result;

fn lire_config(json_str: &str) -> Result<Contact> {
    serde_json::from_str(json_str)
}

fn main() {
    match lire_config(r#"{"nom":"Test","email":"test@example.com"}"#) {
        Ok(contact) => println!("Contact chargé: {:?}", contact),
        Err(e) => eprintln!("Erreur de parsing: {}", e),
    }
}
```

## 3. Modularisation : Organiser votre Code

### Structure de Projet Recommandée

```
mon_projet/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs (optionnel)
    ├── models/
    │   ├── mod.rs
    │   ├── utilisateur.rs
    │   └── produit.rs
    └── utils/
        ├── mod.rs
        └── validation.rs
```

### Créer des Modules dans Fichiers Séparés

**src/models/mod.rs**
```rust
// Déclare les sous-modules
pub mod utilisateur;
pub mod produit;

// Ré-exporte pour simplifier l'usage
pub use utilisateur::Utilisateur;
pub use produit::Produit;
```

**src/models/utilisateur.rs**
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Utilisateur {
    pub id: u32,
    pub nom: String,
    pub email: String,
}

impl Utilisateur {
    pub fn new(id: u32, nom: String, email: String) -> Self {
        Utilisateur { id, nom, email }
    }
    
    pub fn afficher(&self) {
        println!("Utilisateur #{}: {} ({})", self.id, self.nom, self.email);
    }
}
```

**src/models/produit.rs**
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Produit {
    pub id: u32,
    pub nom: String,
    pub prix: f64,
}

impl Produit {
    pub fn new(id: u32, nom: String, prix: f64) -> Self {
        Produit { id, nom, prix }
    }
}
```

**src/utils/mod.rs**
```rust
pub mod validation;
```

**src/utils/validation.rs**
```rust
pub fn valider_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn valider_prix(prix: f64) -> bool {
    prix >= 0.0
}
```

**src/main.rs**
```rust
// Déclare le module models
mod models;
mod utils;

// Importe ce dont on a besoin
use models::{Utilisateur, Produit};
use utils::validation;

fn main() {
    let user = Utilisateur::new(
        1,
        String::from("Alice"),
        String::from("alice@example.com")
    );
    
    user.afficher();
    
    if validation::valider_email(&user.email) {
        println!("Email valide");
    }
    
    // Sérialisation
    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("\nJSON:\n{}", json);
}
```

### Visibilité : pub et pub(crate)

```rust
pub struct Public;              // Visible partout
pub(crate) struct InterneCrate; // Visible dans la crate
struct Prive;                   // Visible dans le module uniquement

pub struct Mixte {
    pub champ_public: i32,
    champ_prive: String,
}
```

## 4. Projet Pratique : Gestionnaire de Bibliothèque

Créons une application complète avec modules et sérialisation.

**Cargo.toml**
```toml
[package]
name = "bibliotheque"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Structure du projet :**
```
bibliotheque/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── models/
    │   ├── mod.rs
    │   └── livre.rs
    └── services/
        ├── mod.rs
        └── bibliotheque.rs
```

**src/models/livre.rs**
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Livre {
    pub id: u32,
    pub titre: String,
    pub auteur: String,
    pub annee: u32,
    #[serde(default)]
    pub emprunte: bool,
}

impl Livre {
    pub fn new(id: u32, titre: String, auteur: String, annee: u32) -> Self {
        Livre {
            id,
            titre,
            auteur,
            annee,
            emprunte: false,
        }
    }
    
    pub fn afficher(&self) {
        let statut = if self.emprunte { "Emprunté" } else { "Disponible" };
        println!("#{} - {} par {} ({}) - {}", 
                 self.id, self.titre, self.auteur, self.annee, statut);
    }
}
```

**src/models/mod.rs**
```rust
pub mod livre;
pub use livre::Livre;
```

**src/services/bibliotheque.rs**
```rust
use crate::models::Livre;
use std::fs;
use std::io;

pub struct Bibliotheque {
    livres: Vec<Livre>,
}

impl Bibliotheque {
    pub fn new() -> Self {
        Bibliotheque {
            livres: Vec::new(),
        }
    }
    
    pub fn ajouter_livre(&mut self, livre: Livre) {
        self.livres.push(livre);
        println!("Livre ajouté avec succès");
    }
    
    pub fn lister_livres(&self) {
        if self.livres.is_empty() {
            println!("Aucun livre dans la bibliothèque");
            return;
        }
        
        println!("\n=== Bibliothèque ===");
        for livre in &self.livres {
            livre.afficher();
        }
    }
    
    pub fn emprunter_livre(&mut self, id: u32) -> Result<(), String> {
        match self.livres.iter_mut().find(|l| l.id == id) {
            Some(livre) => {
                if livre.emprunte {
                    Err(String::from("Ce livre est déjà emprunté"))
                } else {
                    livre.emprunte = true;
                    Ok(())
                }
            }
            None => Err(String::from("Livre non trouvé")),
        }
    }
    
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
    
    pub fn sauvegarder(&self, fichier: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.livres)?;
        fs::write(fichier, json)?;
        println!("Bibliothèque sauvegardée dans {}", fichier);
        Ok(())
    }
    
    pub fn charger(fichier: &str) -> io::Result<Self> {
        let contenu = fs::read_to_string(fichier)?;
        let livres: Vec<Livre> = serde_json::from_str(&contenu)?;
        println!("Bibliothèque chargée depuis {}", fichier);
        Ok(Bibliotheque { livres })
    }
}
```

**src/services/mod.rs**
```rust
pub mod bibliotheque;
pub use bibliotheque::Bibliotheque;
```

**src/main.rs**
```rust
mod models;
mod services;

use models::Livre;
use services::Bibliotheque;
use std::io::{self, Write};

fn afficher_menu() {
    println!("\n=== Menu ===");
    println!("1. Ajouter un livre");
    println!("2. Lister les livres");
    println!("3. Emprunter un livre");
    println!("4. Retourner un livre");
    println!("5. Sauvegarder");
    println!("6. Charger");
    println!("7. Quitter");
    print!("Votre choix: ");
    io::stdout().flush().unwrap();
}

fn lire_ligne() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut biblio = Bibliotheque::new();
    let fichier_sauvegarde = "bibliotheque.json";
    
    loop {
        afficher_menu();
        let choix = lire_ligne();
        
        match choix.as_str() {
            "1" => {
                print!("ID: ");
                io::stdout().flush().unwrap();
                let id: u32 = lire_ligne().parse().unwrap_or(0);
                
                print!("Titre: ");
                io::stdout().flush().unwrap();
                let titre = lire_ligne();
                
                print!("Auteur: ");
                io::stdout().flush().unwrap();
                let auteur = lire_ligne();
                
                print!("Année: ");
                io::stdout().flush().unwrap();
                let annee: u32 = lire_ligne().parse().unwrap_or(0);
                
                let livre = Livre::new(id, titre, auteur, annee);
                biblio.ajouter_livre(livre);
            }
            "2" => {
                biblio.lister_livres();
            }
            "3" => {
                print!("ID du livre à emprunter: ");
                io::stdout().flush().unwrap();
                let id: u32 = lire_ligne().parse().unwrap_or(0);
                
                match biblio.emprunter_livre(id) {
                    Ok(_) => println!("Livre emprunté avec succès"),
                    Err(e) => println!("Erreur: {}", e),
                }
            }
            "4" => {
                print!("ID du livre à retourner: ");
                io::stdout().flush().unwrap();
                let id: u32 = lire_ligne().parse().unwrap_or(0);
                
                match biblio.retourner_livre(id) {
                    Ok(_) => println!("Livre retourné avec succès"),
                    Err(e) => println!("Erreur: {}", e),
                }
            }
            "5" => {
                if let Err(e) = biblio.sauvegarder(fichier_sauvegarde) {
                    println!("Erreur lors de la sauvegarde: {}", e);
                }
            }
            "6" => {
                match Bibliotheque::charger(fichier_sauvegarde) {
                    Ok(b) => {
                        biblio = b;
                    }
                    Err(e) => println!("Erreur lors du chargement: {}", e),
                }
            }
            "7" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide"),
        }
    }
}
```

## 5. Points Clés à Retenir

### Dérive (Derive)
- `#[derive(Debug)]` : permet d'afficher avec `{:?}`
- `#[derive(Clone)]` : permet de cloner la struct
- `#[derive(Serialize, Deserialize)]` : active serde

### Organisation
- Un fichier `mod.rs` dans chaque dossier déclare les sous-modules
- `pub` rend public, accessible depuis l'extérieur
- `use` importe dans le scope local
- `crate::` référence la racine de votre crate

### Serde
- Fonctionne avec JSON, TOML, YAML, etc.
- Les attributs permettent une personnalisation fine
- Gestion d'erreurs avec `Result`

## 6. Exercices

### Exercice 1 : Gestionnaire de Contacts Avancé
Créez une application avec :
- Module `models` avec `Contact` et `Groupe`
- Module `services` avec `CarnetAdresses`
- Sérialisation JSON
- Import/export de contacts

### Exercice 2 : Configuration d'Application
Créez une struct `Config` avec :
- Paramètres de connexion base de données
- Options d'application
- Chargement depuis fichier JSON/TOML
- Valeurs par défaut avec `#[serde(default)]`

### Exercice 3 : API de Produits
Structure modulaire :
- `models/produit.rs` : struct Produit
- `models/categorie.rs` : struct Catégorie
- `services/catalogue.rs` : gestion des produits
- Sauvegarde/chargement JSON

## Conclusion

Vous maîtrisez maintenant :
- ✅ Les structs et leurs implémentations
- ✅ Serde pour la sérialisation
- ✅ L'organisation modulaire du code
- ✅ La séparation des responsabilités

Ces concepts sont essentiels pour construire des applications Rust professionnelles et maintenables.

---

**Prochaine leçon** : Collections (Vec, HashMap, HashSet) et itérateurs