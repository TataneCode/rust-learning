# Leçon n°2 : Les Variables et les Types de Données

Bienvenue dans cette deuxième leçon ! Maintenant que vous avez Rust installé, nous allons explorer les fondations du langage : les variables et les types de données.

## 1. Créer un nouveau projet

Commençons par créer un projet pour cette leçon :

```bash
cargo new variables_demo
cd variables_demo
```

## 2. Les variables en Rust

### Immutabilité par défaut

En Rust, les variables sont **immutables par défaut**. Cela signifie qu'une fois qu'une valeur est assignée, elle ne peut plus être modifiée.

Ouvrez `src/main.rs` et essayez ce code :

```rust
fn main() {
    let x = 5;
    println!("La valeur de x est : {}", x);
    
    // Cette ligne causerait une erreur :
    // x = 6;
}
```

Exécutez avec `cargo run`. Ça fonctionne ! Maintenant décommentez la ligne `x = 6;` et réessayez. Vous obtiendrez une erreur car `x` est immutable.

### Variables mutables

Pour rendre une variable modifiable, utilisez le mot-clé `mut` :

```rust
fn main() {
    let mut y = 5;
    println!("La valeur de y est : {}", y);
    
    y = 6;
    println!("Maintenant y vaut : {}", y);
}
```

Cette fois, ça compile sans problème !

## 3. Les types de données primitifs

### Types entiers

Rust propose plusieurs types d'entiers, signés et non signés :

```rust
fn main() {
    let petit: i8 = -100;        // entier signé sur 8 bits (-128 à 127)
    let moyen: i32 = -50000;     // entier signé sur 32 bits (par défaut)
    let grand: i64 = -9000000;   // entier signé sur 64 bits
    
    let positif: u8 = 255;       // entier non signé sur 8 bits (0 à 255)
    let age: u32 = 25;           // entier non signé sur 32 bits
    
    println!("Petit: {}, Moyen: {}, Grand: {}", petit, moyen, grand);
    println!("Positif: {}, Age: {}", positif, age);
}
```

**Pourquoi différents types ?** Utiliser le bon type économise de la mémoire et évite des bugs. Un âge ne sera jamais négatif, donc `u32` est un meilleur choix que `i32`.

### Types flottants

Pour les nombres décimaux :

```rust
fn main() {
    let pi: f32 = 3.14159;       // flottant 32 bits
    let e: f64 = 2.71828;         // flottant 64 bits (par défaut)
    
    println!("Pi: {}, e: {}", pi, e);
}
```

### Booléens

```rust
fn main() {
    let vrai: bool = true;
    let faux: bool = false;
    
    println!("Vrai: {}, Faux: {}", vrai, faux);
}
```

### Caractères

```rust
fn main() {
    let lettre: char = 'A';
    let emoji: char = '😊';
    
    println!("Lettre: {}, Emoji: {}", lettre, emoji);
}
```

**Important :** Les `char` en Rust sont sur 4 octets et représentent un caractère Unicode, pas juste ASCII.

## 4. Le shadowing (masquage)

Le shadowing permet de réutiliser un nom de variable en déclarant une nouvelle variable avec le même nom :

```rust
fn main() {
    let x = 5;
    println!("x vaut : {}", x);
    
    let x = x + 1;  // Nouvelle variable qui masque l'ancienne
    println!("x vaut maintenant : {}", x);
    
    let x = x * 2;  // Encore une nouvelle variable
    println!("x vaut finalement : {}", x);
}
```

Le shadowing diffère de `mut` car il crée une nouvelle variable, permettant même de changer le type :

```rust
fn main() {
    let espaces = "   ";           // Type: &str
    let espaces = espaces.len();   // Type: usize (nombre)
    
    println!("Nombre d'espaces : {}", espaces);
}
```

## 5. Projet pratique : Calculateur de TVA

Créons un programme qui calcule un prix TTC avec la TVA :

```rust
fn main() {
    let prix_ht: f64 = 100.0;
    let taux_tva: f64 = 20.0;
    
    let montant_tva = prix_ht * (taux_tva / 100.0);
    let prix_ttc = prix_ht + montant_tva;
    
    println!("Prix HT : {:.2}€", prix_ht);
    println!("TVA ({:.0}%) : {:.2}€", taux_tva, montant_tva);
    println!("Prix TTC : {:.2}€", prix_ttc);
}
```

Le `{:.2}` dans `println!` affiche 2 décimales.

Testez avec `cargo run` !

## 6. Exercices

1. **Convertisseur de température** : Créez un programme qui convertit 25°C en Fahrenheit (formule : F = C × 9/5 + 32)

2. **Calcul d'IMC** : Calculez l'Indice de Masse Corporelle (poids en kg / taille en m²) pour un poids de 70kg et une taille de 1.75m

3. **Shadowing** : Créez une variable `age` qui est d'abord une chaîne "25", puis convertissez-la en nombre

## Résumé

Vous avez appris :
- Les variables sont immutables par défaut (`let`)
- Le mot-clé `mut` rend une variable modifiable
- Les différents types : `i32`, `u32`, `f64`, `bool`, `char`
- Le shadowing permet de réutiliser un nom de variable
- Comment formater l'affichage avec `println!`

Dans la prochaine leçon, nous verrons les structures de contrôle (if, else, loop) pour rendre vos programmes plus dynamiques !
