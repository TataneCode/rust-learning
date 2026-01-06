# Leçon 3 : Structures de contrôle et Fonctions

## Introduction

Bienvenue dans la leçon 3 ! Aujourd'hui, nous allons apprendre à contrôler le flux d'exécution de nos programmes avec les conditions et les boucles, puis nous découvrirons comment organiser notre code avec les fonctions.

## 1. Les structures conditionnelles

### 1.1 if/else

La structure `if` permet d'exécuter du code selon une condition.

```rust
fn main() {
    let age = 18;
    
    if age >= 18 {
        println!("Vous êtes majeur");
    } else {
        println!("Vous êtes mineur");
    }
}
```

**Important** : En Rust, la condition doit être un booléen. On ne peut pas écrire `if age {` comme dans certains autres langages.

### 1.2 else if

Pour tester plusieurs conditions :

```rust
fn main() {
    let note = 15;
    
    if note >= 16 {
        println!("Très bien !");
    } else if note >= 14 {
        println!("Bien");
    } else if note >= 12 {
        println!("Assez bien");
    } else if note >= 10 {
        println!("Passable");
    } else {
        println!("Insuffisant");
    }
}
```

### 1.3 if comme expression

En Rust, `if` est une expression et peut retourner une valeur :

```rust
fn main() {
    let condition = true;
    let nombre = if condition { 5 } else { 10 };
    
    println!("La valeur est : {}", nombre);
    
    // Utile pour initialiser des variables
    let age = 20;
    let statut = if age >= 18 { "majeur" } else { "mineur" };
    println!("Statut : {}", statut);
}
```

**Attention** : Les deux branches doivent retourner le même type !

## 2. Le match (correspondance de motifs)

`match` est une structure très puissante en Rust, plus expressive que le `switch` d'autres langages.

### 2.1 Match basique

```rust
fn main() {
    let nombre = 3;
    
    match nombre {
        1 => println!("Un"),
        2 => println!("Deux"),
        3 => println!("Trois"),
        4 => println!("Quatre"),
        5 => println!("Cinq"),
        _ => println!("Autre nombre"),
    }
}
```

Le `_` est le motif par défaut (comme `default` dans un `switch`).

### 2.2 Match avec plages

```rust
fn main() {
    let nombre = 15;
    
    match nombre {
        1..=10 => println!("Entre 1 et 10"),
        11..=20 => println!("Entre 11 et 20"),
        21..=30 => println!("Entre 21 et 30"),
        _ => println!("Au-delà de 30"),
    }
}
```

### 2.3 Match comme expression

```rust
fn main() {
    let nombre = 2;
    
    let description = match nombre {
        1 => "premier",
        2 => "deuxième",
        3 => "troisième",
        _ => "autre",
    };
    
    println!("C'est le {} nombre", description);
}
```

### 2.4 Match avec plusieurs motifs

```rust
fn main() {
    let jour = 6;
    
    match jour {
        1 | 2 | 3 | 4 | 5 => println!("Jour de semaine"),
        6 | 7 => println!("Weekend !"),
        _ => println!("Jour invalide"),
    }
}
```

## 3. Les boucles

### 3.1 loop (boucle infinie)

```rust
fn main() {
    let mut compteur = 0;
    
    loop {
        compteur += 1;
        println!("Compteur : {}", compteur);
        
        if compteur == 5 {
            break; // Sort de la boucle
        }
    }
    
    println!("Boucle terminée !");
}
```

### 3.2 loop avec valeur de retour

```rust
fn main() {
    let mut compteur = 0;
    
    let resultat = loop {
        compteur += 1;
        
        if compteur == 10 {
            break compteur * 2; // Retourne une valeur
        }
    };
    
    println!("Le résultat est : {}", resultat);
}
```

### 3.3 while

```rust
fn main() {
    let mut nombre = 3;
    
    while nombre != 0 {
        println!("{}!", nombre);
        nombre -= 1;
    }
    
    println!("Décollage !");
}
```

### 3.4 for

La boucle `for` est la plus utilisée en Rust.

```rust
fn main() {
    // Parcourir une plage
    for nombre in 1..6 {
        println!("Nombre : {}", nombre);
    }
    
    println!("---");
    
    // Avec plage inclusive
    for nombre in 1..=5 {
        println!("Nombre : {}", nombre);
    }
    
    println!("---");
    
    // Parcourir un tableau
    let fruits = ["pomme", "banane", "orange"];
    
    for fruit in fruits.iter() {
        println!("J'aime les {}s", fruit);
    }
}
```

### 3.5 for avec reverse

```rust
fn main() {
    for nombre in (1..=5).rev() {
        println!("{}!", nombre);
    }
    println!("Décollage !");
}
```

## 4. Les fonctions

### 4.1 Déclaration de base

```rust
fn saluer() {
    println!("Bonjour !");
}

fn main() {
    saluer();
    saluer();
}
```

### 4.2 Fonctions avec paramètres

