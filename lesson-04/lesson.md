# Leçon 4 : Ownership, Borrowing et Lifetimes

## Introduction

Bienvenue à la leçon 4 ! Nous abordons maintenant **le concept le plus important et unique de Rust** : le système d'ownership (propriété). C'est ce qui permet à Rust de garantir la sécurité mémoire sans garbage collector.

## 🎯 Objectifs de la leçon

- Comprendre le système d'ownership
- Maîtriser le borrowing (emprunt) et les références
- Découvrir les lifetimes (durées de vie)
- Apprendre à gérer la mémoire de manière sûre

---

## 1. Le système d'Ownership

### 1.1 Les trois règles fondamentales

```rust
// Règle 1 : Chaque valeur a un propriétaire unique
let s1 = String::from("bonjour");
// s1 est le propriétaire de la chaîne

// Règle 2 : Il ne peut y avoir qu'un seul propriétaire à la fois
let s2 = s1;  // s1 transfère la propriété à s2
// println!("{}", s1);  // ❌ ERREUR : s1 n'est plus valide !

// Règle 3 : Quand le propriétaire sort du scope, la valeur est détruite
{
    let s3 = String::from("temporaire");
    println!("{}", s3);
}  // s3 est détruit ici automatiquement
// println!("{}", s3);  // ❌ ERREUR : s3 n'existe plus
```

### 1.2 Types "Copy" vs types "Move"

```rust
// Types simples : implémentent Copy (copie automatique)
let x = 5;
let y = x;  // x est copié dans y
println!("x = {}, y = {}", x, y);  // ✅ Les deux sont valides

// Types complexes : Move par défaut
let s1 = String::from("hello");
let s2 = s1;  // s1 est déplacé (moved) vers s2
// println!("{}", s1);  // ❌ ERREUR

// Pour copier explicitement, utiliser clone()
let s3 = String::from("hello");
let s4 = s3.clone();  // Copie profonde
println!("s3 = {}, s4 = {}", s3, s4);  // ✅ Les deux sont valides
```

### 1.3 Ownership et fonctions

```rust
fn main() {
    let s = String::from("bonjour");
    prend_ownership(s);  // s est déplacé dans la fonction
    // println!("{}", s);  // ❌ ERREUR : s n'est plus valide
    
    let x = 5;
    fait_copie(x);  // x est copié
    println!("{}", x);  // ✅ OK : x est toujours valide
}

fn prend_ownership(chaine: String) {
    println!("{}", chaine);
}  // chaine est détruite ici

fn fait_copie(nombre: i32) {
    println!("{}", nombre);
}  // nombre est détruit, mais c'était une copie
```

---

## 2. Les références et le Borrowing

### 2.1 Références immuables (&)

```rust
fn main() {
    let s1 = String::from("bonjour");
    
    let longueur = calcule_longueur(&s1);  // Emprunt immuable
    println!("La longueur de '{}' est {}", s1, longueur);  // ✅ s1 est toujours valide
}

fn calcule_longueur(s: &String) -> usize {
    s.len()  // On peut lire, mais pas modifier
}  // s sort du scope, mais on ne détruit pas ce qu'elle référence
```

### 2.2 Références mutables (&mut)

```rust
fn main() {
    let mut s = String::from("bonjour");
    
    modifie_string(&mut s);  // Emprunt mutable
    println!("{}", s);  // Affiche "bonjour, monde"
}

fn modifie_string(s: &mut String) {
    s.push_str(", monde");
}

// ⚠️ Restriction : une seule référence mutable à la fois
fn exemple_restriction() {
    let mut s = String::from("test");
    
    let r1 = &mut s;
    // let r2 = &mut s;  // ❌ ERREUR : on ne peut pas avoir deux refs mutables
    println!("{}", r1);
}
```

### 2.3 Règles de borrowing

