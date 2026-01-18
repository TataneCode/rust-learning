# Lecture et Écriture de Fichiers en Rust

## Introduction

Pour `FileStorage` et `FileLogger`, tu auras besoin de manipuler des fichiers. Rust propose le module `std::fs` pour cela.

---

## 1. Les Imports Essentiels

```rust
use std::fs;                          // Fonctions de haut niveau
use std::fs::{File, OpenOptions};     // Manipulation de fichiers
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::Path;                  // Manipulation de chemins
```

---

## 2. Écriture Simple (Tout en Une Fois)

### fs::write - Le Plus Simple

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // Écrit tout le contenu (crée ou écrase le fichier)
    fs::write("data.txt", "Contenu du fichier")?;

    // Avec des bytes
    fs::write("data.bin", &[0u8, 1, 2, 3])?;

    Ok(())
}
```

### Pour FileStorage

```rust
impl Storage for FileStorage {
    fn save(&self, key: &str, value: &str) -> Result<(), String> {
        let path = format!("{}/{}.txt", self.directory, key);
        fs::write(&path, value)
            .map_err(|e| format!("Erreur d'écriture: {}", e))
    }
}
```

---

## 3. Lecture Simple (Tout en Une Fois)

### fs::read_to_string

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // Lit tout le fichier en String
    let content = fs::read_to_string("data.txt")?;
    println!("{}", content);

    Ok(())
}
```

### fs::read (Bytes)

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // Lit en Vec<u8>
    let bytes = fs::read("data.bin")?;
    println!("{:?}", bytes);

    Ok(())
}
```

### Pour FileStorage

```rust
impl Storage for FileStorage {
    fn load(&self, key: &str) -> Result<String, String> {
        let path = format!("{}/{}.txt", self.directory, key);
        fs::read_to_string(&path)
            .map_err(|e| format!("Erreur de lecture: {}", e))
    }
}
```

---

## 4. Suppression de Fichiers

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // Supprimer un fichier
    fs::remove_file("data.txt")?;

    // Supprimer un dossier vide
    fs::remove_dir("mon_dossier")?;

    // Supprimer un dossier et son contenu
    fs::remove_dir_all("mon_dossier")?;

    Ok(())
}
```

### Pour FileStorage

```rust
impl Storage for FileStorage {
    fn delete(&self, key: &str) -> Result<(), String> {
        let path = format!("{}/{}.txt", self.directory, key);
        fs::remove_file(&path)
            .map_err(|e| format!("Erreur de suppression: {}", e))
    }
}
```

---

## 5. Création de Dossiers

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // Créer un dossier (erreur si existe)
    fs::create_dir("nouveau_dossier")?;

    // Créer un dossier et ses parents (pas d'erreur si existe)
    fs::create_dir_all("parent/enfant/petit_enfant")?;

    Ok(())
}
```

---

## 6. Vérifier l'Existence

```rust
use std::path::Path;

fn main() {
    let path = Path::new("mon_fichier.txt");

    if path.exists() {
        println!("Le chemin existe");
    }

    if path.is_file() {
        println!("C'est un fichier");
    }

    if path.is_dir() {
        println!("C'est un dossier");
    }
}
```

---

## 7. Manipulation Avancée avec File

Pour plus de contrôle, utilise `File` directement :

### Créer et Écrire

```rust
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Crée un nouveau fichier (ou tronque s'il existe)
    let mut file = File::create("output.txt")?;

    // Écrire
    file.write_all(b"Ligne 1\n")?;
    file.write_all(b"Ligne 2\n")?;

    // Avec write! macro
    writeln!(file, "Ligne {}", 3)?;

    Ok(())
}
```

### Ouvrir et Lire

```rust
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("{}", content);

    Ok(())
}
```

---

## 8. OpenOptions - Contrôle Total

`OpenOptions` permet de configurer précisément l'ouverture :

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Ouvrir en append (ajout à la fin)
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)  // Crée si n'existe pas
        .open("log.txt")?;

    writeln!(file, "Nouvelle ligne ajoutée")?;

    Ok(())
}
```

### Options Disponibles

```rust
OpenOptions::new()
    .read(true)       // Lecture
    .write(true)      // Écriture
    .append(true)     // Ajouter à la fin
    .truncate(true)   // Vider le fichier
    .create(true)     // Créer si n'existe pas
    .create_new(true) // Créer, erreur si existe
    .open("fichier.txt")?;
```

### Pour FileLogger (Append Mode)

