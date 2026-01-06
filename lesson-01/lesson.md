# Leçon 1 : Installation et Setup de Rust sur Ubuntu/Debian

## Objectifs
- Installer Rust sur un système Ubuntu/Debian
- Comprendre les outils de base (rustc, cargo)
- Créer et exécuter votre première application console

---

## 1. Installation de Rust

### Méthode recommandée : rustup (gestionnaire officiel)

Rustup est l'outil officiel pour installer et gérer Rust. C'est la méthode recommandée.

```bash
# Installation de rustup
## Leçon 1 : Installation et Setup de Rust sur Ubuntu/Debian

## Objectifs
- Installer Rust sur un système Ubuntu/Debian
- Comprendre les outils de base (rustc, cargo)
- Créer et exécuter votre première application console

---

## 1. Installation de Rust

### Méthode recommandée : rustup (gestionnaire officiel)

Rustup est l'outil officiel pour installer et gérer Rust. C'est la méthode recommandée.

```bash
# Installation de rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Pendant l'installation, choisissez l'option par défaut (1) en appuyant sur Entrée.

**Configuration de l'environnement :**

```bash
# Recharger la configuration du shell
source $HOME/.cargo/env

# Vérifier l'installation
rustc --version
cargo --version
```

Vous devriez voir quelque chose comme :
```
rustc 1.xx.x (hash date)
cargo 1.xx.x (hash date)
```

### Alternative : Installation via APT (non recommandée)

```bash
sudo apt update
sudo apt install rustc cargo
```

⚠️ **Attention** : Cette méthode installe souvent une version obsolète de Rust.

### Alternative : Installation via Flatpak

Flatpak n'est pas recommandé pour installer Rust lui-même, car :
- Rust est un outil de développement, pas une application
- Les toolchains ont besoin d'accès direct au système
- Cargo a besoin de gérer des dépendances système

**Cependant**, vous pouvez utiliser Flatpak pour installer un IDE comme GNOME Builder qui inclut le support Rust.

---

## 2. Outils installés

Après l'installation, vous disposez de :

- **rustc** : Le compilateur Rust
- **cargo** : Le gestionnaire de paquets et système de build
- **rustup** : Le gestionnaire de toolchains Rust

### Commandes utiles de rustup

```bash
# Mettre à jour Rust
rustup update

# Afficher la version actuelle
rustup show

# Installer la documentation locale
rustup doc

# Changer de version
rustup default stable
```

---

## 3. Votre première application : "Bonjour Rust"

### Méthode 1 : Compilation directe avec rustc

```bash
# Créer un fichier source
echo 'fn main() {
    println!("Bonjour Rust !");
}' > bonjour.rs

# Compiler
rustc bonjour.rs

# Exécuter
./bonjour
```

### Méthode 2 : Projet avec Cargo (recommandée)

```bash
# Créer un nouveau projet
cargo new bonjour_rust
cd bonjour_rust

# Structure créée :
# bonjour_rust/
# ├── Cargo.toml
# └── src/
#     └── main.rs
```

Le fichier `src/main.rs` contient déjà :

```rust
fn main() {
    println!("Hello, world!");
}
```

**Modifier le code :**

```bash
# Éditer avec votre éditeur préféré
nano src/main.rs
```

```rust
fn main() {
    println!("Bonjour Rust !");
    println!("Bienvenue dans le monde de la programmation système.");
}
```

**Compiler et exécuter :**

```bash
# Compiler et exécuter en une commande
cargo run

# Ou compiler seulement
cargo build

# Puis exécuter le binaire
./target/debug/bonjour_rust
```

---

## 4. Application console simple : Calculatrice

Créons une application plus interactive.

```bash
cargo new calculatrice
cd calculatrice
```

Éditez `src/main.rs` :

```rust
use std::io;

fn main() {
    println!("=== Calculatrice Simple ===");
    println!("Entrez deux nombres et une opération (+, -, *, /)");

    // Lire le premier nombre
    println!("\nPremier nombre :");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Erreur de lecture");
    let num1: f64 = input1.trim().parse().expect("Nombre invalide");

    // Lire l'opération
    println!("Opération (+, -, *, /) :");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Erreur de lecture");
    let operation = operation.trim();

    // Lire le deuxième nombre
    println!("Deuxième nombre :");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Erreur de lecture");
    let num2: f64 = input2.trim().parse().expect("Nombre invalide");

    // Calculer le résultat
    let resultat = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Erreur : Division par zéro !");
                return;
            }
        }
        _ => {
            println!("Opération invalide !");
            return;
        }
    };

    println!("\nRésultat : {} {} {} = {}", num1, operation, num2, resultat);
}
```

