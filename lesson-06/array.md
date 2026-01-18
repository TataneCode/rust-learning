# Collections en Rust : Vec, HashMap et plus

## Introduction

Pour l'exercice `MemoryStorage`, tu auras besoin de stocker des données en mémoire. Rust propose plusieurs collections dans `std::collections`.

---

## 1. Vec<T> - Tableau Dynamique

### Création

```rust
// Vecteur vide
let mut vec: Vec<String> = Vec::new();

// Avec la macro vec!
let nombres = vec![1, 2, 3, 4, 5];

// Avec capacité pré-allouée (optimisation)
let mut vec: Vec<String> = Vec::with_capacity(100);
```

### Opérations Courantes

```rust
let mut fruits = vec!["pomme", "banane"];

// Ajouter
fruits.push("orange");

// Accéder
let premier = &fruits[0];           // Panique si hors bornes
let premier = fruits.get(0);        // Retourne Option<&T>

// Modifier
fruits[1] = "kiwi";

// Supprimer
let dernier = fruits.pop();         // Retourne Option<T>
fruits.remove(0);                   // Supprime à l'index

// Taille
println!("Taille: {}", fruits.len());
println!("Est vide: {}", fruits.is_empty());
```

### Itération

```rust
let nombres = vec![1, 2, 3];

// Immutable
for n in &nombres {
    println!("{}", n);
}

// Mutable
for n in &mut nombres {
    *n *= 2;
}

// Avec index
for (i, n) in nombres.iter().enumerate() {
    println!("Index {}: {}", i, n);
}
```

---

## 2. HashMap<K, V> - Table de Hachage

C'est la structure idéale pour `MemoryStorage` car elle permet un accès O(1) par clé.

### Import et Création

```rust
use std::collections::HashMap;

// HashMap vide
let mut map: HashMap<String, String> = HashMap::new();

// Avec capacité
let mut map: HashMap<String, String> = HashMap::with_capacity(100);
```

### Opérations CRUD

```rust
use std::collections::HashMap;

let mut storage: HashMap<String, String> = HashMap::new();

// CREATE - Insérer
storage.insert(String::from("user:1"), String::from("Alice"));
storage.insert("user:2".to_string(), "Bob".to_string());

// READ - Lire
// get() retourne Option<&V>
match storage.get("user:1") {
    Some(value) => println!("Trouvé: {}", value),
    None => println!("Non trouvé"),
}

// Avec if let
if let Some(value) = storage.get("user:1") {
    println!("Valeur: {}", value);
}

// UPDATE - Mettre à jour (insert écrase)
storage.insert("user:1".to_string(), "Alice Updated".to_string());

// UPDATE - Seulement si existe
if let Some(value) = storage.get_mut("user:1") {
    *value = String::from("Alice Modified");
}

// DELETE - Supprimer
let removed = storage.remove("user:1");  // Retourne Option<V>
match removed {
    Some(v) => println!("Supprimé: {}", v),
    None => println!("Clé inexistante"),
}
```

### Pattern Entry (Insertion Conditionnelle)

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();

// Insérer seulement si la clé n'existe pas
scores.entry("Blue".to_string()).or_insert(50);

// Insérer avec une valeur par défaut calculée
scores.entry("Red".to_string()).or_insert_with(|| {
    // Calcul coûteux ici
    100
});

// Modifier la valeur existante
let count = scores.entry("Blue".to_string()).or_insert(0);
*count += 1;
```

### Vérifier l'Existence

```rust
if storage.contains_key("user:1") {
    println!("La clé existe");
}
```

### Itération

```rust
// Itérer sur les paires (clé, valeur)
for (key, value) in &storage {
    println!("{}: {}", key, value);
}

// Seulement les clés
for key in storage.keys() {
    println!("Clé: {}", key);
}

// Seulement les valeurs
for value in storage.values() {
    println!("Valeur: {}", value);
}
```

---

## 3. Exemple : MemoryStorage avec HashMap

Voici comment tu pourrais implémenter `MemoryStorage` pour l'exercice 1 :

```rust
use std::collections::HashMap;

