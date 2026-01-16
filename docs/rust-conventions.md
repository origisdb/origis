# 🦀 Conventions de Code Rust

> Ce document définit les conventions de code Rust pour le projet Origis.

---

## Outils obligatoires

| Outil | Usage | Commande |
|-------|-------|----------|
| **rustfmt** | Formatage automatique | `cargo fmt` |
| **clippy** | Linting | `cargo clippy` |

Ces outils sont exécutés automatiquement par la CI.

---

## Formatage

Nous utilisons la configuration par défaut de `rustfmt`.

```bash
# Formater tout le projet
cargo fmt

# Vérifier sans modifier
cargo fmt --check
```

---

## Nommage

| Élément | Convention | Exemple |
|---------|------------|---------|
| **Variables** | snake_case | `user_name`, `file_path` |
| **Fonctions** | snake_case | `create_snapshot()`, `parse_config()` |
| **Types/Structs** | PascalCase | `DatabaseConfig`, `SnapshotManager` |
| **Enums** | PascalCase | `CommandResult`, `ErrorKind` |
| **Variants** | PascalCase | `ErrorKind::NotFound` |
| **Constants** | SCREAMING_SNAKE_CASE | `MAX_CONNECTIONS`, `DEFAULT_PORT` |
| **Modules** | snake_case | `mod snapshot_manager;` |

---

## Structure des fichiers

```
src/
├── main.rs          # Point d'entrée
├── lib.rs           # API publique (si lib)
├── cli/             # Commandes CLI
│   ├── mod.rs
│   ├── init.rs
│   └── snapshot.rs
├── core/            # Logique métier
│   ├── mod.rs
│   └── ...
└── utils/           # Utilitaires
    └── mod.rs
```

---

## Gestion des erreurs

### Utiliser `Result<T, E>`

```rust
// ✅ Correct
fn read_config() -> Result<Config, ConfigError> {
    // ...
}

// ❌ Éviter panic dans le code métier
fn read_config() -> Config {
    panic!("Not found"); // ❌
}
```

### Utiliser l'opérateur `?`

```rust
// ✅ Correct
fn process() -> Result<(), Error> {
    let config = read_config()?;
    let db = connect_db(&config)?;
    Ok(())
}

// ❌ Éviter les unwrap() en production
fn process() {
    let config = read_config().unwrap(); // ❌
}
```

### Types d'erreur