```rust
fn saluer_personne(nom: &str) {
    println!("Bonjour, {} !", nom);
}

fn main() {
    saluer_personne("Alice");
    saluer_personne("Bob");
}
```

**Important** : En Rust, il faut toujours spécifier le type des paramètres.

### 4.3 Fonctions avec plusieurs paramètres

```rust
fn additionner(a: i32, b: i32) {
    let somme = a + b;
    println!("{} + {} = {}", a, b, somme);
}

fn main() {
    additionner(5, 3);
    additionner(10, 20);
}
```

### 4.4 Fonctions avec valeur de retour

```rust
fn additionner(a: i32, b: i32) -> i32 {
    a + b  // Pas de point-virgule = retour implicite
}

fn main() {
    let resultat = additionner(5, 3);
    println!("5 + 3 = {}", resultat);
    
    // Utilisation directe
    println!("10 + 20 = {}", additionner(10, 20));
}
```

**Point clé** : En Rust, la dernière expression d'une fonction (sans `;`) est automatiquement retournée.

### 4.5 Return explicite

```rust
fn est_pair(nombre: i32) -> bool {
    if nombre % 2 == 0 {
        return true;  // Return explicite
    }
    false  // Return implicite
}

fn main() {
    println!("4 est pair : {}", est_pair(4));
    println!("7 est pair : {}", est_pair(7));
}
```

### 4.6 Fonctions multiples

```rust
fn aire_rectangle(longueur: f64, largeur: f64) -> f64 {
    longueur * largeur
}

fn perimetre_rectangle(longueur: f64, largeur: f64) -> f64 {
    2.0 * (longueur + largeur)
}

fn afficher_info_rectangle(longueur: f64, largeur: f64) {
    println!("Rectangle {}x{}", longueur, largeur);
    println!("Aire : {}", aire_rectangle(longueur, largeur));
    println!("Périmètre : {}", perimetre_rectangle(longueur, largeur));
}

fn main() {
    afficher_info_rectangle(5.0, 3.0);
}
```

## 5. Projet pratique : Calculatrice avec menu

Créons une calculatrice interactive avec un menu.

```rust
use std::io;

fn addition(a: f64, b: f64) -> f64 {
    a + b
}

fn soustraction(a: f64, b: f64) -> f64 {
    a - b
}

fn multiplication(a: f64, b: f64) -> f64 {
    a * b
}

fn division(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        println!("Erreur : division par zéro !");
        0.0
    }
}

fn lire_nombre(message: &str) -> f64 {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().parse().expect("Veuillez entrer un nombre valide")
}

fn afficher_menu() {
    println!("\n=== CALCULATRICE ===");
    println!("1. Addition");
    println!("2. Soustraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Quitter");
    println!("====================");
}

fn main() {
    loop {
        afficher_menu();
        
        let choix = lire_nombre("Votre choix :") as i32;
        
        if choix == 5 {
            println!("Au revoir !");
            break;
        }
        
        if choix < 1 || choix > 5 {
            println!("Choix invalide !");
            continue;
        }
        
        let a = lire_nombre("Premier nombre :");
        let b = lire_nombre("Deuxième nombre :");
        
        let resultat = match choix {
            1 => addition(a, b),
            2 => soustraction(a, b),
            3 => multiplication(a, b),
            4 => division(a, b),
            _ => 0.0,
        };
        
        let operation = match choix {
            1 => "+",
            2 => "-",
            3 => "×",
            4 => "÷",
            _ => "?",
        };
        
        println!("\nRésultat : {} {} {} = {}", a, operation, b, resultat);
    }
}
```

## 6. Projet : Jeu du FizzBuzz

Un classique de la programmation !

```rust
fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}

fn main() {
    println!("=== JEU FIZZBUZZ ===");
    
    for i in 1..=100 {
        println!("{}: {}", i, fizzbuzz(i));
    }
}
```

## 7. Projet : Validateur de mot de passe