pub struct MemoryStorage {
    data: HashMap<String, String>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            data: HashMap::new(),
        }
    }
}

impl Storage for MemoryStorage {
    fn save(&mut self, key: &str, value: &str) -> Result<(), String> {
        self.data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn load(&self, key: &str) -> Result<String, String> {
        self.data
            .get(key)
            .cloned()  // Clone la String pour la retourner
            .ok_or_else(|| format!("Clé '{}' non trouvée", key))
    }

    fn delete(&mut self, key: &str) -> Result<(), String> {
        self.data
            .remove(key)
            .map(|_| ())  // Transforme Option<String> en Option<()>
            .ok_or_else(|| format!("Clé '{}' non trouvée", key))
    }
}
```

---

## 4. Autres Collections Utiles

### HashSet<T> - Ensemble sans Doublons

```rust
use std::collections::HashSet;

let mut set: HashSet<String> = HashSet::new();
set.insert("unique".to_string());
set.insert("unique".to_string());  // Ignoré, déjà présent

println!("Contient: {}", set.contains("unique"));
```

### VecDeque<T> - File Double-Ended

```rust
use std::collections::VecDeque;

let mut queue: VecDeque<i32> = VecDeque::new();
queue.push_back(1);   // Ajouter à la fin
queue.push_front(0);  // Ajouter au début
queue.pop_front();    // Retirer du début
queue.pop_back();     // Retirer de la fin
```

### BTreeMap<K, V> - Map Ordonnée

```rust
use std::collections::BTreeMap;

let mut map: BTreeMap<String, i32> = BTreeMap::new();
// Les clés sont automatiquement triées
```

---

## 5. Gestion de la Mutabilité

### Attention au Trait

Pour `MemoryStorage`, le trait `Storage` définit :

```rust
fn save(&self, key: &str, value: &str) -> Result<(), String>;
```

Mais `HashMap::insert` nécessite `&mut self`. Solutions :

#### Option 1 : Modifier le trait (Recommandé)

```rust
trait Storage {
    fn save(&mut self, key: &str, value: &str) -> Result<(), String>;
    fn load(&self, key: &str) -> Result<String, String>;
    fn delete(&mut self, key: &str) -> Result<(), String>;
}
```

#### Option 2 : Interior Mutability avec RefCell

```rust
use std::cell::RefCell;
use std::collections::HashMap;

pub struct MemoryStorage {
    data: RefCell<HashMap<String, String>>,
}

impl Storage for MemoryStorage {
    fn save(&self, key: &str, value: &str) -> Result<(), String> {
        self.data.borrow_mut().insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn load(&self, key: &str) -> Result<String, String> {
        self.data
            .borrow()
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Clé '{}' non trouvée", key))
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        self.data
            .borrow_mut()
            .remove(key)
            .map(|_| ())
            .ok_or_else(|| format!("Clé '{}' non trouvée", key))
    }
}
```

---

## 6. Méthodes Utiles sur Option

Quand tu travailles avec `HashMap::get()`, tu reçois un `Option`. Voici comment le manipuler :

```rust
let opt: Option<&String> = map.get("key");

// Transformer en Result
let result: Result<&String, String> = opt.ok_or("Erreur".to_string());
let result: Result<&String, String> = opt.ok_or_else(|| "Erreur".to_string());

// Cloner le contenu
let owned: Option<String> = opt.cloned();

// Valeur par défaut
let value: &String = opt.unwrap_or(&default);
let value: String = opt.cloned().unwrap_or_default();
```

---

## Résumé

| Collection | Usage | Accès | Ordre |
|------------|-------|-------|-------|
| `Vec<T>` | Liste ordonnée | O(1) par index | Insertion |
| `HashMap<K,V>` | Clé-valeur | O(1) par clé | Aucun |
| `HashSet<T>` | Ensemble unique | O(1) | Aucun |
| `BTreeMap<K,V>` | Clé-valeur trié | O(log n) | Clés triées |
| `VecDeque<T>` | File/Pile | O(1) aux extrémités | Insertion |

Pour `MemoryStorage`, utilise **HashMap<String, String>** - c'est le choix naturel pour un système clé-valeur.