**Exécuter l'application :**

```bash
cargo run
```

---

## 5. Commandes Cargo essentielles

```bash
# Créer un nouveau projet
cargo new mon_projet

# Compiler (mode debug)
cargo build

# Compiler (mode release, optimisé)
cargo build --release

# Compiler et exécuter
cargo run

# Vérifier le code sans compiler
cargo check

# Formater le code
cargo fmt

# Vérifier les erreurs de style
cargo clippy

# Exécuter les tests
cargo test

# Générer la documentation
cargo doc --open
```

---

## 6. Configuration de l'éditeur (optionnel)

### VS Code
```bash
# Installer l'extension rust-analyzer
code --install-extension rust-lang.rust-analyzer
```

### Vim/Neovim
Ajoutez rust-analyzer via votre gestionnaire de plugins (comme coc.nvim ou native LSP).

---

## Exercices pratiques

1. **Modifier la calculatrice** : Ajoutez le modulo (%) et la puissance (^)

2. **Application de conversion** : Créez un programme qui convertit des températures Celsius en Fahrenheit

3. **Jeu de devinette** : Créez un jeu où l'ordinateur choisit un nombre aléatoire et l'utilisateur doit le deviner
   - Indice : utilisez `use rand::Rng;` et ajoutez `rand = "0.8"` dans `Cargo.toml`

---

## Résumé

- Rust s'installe via `rustup` (méthode recommandée)
- `cargo` est l'outil principal pour gérer vos projets
- Les applications console utilisent `std::io` pour les entrées/sorties
- `cargo run` compile et exécute votre code en une commande
- Flatpak n'est pas adapté pour installer Rust directement

## Prochaine étape

Dans la leçon 2, nous explorerons les concepts fondamentaux de Rust : ownership, borrowing et types de données. Leçon 1 : Installation et Setup de Rust sur Ubuntu/Debian

## Objectifs
- Installer Rust sur un système Ubuntu/Debian
- Comprendre les outils de base (rustc, cargo)
- Créer et exécuter votre première application console

---

## 1. Installation de Rust

### Méthode recommandée : rustup (gestionnaire officiel)

Rustup est l'outil officiel pour installer et gérer Rust. C'est la méthode recommandée.

```bash
# Installation de rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Pendant l'installation, choisissez l'option par défaut (1) en appuyant sur Entrée.

**Configuration de l'environnement :**

```bash
# Recharger la configuration du shell
source $HOME/.cargo/env

# Vérifier l'installation
rustc --version
cargo --version
```

Vous devriez voir quelque chose comme :
```
rustc 1.xx.x (hash date)
cargo 1.xx.x (hash date)
```

### Alternative : Installation via APT (non recommandée)

```bash
sudo apt update
sudo apt install rustc cargo
```

⚠️ **Attention** : Cette méthode installe souvent une version obsolète de Rust.

### Alternative : Installation via Flatpak

Flatpak n'est pas recommandé pour installer Rust lui-même, car :
- Rust est un outil de développement, pas une application
- Les toolchains ont besoin d'accès direct au système
- Cargo a besoin de gérer des dépendances système

**Cependant**, vous pouvez utiliser Flatpak pour installer un IDE comme GNOME Builder qui inclut le support Rust.

---

## 2. Outils installés

Après l'installation, vous disposez de :

- **rustc** : Le compilateur Rust
- **cargo** : Le gestionnaire de paquets et système de build
- **rustup** : Le gestionnaire de toolchains Rust

### Commandes utiles de rustup

```bash
# Mettre à jour Rust
rustup update

# Afficher la version actuelle
rustup show

# Installer la documentation locale
rustup doc

# Changer de version
rustup default stable
```

---

## 3. Votre première application : "Bonjour Rust"

### Méthode 1 : Compilation directe avec rustc

```bash
# Créer un fichier source
echo 'fn main() {
    println!("Bonjour Rust !");
}' > bonjour.rs

# Compiler
rustc bonjour.rs

# Exécuter
./bonjour
```

### Méthode 2 : Projet avec Cargo (recommandée)

```bash
# Créer un nouveau projet
cargo new bonjour_rust
cd bonjour_rust