```rust
use std::fs::OpenOptions;
use std::io::Write;

pub struct FileLogger {
    path: String,
}

impl FileLogger {
    pub fn new(path: &str) -> Self {
        FileLogger { path: path.to_string() }
    }

    fn write_log(&self, message: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)?;

        writeln!(file, "{}", message)?;
        Ok(())
    }
}

impl Logger for FileLogger {
    fn log(&self, level: LogLevel, message: &str) {
        let log_line = format!("[{:?}] {}", level, message);
        if let Err(e) = self.write_log(&log_line) {
            eprintln!("Erreur d'écriture log: {}", e);
        }
    }
}
```

---

## 9. Lecture Ligne par Ligne (BufReader)

Pour de gros fichiers, utilise `BufReader` :

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("large_file.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
```

---

## 10. Écriture Bufferisée (BufWriter)

Pour de nombreuses petites écritures :

```rust
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("output.txt")?;
    let mut writer = BufWriter::new(file);

    for i in 0..1000 {
        writeln!(writer, "Ligne {}", i)?;
    }

    writer.flush()?;  // Force l'écriture du buffer

    Ok(())
}
```

---

## 11. Gestion des Chemins avec Path et PathBuf

```rust
use std::path::{Path, PathBuf};

fn main() {
    // Path (référence, comme &str)
    let path = Path::new("dossier/fichier.txt");

    // PathBuf (owned, comme String)
    let mut path_buf = PathBuf::from("dossier");
    path_buf.push("sous_dossier");
    path_buf.push("fichier.txt");
    // Résultat: "dossier/sous_dossier/fichier.txt"

    // Composants
    println!("Parent: {:?}", path.parent());
    println!("Nom: {:?}", path.file_name());
    println!("Extension: {:?}", path.extension());
    println!("Stem: {:?}", path.file_stem());

    // Joindre des chemins
    let full_path = Path::new("base").join("sub").join("file.txt");
}
```

---

## 12. Exemple Complet : FileStorage

```rust
use std::fs;
use std::path::Path;

pub struct FileStorage {
    directory: String,
}

impl FileStorage {
    pub fn new(directory: &str) -> Result<Self, String> {
        // Créer le dossier s'il n'existe pas
        fs::create_dir_all(directory)
            .map_err(|e| format!("Impossible de créer le dossier: {}", e))?;

        Ok(FileStorage {
            directory: directory.to_string(),
        })
    }

    fn get_path(&self, key: &str) -> String {
        format!("{}/{}.dat", self.directory, key)
    }
}

impl Storage for FileStorage {
    fn save(&self, key: &str, value: &str) -> Result<(), String> {
        let path = self.get_path(key);
        fs::write(&path, value)
            .map_err(|e| format!("Erreur de sauvegarde '{}': {}", key, e))
    }

    fn load(&self, key: &str) -> Result<String, String> {
        let path = self.get_path(key);

        if !Path::new(&path).exists() {
            return Err(format!("Clé '{}' non trouvée", key));
        }

        fs::read_to_string(&path)
            .map_err(|e| format!("Erreur de lecture '{}': {}", key, e))
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        let path = self.get_path(key);

        if !Path::new(&path).exists() {
            return Err(format!("Clé '{}' non trouvée", key));
        }

        fs::remove_file(&path)
            .map_err(|e| format!("Erreur de suppression '{}': {}", key, e))
    }
}
```

---

## 13. Gestion des Erreurs io::Result

`std::io::Result<T>` est un alias pour `Result<T, std::io::Error>`.

```rust
use std::io;

fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn main() {
    match read_file("config.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => {
            // Analyser le type d'erreur
            match e.kind() {
                io::ErrorKind::NotFound => println!("Fichier non trouvé"),
                io::ErrorKind::PermissionDenied => println!("Permission refusée"),
                _ => println!("Autre erreur: {}", e),
            }
        }
    }
}
```

---

## Résumé des Fonctions

| Fonction | Usage |
|----------|-------|
| `fs::write(path, data)` | Écrire tout le contenu |
| `fs::read_to_string(path)` | Lire tout en String |
| `fs::read(path)` | Lire tout en Vec<u8> |
| `fs::remove_file(path)` | Supprimer un fichier |
| `fs::create_dir_all(path)` | Créer un dossier |
| `File::create(path)` | Créer/tronquer un fichier |
| `File::open(path)` | Ouvrir en lecture |
| `OpenOptions::new()...open()` | Contrôle total |

Pour `FileStorage` et `FileLogger`, les fonctions `fs::write`, `fs::read_to_string`, et `OpenOptions` (pour l'append) couvrent tous tes besoins.