```rust
fn main() {
    let mut s = String::from("hello");
    
    // ✅ OK : Plusieurs références immuables
    let r1 = &s;
    let r2 = &s;
    println!("{} et {}", r1, r2);
    
    // ✅ OK : La portée de r1 et r2 est terminée
    let r3 = &mut s;  // Maintenant on peut avoir une ref mutable
    r3.push_str(" world");
    println!("{}", r3);
}

fn references_mixtes() {
    let mut s = String::from("test");
    
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;  // ❌ ERREUR : on ne peut pas mélanger
    println!("{} {}", r1, r2);
}
```

---

## 3. Les Slices

### 3.1 String slices

```rust
fn main() {
    let s = String::from("bonjour le monde");
    
    let bonjour = &s[0..7];    // ou &s[..7]
    let monde = &s[11..16];    // ou &s[11..]
    let tout = &s[..];         // Tout le string
    
    println!("{}", bonjour);   // "bonjour"
    println!("{}", monde);     // "monde"
}

// Fonction pratique avec slices
fn premier_mot(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn exemple_usage() {
    let phrase = String::from("hello world");
    let mot = premier_mot(&phrase);
    println!("Premier mot : {}", mot);  // "hello"
}
```

### 3.2 Array slices

```rust
fn main() {
    let tableau = [1, 2, 3, 4, 5];
    
    let slice = &tableau[1..4];  // [2, 3, 4]
    
    for element in slice {
        println!("{}", element);
    }
}
```

---

## 4. Lifetimes (Durées de vie)

### 4.1 Pourquoi les lifetimes ?

```rust
// Le compilateur doit savoir quelle référence sera retournée
fn plus_long<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let string2 = String::from("xyz");
    
    let resultat = plus_long(&string1, &string2);
    println!("La plus longue : {}", resultat);
}
```

### 4.2 Syntaxe des lifetimes

```rust
// Lifetime explicite
fn retourne_premiere<'a>(x: &'a str, y: &str) -> &'a str {
    x  // On retourne toujours x, donc seul x a besoin du lifetime
}

// Plusieurs lifetimes
fn compare<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("Comparaison de {} et {}", x, y);
    x
}

// Lifetime dans une structure
struct ImportantExcerpt<'a> {
    partie: &'a str,
}

fn main() {
    let roman = String::from("Appelez-moi Ismaël. Il y a des années...");
    let premiere_phrase = roman.split('.').next().unwrap();
    
    let extrait = ImportantExcerpt {
        partie: premiere_phrase,
    };
    
    println!("Extrait : {}", extrait.partie);
}
```

### 4.3 Lifetime elision (règles implicites)

```rust
// Le compilateur infère automatiquement les lifetimes dans ces cas :

// Règle 1 : Chaque paramètre référence obtient son propre lifetime
fn foo(x: &str) -> &str {  // Équivalent à foo<'a>(x: &'a str) -> &'a str
    x
}

// Règle 2 : S'il y a exactement un paramètre référence, 
// son lifetime est assigné à toutes les références de sortie
fn premier_mot(s: &str) -> &str {
    // Lifetime automatique
    s
}

// Règle 3 : Si plusieurs paramètres et l'un est &self ou &mut self,
// le lifetime de self est assigné aux références de sortie
impl<'a> ImportantExcerpt<'a> {
    fn niveau(&self) -> i32 {
        3
    }
    
    fn annonce_et_retourne(&self, annonce: &str) -> &str {
        println!("Attention : {}", annonce);
        self.partie
    }
}
```

---

## 5. Patterns courants

### 5.1 Retourner ownership

```rust
fn cree_et_retourne() -> String {
    let s = String::from("créé");
    s  // s est retourné et son ownership est transféré
}

fn prend_et_retourne(s: String) -> String {
    println!("Traitement de : {}", s);
    s  // On retourne l'ownership
}

fn main() {
    let s1 = cree_et_retourne();
    let s2 = prend_et_retourne(s1);
    println!("{}", s2);
}
```

### 5.2 Utiliser des tuples

```rust
fn calcule_longueur_et_retourne(s: String) -> (String, usize) {
    let longueur = s.len();
    (s, longueur)  // On retourne à la fois le string et sa longueur
}

fn main() {
    let s1 = String::from("bonjour");
    let (s2, len) = calcule_longueur_et_retourne(s1);
    println!("'{}' a une longueur de {}", s2, len);
}
```