Utiliser `thiserror` pour les erreurs custom :

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrigisError {
    #[error("Database not found: {0}")]
    DatabaseNotFound(String),
    
    #[error("Invalid snapshot: {0}")]
    InvalidSnapshot(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

---

## Documentation

### Documenter les fonctions publiques

```rust
/// Creates a new snapshot of the database.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `message` - Snapshot description
///
/// # Returns
///
/// The snapshot ID on success.
///
/// # Errors
///
/// Returns an error if the database is empty.
pub fn create_snapshot(db: &Database, message: &str) -> Result<SnapshotId, Error> {
    // ...
}
```

### Exemples dans la documentation

```rust
/// Parses a configuration file.
///
/// # Example
///
/// ```
/// let config = parse_config("origis.toml")?;
/// assert_eq!(config.database, "sqlite");
/// ```
pub fn parse_config(path: &str) -> Result<Config, Error> {
    // ...
}
```

---

## Patterns idiomatiques

### Utiliser `Option` et `Result`

```rust
// ✅ Utiliser Option pour les valeurs optionnelles
fn find_user(id: u32) -> Option<User> {
    // ...
}

// ✅ Utiliser les combinateurs
let name = find_user(42)
    .map(|u| u.name)
    .unwrap_or_default();
```

### Utiliser les itérateurs

```rust
// ✅ Idiomatique
let sum: i32 = numbers.iter().filter(|&n| n > 0).sum();

// ❌ Style impératif
let mut sum = 0;
for n in numbers {
    if n > 0 {
        sum += n;
    }
}
```

### Ownership et borrowing

```rust
// ✅ Emprunter quand possible
fn process(data: &str) { ... }

// ❌ Éviter de cloner inutilement
fn process(data: String) { ... } // Force un clone à l'appel
```

---

## Clippy

### Configuration

Activer les warnings stricts dans `main.rs` ou `lib.rs` :

```rust
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
```

### Exécution

```bash
# Lancer clippy
cargo clippy

# Avec warnings comme erreurs (CI)
cargo clippy -- -D warnings
```

---

## Tests

### Organisation

```rust
// Tests unitaires dans le même fichier
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_snapshot() {
        // ...
    }
}
```

### Tests d'intégration

```
tests/
├── integration_test.rs
└── common/
    └── mod.rs
```

---

## Principes SOLID (adaptés à Rust)

### S — Single Responsibility Principle

> Une struct/module ne doit avoir qu'une seule raison de changer.

```rust
// ❌ Mauvais : fait trop de choses
struct SnapshotManager {
    fn create_snapshot() { ... }
    fn send_email_notification() { ... }  // Pas sa responsabilité
    fn log_to_file() { ... }              // Pas sa responsabilité
}

// ✅ Bon : responsabilités séparées
struct SnapshotManager { ... }
struct NotificationService { ... }
struct Logger { ... }
```

### O — Open/Closed Principle

> Ouvert à l'extension, fermé à la modification.

```rust
// ✅ Utiliser des traits pour l'extensibilité
trait DatabaseAdapter {
    fn connect(&self) -> Result<Connection, Error>;
    fn snapshot(&self) -> Result<Snapshot, Error>;
}

struct SqliteAdapter;
struct PostgresAdapter;  // Ajouter sans modifier l'existant

impl DatabaseAdapter for SqliteAdapter { ... }
impl DatabaseAdapter for PostgresAdapter { ... }
```

### L — Liskov Substitution Principle

> Les types qui implémentent un trait doivent être interchangeables.

```rust
// ✅ Toutes les implémentations respectent le contrat
fn backup<T: DatabaseAdapter>(db: &T) -> Result<(), Error> {
    db.snapshot()?;  // Fonctionne avec n'importe quel adapter
    Ok(())
}
```

### I — Interface Segregation Principle

> Préférer plusieurs petits traits à un gros trait.

```rust
// ❌ Mauvais : trait trop large
trait Database {
    fn read(&self);
    fn write(&self);
    fn backup(&self);
    fn restore(&self);
    fn migrate(&self);
}

// ✅ Bon : traits séparés
trait Readable { fn read(&self); }
trait Writable { fn write(&self); }
trait Backupable { fn backup(&self); fn restore(&self); }
```

### D — Dependency Inversion Principle

> Dépendre des abstractions (traits), pas des implémentations.

```rust
// ❌ Mauvais : dépendance concrète
struct SnapshotService {
    db: SqliteDatabase,  // Couplé à SQLite
}

// ✅ Bon : dépendance abstraite
struct SnapshotService<T: DatabaseAdapter> {
    db: T,  // Fonctionne avec n'importe quel adapter
}
```

---

## Clean Code Practices

### Noms expressifs

```rust
// ❌ Noms cryptiques
fn proc(d: &str, n: i32) -> bool { ... }

// ✅ Noms clairs et descriptifs
fn process_snapshot(database_path: &str, max_retries: i32) -> bool { ... }
```

### Fonctions courtes et focalisées

```rust
// ❌ Fonction trop longue
fn create_and_validate_and_save_snapshot() { 
    // 200 lignes...
}

// ✅ Fonctions courtes avec une seule responsabilité
fn create_snapshot() -> Snapshot { ... }
fn validate_snapshot(s: &Snapshot) -> Result<(), Error> { ... }
fn save_snapshot(s: &Snapshot) -> Result<(), Error> { ... }
```

### Éviter les magic numbers

```rust
// ❌ Magic number
if retries > 3 { ... }

// ✅ Constante nommée
const MAX_RETRIES: u32 = 3;
if retries > MAX_RETRIES { ... }
```

### Early return

```rust
// ❌ Nesting profond
fn process(input: Option<Data>) -> Result<(), Error> {
    if let Some(data) = input {
        if data.is_valid() {
            if data.size() > 0 {
                // Logique...
            }
        }
    }
    Ok(())
}

// ✅ Early return
fn process(input: Option<Data>) -> Result<(), Error> {
    let data = input.ok_or(Error::NoInput)?;
    if !data.is_valid() {
        return Err(Error::InvalidData);
    }
    if data.size() == 0 {
        return Err(Error::EmptyData);
    }
    // Logique...
    Ok(())
}
```

### Pas de code mort

```rust
// ❌ Code commenté qui traîne
// fn old_function() { ... }

// ✅ Supprimer le code mort, Git garde l'historique
```

---

## Bonnes Pratiques Générales

### DRY — Don't Repeat Yourself

```rust
// ❌ Duplication
fn create_user_snapshot() { 
    let timestamp = chrono::Utc::now();
    // ...
}
fn create_table_snapshot() { 
    let timestamp = chrono::Utc::now();
    // ...
}

// ✅ Extraction
fn current_timestamp() -> DateTime<Utc> {
    chrono::Utc::now()
}
```

### KISS — Keep It Simple, Stupid

```rust
// ❌ Sur-ingénierie
trait SnapshotStrategyFactory<T: Clone + Send + Sync> { ... }

// ✅ Simple et direct
fn create_snapshot(db: &Database) -> Snapshot { ... }
```

### YAGNI — You Ain't Gonna Need It

```rust
// ❌ Features "au cas où"
struct Config {
    database_url: String,
    redis_url: Option<String>,      // Pas prévu pour le MVP
    kafka_brokers: Vec<String>,     // Pas prévu pour le MVP
}

// ✅ Seulement ce qui est nécessaire maintenant
struct Config {
    database_url: String,
}
```

### Fail Fast

```rust
// ✅ Valider les inputs dès le début
fn process(path: &str) -> Result<(), Error> {
    if path.is_empty() {
        return Err(Error::EmptyPath);  // Fail immédiatement
    }
    // Continue seulement si valid
}
```

### Composition over Inheritance

Rust n'a pas d'héritage, mais le principe s'applique :

```rust
// ✅ Composer avec des traits
struct SnapshotService {
    db: Box<dyn DatabaseAdapter>,
    logger: Box<dyn Logger>,
    notifier: Box<dyn Notifier>,
}
```

---

## Code Review Checklist

Avant chaque PR, vérifier :

- [ ] Les noms sont-ils explicites ?
- [ ] Les fonctions font-elles une seule chose ?
- [ ] Y a-t-il de la duplication ?
- [ ] Les erreurs sont-elles gérées proprement ?
- [ ] Le code est-il testable ?
- [ ] Les dépendances sont-elles abstraites (traits) ?
- [ ] Le code est-il plus simple que nécessaire ?

---

## Ressources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Clean Code (Robert C. Martin)](https://www.oreilly.com/library/view/clean-code-a/9780136083238/)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