```rust
fn contient_majuscule(mot_de_passe: &str) -> bool {
    mot_de_passe.chars().any(|c| c.is_uppercase())
}

fn contient_minuscule(mot_de_passe: &str) -> bool {
    mot_de_passe.chars().any(|c| c.is_lowercase())
}

fn contient_chiffre(mot_de_passe: &str) -> bool {
    mot_de_passe.chars().any(|c| c.is_numeric())
}

fn est_assez_long(mot_de_passe: &str) -> bool {
    mot_de_passe.len() >= 8
}

fn valider_mot_de_passe(mot_de_passe: &str) -> bool {
    let mut criteres_valides = 0;
    let mut messages = Vec::new();
    
    if est_assez_long(mot_de_passe) {
        criteres_valides += 1;
    } else {
        messages.push("❌ Au moins 8 caractères");
    }
    
    if contient_majuscule(mot_de_passe) {
        criteres_valides += 1;
    } else {
        messages.push("❌ Au moins une majuscule");
    }
    
    if contient_minuscule(mot_de_passe) {
        criteres_valides += 1;
    } else {
        messages.push("❌ Au moins une minuscule");
    }
    
    if contient_chiffre(mot_de_passe) {
        criteres_valides += 1;
    } else {
        messages.push("❌ Au moins un chiffre");
    }
    
    println!("\nAnalyse du mot de passe :");
    println!("Critères validés : {}/4", criteres_valides);
    
    for message in messages {
        println!("{}", message);
    }
    
    criteres_valides == 4
}

fn main() {
    use std::io;
    
    println!("=== VALIDATEUR DE MOT DE PASSE ===");
    println!("Critères :");
    println!("- Au moins 8 caractères");
    println!("- Au moins une majuscule");
    println!("- Au moins une minuscule");
    println!("- Au moins un chiffre");
    
    loop {
        println!("\nEntrez un mot de passe (ou 'quit' pour quitter) :");
        let mut mot_de_passe = String::new();
        io::stdin().read_line(&mut mot_de_passe).expect("Erreur de lecture");
        let mot_de_passe = mot_de_passe.trim();
        
        if mot_de_passe == "quit" {
            break;
        }
        
        if valider_mot_de_passe(mot_de_passe) {
            println!("✅ Mot de passe VALIDE !");
        } else {
            println!("❌ Mot de passe INVALIDE");
        }
    }
}
```

## 8. Différence entre fonction et méthode en Rust

### 8.1 Fonctions

Les fonctions sont des blocs de code indépendants, déclarés avec `fn` :

```rust
fn calculer_carre(x: i32) -> i32 {
    x * x
}

fn main() {
    let resultat = calculer_carre(5);
    println!("Carré de 5 : {}", resultat);
}
```

### 8.2 Méthodes

Les méthodes sont des fonctions associées à un type (structure, enum, etc.). On les appelle avec la syntaxe `.methode()` :

```rust
fn main() {
    let texte = String::from("Bonjour");
    
    // Méthodes sur String
    println!("Longueur : {}", texte.len());
    println!("Majuscules : {}", texte.to_uppercase());
    println!("Contient 'jour' : {}", texte.contains("jour"));
    
    // Méthodes sur nombres
    let nombre = -42;
    println!("Valeur absolue : {}", nombre.abs());
    
    let flottant = 3.7;
    println!("Arrondi supérieur : {}", flottant.ceil());
    println!("Arrondi inférieur : {}", flottant.floor());
}
```

**Pour l'instant** : Nous utilisons des fonctions que nous créons, et des méthodes fournies par Rust sur les types de base. Dans les prochaines leçons, nous apprendrons à créer nos propres types avec leurs méthodes !

## Exercices

### Exercice 1 : Table de multiplication
Créez une fonction `table_multiplication(n: i32)` qui affiche la table de multiplication de `n` de 1 à 10.

### Exercice 2 : Nombre premier
Créez une fonction `est_premier(n: i32) -> bool` qui retourne `true` si le nombre est premier, `false` sinon. Testez avec les nombres de 1 à 20.

### Exercice 3 : Convertisseur de température avancé
Créez un programme avec un menu permettant de :
1. Convertir Celsius → Fahrenheit
2. Convertir Fahrenheit → Celsius
3. Convertir Celsius → Kelvin
4. Convertir Kelvin → Celsius

Utilisez des fonctions séparées pour chaque conversion.

### Exercice 4 : Calcul de moyenne
Créez une fonction qui calcule la moyenne de 5 notes entrées par l'utilisateur, puis affiche :
- La moyenne
- La mention (Très bien ≥16, Bien ≥14, Assez bien ≥12, Passable ≥10, Insuffisant <10)

### Exercice 5 : Jeu de devinette amélioré
Améliorez le jeu de devinette de la leçon 2 en :
- Utilisant des fonctions pour organiser le code
- Limitant le nombre de tentatives à 7
- Affichant un message différent selon le nombre de tentatives utilisées

## Résumé

Dans cette leçon, vous avez appris :

✅ Les structures conditionnelles (`if/else`)
✅ Le `match` pour la correspondance de motifs
✅ Les boucles (`loop`, `while`, `for`)
✅ Comment créer et utiliser des fonctions
✅ Les paramètres et valeurs de retour
✅ La différence entre fonctions et méthodes
✅ Comment organiser du code avec des fonctions

**Points clés** :
- `if` est une expression en Rust
- `match` doit couvrir tous les cas possibles
- Les fonctions doivent déclarer les types de leurs paramètres et retours
- La dernière expression (sans `;`) est retournée automatiquement
- Les méthodes s'appellent avec `.methode()` sur des valeurs

**Prochain sujet** : Dans la leçon 4, nous découvrirons la propriété (ownership), un concept fondamental de Rust !

---

**Note** : N'hésitez pas à expérimenter avec ces concepts. Essayez de combiner boucles, conditions et fonctions pour créer vos propres programmes !