---

## 💻 Exercices pratiques

### Exercice 1 : Gestionnaire de texte

Créez un programme qui manipule du texte sans copier inutilement :

```rust
fn compte_mots(texte: &str) -> usize {
    // Compter les mots dans le texte
    todo!()
}

fn extrait_ligne(texte: &str, numero: usize) -> Option<&str> {
    // Extraire une ligne spécifique
    todo!()
}

fn remplace_mot(texte: &mut String, ancien: &str, nouveau: &str) {
    // Remplacer toutes les occurrences d'un mot
    todo!()
}

fn main() {
    let mut texte = String::from("Rust est génial\nLe borrowing est puissant");
    
    println!("Nombre de mots : {}", compte_mots(&texte));
    
    if let Some(ligne) = extrait_ligne(&texte, 1) {
        println!("Ligne 1 : {}", ligne);
    }
    
    remplace_mot(&mut texte, "génial", "extraordinaire");
    println!("Texte modifié : {}", texte);
}
```

### Exercice 2 : Analyseur de liste

```rust
fn trouve_plus_grand<'a>(liste: &'a [i32]) -> Option<&'a i32> {
    // Retourner une référence au plus grand élément
    todo!()
}

fn filtre_pairs(liste: &[i32]) -> Vec<i32> {
    // Créer un nouveau vecteur avec uniquement les nombres pairs
    todo!()
}

fn main() {
    let nombres = vec![23, 45, 12, 67, 89, 34];
    
    if let Some(max) = trouve_plus_grand(&nombres) {
        println!("Plus grand : {}", max);
    }
    
    let pairs = filtre_pairs(&nombres);
    println!("Nombres pairs : {:?}", pairs);
}
```

---

## 🎯 Projet : Gestionnaire de contacts

Créez un système simple de gestion de contacts :

```rust
struct Contact<'a> {
    nom: &'a str,
    telephone: &'a str,
    email: &'a str,
}

struct CarnetAdresses<'a> {
    contacts: Vec<Contact<'a>>,
}

impl<'a> CarnetAdresses<'a> {
    fn nouveau() -> Self {
        CarnetAdresses {
            contacts: Vec::new(),
        }
    }
    
    fn ajouter_contact(&mut self, contact: Contact<'a>) {
        self.contacts.push(contact);
    }
    
    fn chercher_par_nom(&self, nom: &str) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.nom == nom)
    }
    
    fn afficher_tous(&self) {
        for contact in &self.contacts {
            println!("{}: {} ({})", contact.nom, contact.telephone, contact.email);
        }
    }
}

fn main() {
    let nom1 = String::from("Alice Dupont");
    let tel1 = String::from("0123456789");
    let email1 = String::from("alice@example.com");
    
    let mut carnet = CarnetAdresses::nouveau();
    
    carnet.ajouter_contact(Contact {
        nom: &nom1,
        telephone: &tel1,
        email: &email1,
    });
    
    carnet.afficher_tous();
    
    if let Some(contact) = carnet.chercher_par_nom("Alice Dupont") {
        println!("Trouvé : {}", contact.telephone);
    }
}
```

---

## 📚 Résumé

### Concepts clés

- **Ownership** : Chaque valeur a un propriétaire unique
- **Move** : Transfert de propriété (types complexes)
- **Copy** : Copie automatique (types simples)
- **Borrowing** : Emprunter une référence sans prendre ownership
- **Références immuables** (`&`) : Lecture seule, plusieurs simultanées possibles
- **Références mutables** (`&mut`) : Une seule à la fois
- **Slices** : Références à une portion de données
- **Lifetimes** : Garantir la validité des références

### Règles d'or

1. Une valeur ne peut avoir qu'un seul propriétaire
2. Soit plusieurs références immuables, soit une seule mutable
3. Les références doivent toujours être valides
4. Les lifetimes garantissent qu'on ne référence pas de données détruites

---

## 🚀 Prochaine étape

La leçon 5 couvrira les **Structs et Enums** pour organiser vos données de manière élégante et sûre !

---

**Bon courage et n'hésitez pas si tu as des questions ! 🦀**
