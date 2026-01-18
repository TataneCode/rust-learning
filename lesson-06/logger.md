# Système de Logging en Rust

## Introduction

Pour l'exercice 2, tu dois créer un trait `Logger` avec plusieurs implémentations. Ce fichier explique les concepts et patterns de logging en Rust.

---

## 1. Définir les Niveaux de Log

### Enum LogLevel

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,    // Très détaillé
    Debug,    // Développement
    Info,     // Information générale
    Warn,     // Avertissement
    Error,    // Erreur
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

// Permet la comparaison: LogLevel::Error > LogLevel::Info
```

### Avec Display

```rust
use std::fmt;

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```

---

## 2. Le Trait Logger

### Définition avec Méthodes par Défaut

```rust
pub trait Logger {
    /// Méthode principale à implémenter
    fn log(&self, level: LogLevel, message: &str);

    /// Niveau minimum pour afficher (optionnel)
    fn min_level(&self) -> LogLevel {
        LogLevel::Debug
    }

    /// Méthodes de convenance (utilisent log())
    fn trace(&self, message: &str) {
        if LogLevel::Trace >= self.min_level() {
            self.log(LogLevel::Trace, message);
        }
    }

    fn debug(&self, message: &str) {
        if LogLevel::Debug >= self.min_level() {
            self.log(LogLevel::Debug, message);
        }
    }

    fn info(&self, message: &str) {
        if LogLevel::Info >= self.min_level() {
            self.log(LogLevel::Info, message);
        }
    }

    fn warn(&self, message: &str) {
        if LogLevel::Warn >= self.min_level() {
            self.log(LogLevel::Warn, message);
        }
    }

    fn error(&self, message: &str) {
        if LogLevel::Error >= self.min_level() {
            self.log(LogLevel::Error, message);
        }
    }
}
```

---

## 3. Implémentation : ConsoleLogger

### Version Simple

```rust
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", level, message);
    }
}
```

### Avec Timestamp

```rust
use std::time::SystemTime;

pub struct ConsoleLogger {
    min_level: LogLevel,
}

impl ConsoleLogger {
    pub fn new(min_level: LogLevel) -> Self {
        ConsoleLogger { min_level }
    }

    fn timestamp(&self) -> String {
        // Timestamp simple (secondes depuis UNIX epoch)
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => format!("{}", d.as_secs()),
            Err(_) => String::from("???"),
        }
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        if level >= self.min_level {
            println!("[{}] [{}] {}", self.timestamp(), level, message);
        }
    }

    fn min_level(&self) -> LogLevel {
        self.min_level
    }
}
```

### Avec Couleurs (ANSI)

```rust
pub struct ColoredConsoleLogger {
    min_level: LogLevel,
}

impl ColoredConsoleLogger {
    fn color_code(&self, level: LogLevel) -> &'static str {
        match level {
            LogLevel::Trace => "\x1b[90m",  // Gris
            LogLevel::Debug => "\x1b[36m",  // Cyan
            LogLevel::Info => "\x1b[32m",   // Vert
            LogLevel::Warn => "\x1b[33m",   // Jaune
            LogLevel::Error => "\x1b[31m",  // Rouge
        }
    }

    const RESET: &'static str = "\x1b[0m";
}

impl Logger for ColoredConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        if level >= self.min_level {
            println!(
                "{}[{}]{} {}",
                self.color_code(level),
                level,
                Self::RESET,
                message
            );
        }
    }
}
```

---

## 4. Implémentation : FileLogger

```rust
use std::fs::OpenOptions;
use std::io::Write;

pub struct FileLogger {
    path: String,
    min_level: LogLevel,
}

impl FileLogger {
    pub fn new(path: &str, min_level: LogLevel) -> Self {
        FileLogger {
            path: path.to_string(),
            min_level,
        }
    }

    fn write_to_file(&self, message: &str) {
        let result = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .and_then(|mut file| writeln!(file, "{}", message));

        if let Err(e) = result {
            eprintln!("Erreur d'écriture dans le log: {}", e);
        }
    }

    fn timestamp(&self) -> String {
        use std::time::SystemTime;
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => format!("{}", d.as_secs()),
            Err(_) => String::from("0"),
        }
    }
}

impl Logger for FileLogger {
    fn log(&self, level: LogLevel, message: &str) {
        if level >= self.min_level {
            let line = format!("[{}] [{}] {}", self.timestamp(), level, message);
            self.write_to_file(&line);
        }
    }

    fn min_level(&self) -> LogLevel {
        self.min_level
    }
}
```

---

## 5. Implémentation : NullLogger

Le "Null Object Pattern" - un logger qui ne fait rien.

```rust
pub struct NullLogger;

impl Logger for NullLogger {
    fn log(&self, _level: LogLevel, _message: &str) {
        // Ne fait rien intentionnellement
    }
}
```

**Utilité :**
- Désactiver les logs sans modifier le code
- Tests où les logs ne sont pas nécessaires
- Évite les `Option<Box<dyn Logger>>` partout

---

## 6. Combiner Plusieurs Loggers

### MultiLogger

```rust
pub struct MultiLogger {
    loggers: Vec<Box<dyn Logger>>,
}

impl MultiLogger {
    pub fn new() -> Self {
        MultiLogger { loggers: Vec::new() }
    }

    pub fn add(&mut self, logger: Box<dyn Logger>) {
        self.loggers.push(logger);
    }
}