# Structure créée :
# bonjour_rust/
# ├── Cargo.toml
# └── src/
#     └── main.rs
```

Le fichier `src/main.rs` contient déjà :

```rust
fn main() {
    println!("Hello, world!");
}
```

**Modifier le code :**

```bash
# Éditer avec votre éditeur préféré
nano src/main.rs
```

```rust
fn main() {
    println!("Bonjour Rust !");
    println!("Bienvenue dans le monde de la programmation système.");
}
```

**Compiler et exécuter :**

```bash
# Compiler et exécuter en une commande
cargo run

# Ou compiler seulement
cargo build

# Puis exécuter le binaire
./target/debug/bonjour_rust
```

---

## 4. Application console simple : Calculatrice

Créons une application plus interactive.

```bash
cargo new calculatrice
cd calculatrice
```

Éditez `src/main.rs` :

```rust
use std::io;

fn main() {
    println!("=== Calculatrice Simple ===");
    println!("Entrez deux nombres et une opération (+, -, *, /)");

    // Lire le premier nombre
    println!("\nPremier nombre :");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Erreur de lecture");
    let num1: f64 = input1.trim().parse().expect("Nombre invalide");

    // Lire l'opération
    println!("Opération (+, -, *, /) :");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Erreur de lecture");
    let operation = operation.trim();

    // Lire le deuxième nombre
    println!("Deuxième nombre :");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Erreur de lecture");
    let num2: f64 = input2.trim().parse().expect("Nombre invalide");

    // Calculer le résultat
    let resultat = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Erreur : Division par zéro !");
                return;
            }
        }
        _ => {
            println!("Opération invalide !");
            return;
        }
    };

    println!("\nRésultat : {} {} {} = {}", num1, operation, num2, resultat);
}
```

**Exécuter l'application :**

```bash
cargo run
```

---

## 5. Commandes Cargo essentielles

```bash
# Créer un nouveau projet
cargo new mon_projet

# Compiler (mode debug)
cargo build

# Compiler (mode release, optimisé)
cargo build --release

# Compiler et exécuter
cargo run

# Vérifier le code sans compiler
cargo check

# Formater le code
cargo fmt

# Vérifier les erreurs de style
cargo clippy

# Exécuter les tests
cargo test

# Générer la documentation
cargo doc --open
```

---

## 6. Configuration de l'éditeur (optionnel)

### VS Code
```bash
# Installer l'extension rust-analyzer
code --install-extension rust-lang.rust-analyzer
```

### Vim/Neovim
Ajoutez rust-analyzer via votre gestionnaire de plugins (comme coc.nvim ou native LSP).

---

## Exercices pratiques

1. **Modifier la calculatrice** : Ajoutez le modulo (%) et la puissance (^)

2. **Application de conversion** : Créez un programme qui convertit des températures Celsius en Fahrenheit

3. **Jeu de devinette** : Créez un jeu où l'ordinateur choisit un nombre aléatoire et l'utilisateur doit le deviner
   - Indice : utilisez `use rand::Rng;` et ajoutez `rand = "0.8"` dans `Cargo.toml`

---

## Résumé

- Rust s'installe via `rustup` (méthode recommandée)
- `cargo` est l'outil principal pour gérer vos projets
- Les applications console utilisent `std::io` pour les entrées/sorties
- `cargo run` compile et exécute votre code en une commande
- Flatpak n'est pas adapté pour installer Rust directement

## Prochaine étape

Dans la leçon 2, nous explorerons les concepts fondamentaux de Rust : ownership, borrowing et types de données.rl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Pendant l'installation, choisissez l'option par défaut (1) en appuyant sur Entrée.

**Configuration de l'environnement :**

```bash
# Recharger la configuration du shell
source $HOME/.cargo/env

# Vérifier l'installation
rustc --version
cargo --version
```

Vous devriez voir quelque chose comme :
```
rustc 1.xx.x (hash date)
cargo 1.xx.x (hash date)
```

### Alternative : Installation via APT (non recommandée)

```bash
sudo apt update
sudo apt install rustc cargo
```

⚠️ **Attention** : Cette méthode installe souvent une version obsolète de Rust.

### Alternative : Installation via Flatpak

Flatpak n'est pas recommandé pour installer Rust lui-même, car :
- Rust est un outil de développement, pas une application
- Les toolchains ont besoin d'accès direct au système
- Cargo a besoin de gérer des dépendances système

**Cependant**, vous pouvez utiliser Flatpak pour installer un IDE comme GNOME Builder qui inclut le support Rust.

---

## 2. Outils installés

Après l'installation, vous disposez de :

- **rustc** : Le compilateur Rust
- **cargo** : Le gestionnaire de paquets et système de build
- **rustup** : Le gestionnaire de toolchains Rust

### Commandes utiles de rustup

```bash
# Mettre à jour Rust
rustup update