impl Logger for MultiLogger {
    fn log(&self, level: LogLevel, message: &str) {
        for logger in &self.loggers {
            logger.log(level, message);
        }
    }
}

// Utilisation
fn main() {
    let mut multi = MultiLogger::new();
    multi.add(Box::new(ConsoleLogger::new(LogLevel::Debug)));
    multi.add(Box::new(FileLogger::new("app.log", LogLevel::Info)));

    multi.info("Ce message va dans la console ET le fichier");
}
```

---

## 7. Formatage des Messages

### Trait pour le Formatage

```rust
pub trait LogFormatter {
    fn format(&self, level: LogLevel, message: &str) -> String;
}

pub struct SimpleFormatter;

impl LogFormatter for SimpleFormatter {
    fn format(&self, level: LogLevel, message: &str) -> String {
        format!("[{}] {}", level, message)
    }
}

pub struct DetailedFormatter {
    app_name: String,
}

impl LogFormatter for DetailedFormatter {
    fn format(&self, level: LogLevel, message: &str) -> String {
        use std::time::SystemTime;
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        format!("{} [{}] [{}] {}", ts, self.app_name, level, message)
    }
}
```

---

## 8. Exemple Complet

```rust
// lib.rs ou main.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };
        write!(f, "{}", s)
    }
}

pub trait Logger {
    fn log(&self, level: LogLevel, message: &str);

    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

// Implémentations
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", level, message);
    }
}

pub struct FileLogger {
    path: String,
}

impl FileLogger {
    pub fn new(path: &str) -> Self {
        FileLogger { path: path.to_string() }
    }
}

impl Logger for FileLogger {
    fn log(&self, level: LogLevel, message: &str) {
        use std::fs::OpenOptions;
        use std::io::Write;

        if let Ok(mut file) = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
        {
            let _ = writeln!(file, "[{}] {}", level, message);
        }
    }
}

pub struct NullLogger;

impl Logger for NullLogger {
    fn log(&self, _level: LogLevel, _message: &str) {}
}

// Service utilisant l'injection de dépendance
pub struct MyService<L: Logger> {
    logger: L,
}

impl<L: Logger> MyService<L> {
    pub fn new(logger: L) -> Self {
        MyService { logger }
    }

    pub fn do_something(&self) {
        self.logger.info("Début de l'opération");
        // ... travail ...
        self.logger.info("Fin de l'opération");
    }
}

fn main() {
    // Avec ConsoleLogger
    let service = MyService::new(ConsoleLogger);
    service.do_something();

    // Avec FileLogger
    let service = MyService::new(FileLogger::new("app.log"));
    service.do_something();

    // Sans logs (tests, production silencieuse)
    let service = MyService::new(NullLogger);
    service.do_something();
}
```

---

## 9. Crates de Logging Populaires

Pour des projets réels, ces crates sont recommandées :

### log (Facade Standard)

```toml
[dependencies]
log = "0.4"
```

```rust
use log::{info, warn, error, debug, trace};

fn main() {
    info!("Application démarrée");
    warn!("Attention: {}", "quelque chose");
    error!("Erreur critique!");
}
```

### env_logger (Implémentation Simple)

```toml
[dependencies]
log = "0.4"
env_logger = "0.11"
```

```rust
fn main() {
    env_logger::init();

    log::info!("Hello!");
}
```

```bash
# Configurer le niveau via variable d'environnement
RUST_LOG=debug cargo run
RUST_LOG=my_app=trace cargo run
```

### tracing (Moderne, Async-friendly)

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
use tracing::{info, warn, error, instrument, span, Level};

#[instrument]
fn process_request(id: u32) {
    info!("Traitement de la requête");
}

fn main() {
    tracing_subscriber::fmt::init();

    let span = span!(Level::INFO, "app_startup");
    let _enter = span.enter();

    info!("Application démarrée");
    process_request(42);
}
```

---

## 10. Bonnes Pratiques

### 1. Utiliser des Niveaux Appropriés

| Niveau | Usage |
|--------|-------|
| `Trace` | Détails fins (boucles, variables) |
| `Debug` | Info de développement |
| `Info` | Événements normaux importants |
| `Warn` | Problèmes potentiels |
| `Error` | Erreurs qui affectent le fonctionnement |

### 2. Messages Informatifs

```rust
// Mauvais
logger.info("Error");

// Bon
logger.error("Échec de connexion à la DB: timeout après 30s");
```

### 3. Éviter les Logs Sensibles

```rust
// Mauvais
logger.info(&format!("User login: password={}", password));

// Bon
logger.info(&format!("User login: user_id={}", user_id));
```

### 4. Structurer pour le Parsing

```rust
// Format parsable
logger.info(&format!(
    "event=request_completed status=200 duration_ms=45 path=/api/users"
));
```

---

## Résumé

| Type | Rôle |
|------|------|
| `LogLevel` | Enum des niveaux (Trace, Debug, Info, Warn, Error) |
| `Logger` trait | Contrat avec `log()` et méthodes par défaut |
| `ConsoleLogger` | Affiche sur stdout |
| `FileLogger` | Écrit dans un fichier (append) |
| `NullLogger` | Ne fait rien (désactive les logs) |

Pour l'exercice, implémente ces trois structs avec le trait `Logger`. Tu peux ensuite les injecter dans n'importe quel service grâce à l'inversion de dépendance vue dans la leçon.