# Afficher la version actuelle
rustup show

# Installer la documentation locale
rustup doc

# Changer de version
rustup default stable
```

---

## 3. Votre première application : "Bonjour Rust"

### Méthode 1 : Compilation directe avec rustc

```bash
# Créer un fichier source
echo 'fn main() {
    println!("Bonjour Rust !");
}' > bonjour.rs

# Compiler
rustc bonjour.rs

# Exécuter
./bonjour
```

### Méthode 2 : Projet avec Cargo (recommandée)

```bash
# Créer un nouveau projet
cargo new bonjour_rust
cd bonjour_rust

# Structure créée :
# bonjour_rust/
# ├── Cargo.toml
# └── src/
#     └── main.rs
```

Le fichier `src/main.rs` contient déjà :

```rust
fn main() {
    println!("Hello, world!");
}
```

**Modifier le code :**

```bash
# Éditer avec votre éditeur préféré
nano src/main.rs
```

```rust
fn main() {
    println!("Bonjour Rust !");
    println!("Bienvenue dans le monde de la programmation système.");
}
```

**Compiler et exécuter :**

```bash
# Compiler et exécuter en une commande
cargo run

# Ou compiler seulement
cargo build

# Puis exécuter le binaire
./target/debug/bonjour_rust
```

---

## 4. Application console simple : Calculatrice

Créons une application plus interactive.

```bash
cargo new calculatrice
cd calculatrice
```

Éditez `src/main.rs` :

```rust
use std::io;

fn main() {
    println!("=== Calculatrice Simple ===");
    println!("Entrez deux nombres et une opération (+, -, *, /)");

    // Lire le premier nombre
    println!("\nPremier nombre :");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Erreur de lecture");
    let num1: f64 = input1.trim().parse().expect("Nombre invalide");

    // Lire l'opération
    println!("Opération (+, -, *, /) :");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Erreur de lecture");
    let operation = operation.trim();

    // Lire le deuxième nombre
    println!("Deuxième nombre :");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Erreur de lecture");
    let num2: f64 = input2.trim().parse().expect("Nombre invalide");

    // Calculer le résultat
    let resultat = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Erreur : Division par zéro !");
                return;
            }
        }
        _ => {
            println!("Opération invalide !");
            return;
        }
    };

    println!("\nRésultat : {} {} {} = {}", num1, operation, num2, resultat);
}
```

**Exécuter l'application :**

```bash
cargo run
```

---

## 5. Commandes Cargo essentielles

```bash
# Créer un nouveau projet
cargo new mon_projet

# Compiler (mode debug)
cargo build

# Compiler (mode release, optimisé)
cargo build --release

# Compiler et exécuter
cargo run

# Vérifier le code sans compiler
cargo check

# Formater le code
cargo fmt

# Vérifier les erreurs de style
cargo clippy

# Exécuter les tests
cargo test

# Générer la documentation
cargo doc --open
```

---

## 6. Configuration de l'éditeur (optionnel)

### VS Code
```bash
# Installer l'extension rust-analyzer
code --install-extension rust-lang.rust-analyzer
```

### Vim/Neovim
Ajoutez rust-analyzer via votre gestionnaire de plugins (comme coc.nvim ou native LSP).

---

## Exercices pratiques

1. **Modifier la calculatrice** : Ajoutez le modulo (%) et la puissance (^)

2. **Application de conversion** : Créez un programme qui convertit des températures Celsius en Fahrenheit

3. **Jeu de devinette** : Créez un jeu où l'ordinateur choisit un nombre aléatoire et l'utilisateur doit le deviner
   - Indice : utilisez `use rand::Rng;` et ajoutez `rand = "0.8"` dans `Cargo.toml`

---

## Résumé

- Rust s'installe via `rustup` (méthode recommandée)
- `cargo` est l'outil principal pour gérer vos projets
- Les applications console utilisent `std::io` pour les entrées/sorties
- `cargo run` compile et exécute votre code en une commande
- Flatpak n'est pas adapté pour installer Rust directement

## Prochaine étape

Dans la leçon 2, nous explorerons les concepts fondamentaux de Rust : ownership, borrowing et types de données.
