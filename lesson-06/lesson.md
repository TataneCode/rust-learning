# Leçon 6 : Inversion de Dépendance avec les Traits

## Introduction

L'**inversion de dépendance** est le principe "D" des principes SOLID. Il stipule que :
- Les modules de haut niveau ne doivent pas dépendre des modules de bas niveau
- Les deux doivent dépendre d'abstractions

En Rust, les **traits** sont le mécanisme principal pour créer ces abstractions. Cette leçon vous apprendra à :
- Définir des traits comme contrats d'interface
- Créer plusieurs implémentations d'un même trait
- Injecter différentes implémentations dans votre code
- Choisir entre génériques et trait objects (`dyn Trait`)

## 1. Le Problème : Couplage Fort

### Sans Inversion de Dépendance

Imaginons un système de notification :

```rust
// ❌ Mauvaise approche : couplage fort
struct EmailSender {
    smtp_server: String,
}

impl EmailSender {
    fn send(&self, message: &str) {
        println!("Envoi email via {} : {}", self.smtp_server, message);
    }
}

struct OrderService {
    email_sender: EmailSender,  // Dépendance concrète !
}

impl OrderService {
    fn new() -> Self {
        OrderService {
            email_sender: EmailSender {
                smtp_server: String::from("smtp.example.com"),
            },
        }
    }

    fn process_order(&self, order_id: u32) {
        println!("Traitement de la commande #{}", order_id);
        self.email_sender.send(&format!("Commande #{} confirmée", order_id));
    }
}
```

**Problèmes :**
- `OrderService` est lié à `EmailSender`
- Impossible de changer le système de notification sans modifier `OrderService`
- Difficile à tester (on ne peut pas simuler l'envoi d'email)

## 2. La Solution : Traits comme Abstractions

### Définir un Trait

```rust
// ✅ Bonne approche : définir une abstraction
trait Notifier {
    fn notify(&self, message: &str);
}
```

### Implémenter le Trait pour Différents Types

```rust
// Implémentation Email
struct EmailNotifier {
    smtp_server: String,
}

impl Notifier for EmailNotifier {
    fn notify(&self, message: &str) {
        println!("[EMAIL via {}] {}", self.smtp_server, message);
    }
}

// Implémentation SMS
struct SmsNotifier {
    phone_number: String,
}

impl Notifier for SmsNotifier {
    fn notify(&self, message: &str) {
        println!("[SMS à {}] {}", self.phone_number, message);
    }
}

// Implémentation Console (pour les tests)
struct ConsoleNotifier;

impl Notifier for ConsoleNotifier {
    fn notify(&self, message: &str) {
        println!("[CONSOLE] {}", message);
    }
}
```

## 3. Injection de Dépendance avec Génériques

La première approche utilise les **génériques** avec des contraintes de trait :

```rust
// T doit implémenter le trait Notifier
struct OrderService<T: Notifier> {
    notifier: T,
}

impl<T: Notifier> OrderService<T> {
    fn new(notifier: T) -> Self {
        OrderService { notifier }
    }

    fn process_order(&self, order_id: u32) {
        println!("Traitement de la commande #{}", order_id);
        self.notifier.notify(&format!("Commande #{} confirmée", order_id));
    }
}

fn main() {
    // Avec Email
    let email_notifier = EmailNotifier {
        smtp_server: String::from("smtp.example.com"),
    };
    let order_service_email = OrderService::new(email_notifier);
    order_service_email.process_order(1001);

    // Avec SMS
    let sms_notifier = SmsNotifier {
        phone_number: String::from("+33612345678"),
    };
    let order_service_sms = OrderService::new(sms_notifier);
    order_service_sms.process_order(1002);

    // Avec Console (pour tests)
    let console_notifier = ConsoleNotifier;
    let order_service_test = OrderService::new(console_notifier);
    order_service_test.process_order(1003);
}
```

**Avantages des génériques :**
- Pas d'allocation dynamique (pas de `Box`)
- Monomorphisation : le compilateur génère du code optimisé pour chaque type
- Performance maximale

**Inconvénients :**
- Chaque `OrderService<EmailNotifier>` et `OrderService<SmsNotifier>` sont des types différents
- Impossible de stocker différentes implémentations dans une collection

## 4. Injection de Dépendance avec Trait Objects (`dyn`)

Pour plus de flexibilité, utilisez les **trait objects** :

```rust
// Utilise un pointeur vers un trait (trait object)
struct OrderServiceDyn {
    notifier: Box<dyn Notifier>,
}

impl OrderServiceDyn {
    fn new(notifier: Box<dyn Notifier>) -> Self {
        OrderServiceDyn { notifier }
    }

    fn process_order(&self, order_id: u32) {
        println!("Traitement de la commande #{}", order_id);
        self.notifier.notify(&format!("Commande #{} confirmée", order_id));
    }
}

fn main() {
    // On peut changer l'implémentation à l'exécution
    let notifier: Box<dyn Notifier> = if cfg!(debug_assertions) {
        Box::new(ConsoleNotifier)
    } else {
        Box::new(EmailNotifier {
            smtp_server: String::from("smtp.example.com"),
        })
    };

    let service = OrderServiceDyn::new(notifier);
    service.process_order(1001);
}
```

### Stocker Plusieurs Implémentations

```rust
fn main() {
    // Collection de différents notifiers
    let notifiers: Vec<Box<dyn Notifier>> = vec![
        Box::new(EmailNotifier {
            smtp_server: String::from("smtp.example.com"),
        }),
        Box::new(SmsNotifier {
            phone_number: String::from("+33612345678"),
        }),
        Box::new(ConsoleNotifier),
    ];

    // Notifier tous
    for notifier in &notifiers {
        notifier.notify("Message broadcast");
    }
}
```

**Avantages de `dyn Trait` :**
- Flexibilité à l'exécution
- Un seul type pour différentes implémentations
- Collections hétérogènes possibles

**Inconvénients :**
- Allocation sur le tas (`Box`)
- Dispatch dynamique (vtable lookup)
- Légèrement moins performant

## 5. Comparaison : Génériques vs `dyn Trait`

| Critère | Génériques `<T: Trait>` | Trait Objects `dyn Trait` |
|---------|-------------------------|---------------------------|
| Performance | Optimale (inlining) | Légère indirection |
| Allocation | Stack | Heap (Box) |
| Flexibilité | Type fixé à la compilation | Changeable à l'exécution |
| Collections | Homogènes uniquement | Hétérogènes possibles |
| Taille binaire | Plus grande (monomorphisation) | Plus petite |

### Règle Générale

- **Utilisez les génériques** quand vous connaissez le type à la compilation et voulez la performance maximale
- **Utilisez `dyn Trait`** quand vous avez besoin de flexibilité à l'exécution ou de collections hétérogènes

## 6. Pattern Factory avec Traits

Créons une factory qui retourne différentes implémentations :

```rust
enum NotifierType {
    Email,
    Sms,
    Console,
}

fn create_notifier(notifier_type: NotifierType) -> Box<dyn Notifier> {
    match notifier_type {
        NotifierType::Email => Box::new(EmailNotifier {
            smtp_server: String::from("smtp.example.com"),
        }),
        NotifierType::Sms => Box::new(SmsNotifier {
            phone_number: String::from("+33612345678"),
        }),
        NotifierType::Console => Box::new(ConsoleNotifier),
    }
}

fn main() {
    let notifier = create_notifier(NotifierType::Email);
    notifier.notify("Bonjour depuis la factory !");
}
```

## 7. Traits avec Méthodes par Défaut

Les traits peuvent fournir des implémentations par défaut :

```rust
trait Notifier {
    fn notify(&self, message: &str);

    // Méthode avec implémentation par défaut
    fn notify_urgent(&self, message: &str) {
        let urgent_msg = format!("[URGENT] {}", message);
        self.notify(&urgent_msg);
    }

    fn notify_multiple(&self, messages: &[&str]) {
        for msg in messages {
            self.notify(msg);
        }
    }
}

// EmailNotifier hérite automatiquement de notify_urgent et notify_multiple
impl Notifier for EmailNotifier {
    fn notify(&self, message: &str) {
        println!("[EMAIL] {}", message);
    }
}

fn main() {
    let notifier = EmailNotifier {
        smtp_server: String::from("smtp.example.com"),
    };

    notifier.notify("Message normal");
    notifier.notify_urgent("Le serveur est down !");
    notifier.notify_multiple(&["Premier", "Deuxième", "Troisième"]);
}
```

## 8. Projet Pratique : Système de Paiement

Créons un système de paiement avec plusieurs providers.

### Structure du Projet

```
payment_system/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── payments/
    │   ├── mod.rs
    │   ├── traits.rs
    │   ├── stripe.rs
    │   ├── paypal.rs
    │   └── mock.rs
    └── services/
        ├── mod.rs
        └── checkout.rs
```

### Cargo.toml

```toml
[package]
name = "payment_system"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### src/payments/traits.rs

```rust
#[derive(Debug, Clone)]
pub struct PaymentResult {
    pub success: bool,
    pub transaction_id: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct PaymentRequest {
    pub amount: f64,
    pub currency: String,
    pub description: String,
}

// Le trait définit le contrat pour tous les processeurs de paiement
pub trait PaymentProcessor {
    fn name(&self) -> &str;
    fn process_payment(&self, request: &PaymentRequest) -> PaymentResult;
    fn refund(&self, transaction_id: &str) -> PaymentResult;

    // Méthode par défaut
    fn supports_currency(&self, currency: &str) -> bool {
        matches!(currency, "EUR" | "USD")
    }
}
```

### src/payments/stripe.rs

```rust
use super::traits::{PaymentProcessor, PaymentRequest, PaymentResult};

pub struct StripeProcessor {
    api_key: String,
}

impl StripeProcessor {
    pub fn new(api_key: String) -> Self {
        StripeProcessor { api_key }
    }
}

impl PaymentProcessor for StripeProcessor {
    fn name(&self) -> &str {
        "Stripe"
    }

    fn process_payment(&self, request: &PaymentRequest) -> PaymentResult {
        println!(
            "[Stripe] Traitement de {:.2} {} - {}",
            request.amount, request.currency, request.description
        );

        // Simulation d'appel API
        PaymentResult {
            success: true,
            transaction_id: format!("stripe_txn_{}", rand_id()),
            message: String::from("Paiement accepté via Stripe"),
        }
    }

    fn refund(&self, transaction_id: &str) -> PaymentResult {
        println!("[Stripe] Remboursement de {}", transaction_id);
        PaymentResult {
            success: true,
            transaction_id: format!("stripe_refund_{}", rand_id()),
            message: String::from("Remboursement effectué"),
        }
    }

    fn supports_currency(&self, currency: &str) -> bool {
        matches!(currency, "EUR" | "USD" | "GBP" | "CHF")
    }
}

fn rand_id() -> u32 {
    // Simulation d'ID aléatoire
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
}
```

### src/payments/paypal.rs

```rust
use super::traits::{PaymentProcessor, PaymentRequest, PaymentResult};

pub struct PayPalProcessor {
    client_id: String,
    sandbox: bool,
}

impl PayPalProcessor {
    pub fn new(client_id: String, sandbox: bool) -> Self {
        PayPalProcessor { client_id, sandbox }
    }
}

impl PaymentProcessor for PayPalProcessor {
    fn name(&self) -> &str {
        "PayPal"
    }

    fn process_payment(&self, request: &PaymentRequest) -> PaymentResult {
        let env = if self.sandbox { "SANDBOX" } else { "PROD" };
        println!(
            "[PayPal {}] Traitement de {:.2} {} - {}",
            env, request.amount, request.currency, request.description
        );

        PaymentResult {
            success: true,
            transaction_id: format!("paypal_{}", rand_id()),
            message: String::from("Paiement accepté via PayPal"),
        }
    }

    fn refund(&self, transaction_id: &str) -> PaymentResult {
        println!("[PayPal] Remboursement de {}", transaction_id);
        PaymentResult {
            success: true,
            transaction_id: format!("paypal_refund_{}", rand_id()),
            message: String::from("Remboursement PayPal effectué"),
        }
    }
}

fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
}
```

### src/payments/mock.rs

```rust
use super::traits::{PaymentProcessor, PaymentRequest, PaymentResult};

/// Processeur de paiement simulé pour les tests
pub struct MockProcessor {
    should_succeed: bool,
}

impl MockProcessor {
    pub fn new(should_succeed: bool) -> Self {
        MockProcessor { should_succeed }
    }

    pub fn always_success() -> Self {
        MockProcessor { should_succeed: true }
    }

    pub fn always_fail() -> Self {
        MockProcessor { should_succeed: false }
    }
}

impl PaymentProcessor for MockProcessor {
    fn name(&self) -> &str {
        "MockProcessor"
    }

    fn process_payment(&self, request: &PaymentRequest) -> PaymentResult {
        println!(
            "[MOCK] Simulation de paiement: {:.2} {}",
            request.amount, request.currency
        );

        if self.should_succeed {
            PaymentResult {
                success: true,
                transaction_id: String::from("mock_txn_12345"),
                message: String::from("Paiement simulé réussi"),
            }
        } else {
            PaymentResult {
                success: false,
                transaction_id: String::new(),
                message: String::from("Paiement simulé échoué"),
            }
        }
    }

    fn refund(&self, transaction_id: &str) -> PaymentResult {
        println!("[MOCK] Simulation de remboursement: {}", transaction_id);
        PaymentResult {
            success: self.should_succeed,
            transaction_id: String::from("mock_refund_12345"),
            message: String::from("Remboursement simulé"),
        }
    }
}
```

### src/payments/mod.rs

```rust
pub mod traits;
pub mod stripe;
pub mod paypal;
pub mod mock;

pub use traits::{PaymentProcessor, PaymentRequest, PaymentResult};
pub use stripe::StripeProcessor;
pub use paypal::PayPalProcessor;
pub use mock::MockProcessor;
```

### src/services/checkout.rs

```rust
use crate::payments::{PaymentProcessor, PaymentRequest, PaymentResult};

/// Service de checkout qui utilise l'inversion de dépendance
pub struct CheckoutService<P: PaymentProcessor> {
    processor: P,
}

impl<P: PaymentProcessor> CheckoutService<P> {
    pub fn new(processor: P) -> Self {
        CheckoutService { processor }
    }

    pub fn checkout(&self, amount: f64, currency: &str, description: &str) -> PaymentResult {
        // Vérification de la devise
        if !self.processor.supports_currency(currency) {
            return PaymentResult {
                success: false,
                transaction_id: String::new(),
                message: format!(
                    "La devise {} n'est pas supportée par {}",
                    currency,
                    self.processor.name()
                ),
            };
        }

        let request = PaymentRequest {
            amount,
            currency: currency.to_string(),
            description: description.to_string(),
        };

        println!("=== Checkout via {} ===", self.processor.name());
        self.processor.process_payment(&request)
    }
}

/// Version avec trait object pour plus de flexibilité
pub struct CheckoutServiceDyn {
    processor: Box<dyn PaymentProcessor>,
}

impl CheckoutServiceDyn {
    pub fn new(processor: Box<dyn PaymentProcessor>) -> Self {
        CheckoutServiceDyn { processor }
    }

    pub fn set_processor(&mut self, processor: Box<dyn PaymentProcessor>) {
        self.processor = processor;
    }

    pub fn checkout(&self, amount: f64, currency: &str, description: &str) -> PaymentResult {
        if !self.processor.supports_currency(currency) {
            return PaymentResult {
                success: false,
                transaction_id: String::new(),
                message: format!(
                    "La devise {} n'est pas supportée par {}",
                    currency,
                    self.processor.name()
                ),
            };
        }

        let request = PaymentRequest {
            amount,
            currency: currency.to_string(),
            description: description.to_string(),
        };

        println!("=== Checkout via {} ===", self.processor.name());
        self.processor.process_payment(&request)
    }
}
```

### src/services/mod.rs

```rust
pub mod checkout;
pub use checkout::{CheckoutService, CheckoutServiceDyn};
```

### src/main.rs

```rust
mod payments;
mod services;

use payments::{StripeProcessor, PayPalProcessor, MockProcessor, PaymentProcessor};
use services::{CheckoutService, CheckoutServiceDyn};

fn main() {
    println!("=== Démonstration : Inversion de Dépendance ===\n");

    // 1. Avec génériques (type fixé à la compilation)
    println!("--- Avec Génériques ---\n");

    let stripe = StripeProcessor::new(String::from("sk_test_xxx"));
    let checkout_stripe = CheckoutService::new(stripe);
    let result = checkout_stripe.checkout(99.99, "EUR", "Achat de livre");
    println!("Résultat: {:?}\n", result);

    let paypal = PayPalProcessor::new(String::from("client_xxx"), true);
    let checkout_paypal = CheckoutService::new(paypal);
    let result = checkout_paypal.checkout(149.99, "USD", "Abonnement premium");
    println!("Résultat: {:?}\n", result);

    // 2. Avec trait objects (flexibilité à l'exécution)
    println!("--- Avec Trait Objects ---\n");

    // Sélection du processeur basée sur une condition
    let use_sandbox = true;
    let processor: Box<dyn PaymentProcessor> = if use_sandbox {
        println!("Mode sandbox activé - utilisation du MockProcessor");
        Box::new(MockProcessor::always_success())
    } else {
        println!("Mode production - utilisation de Stripe");
        Box::new(StripeProcessor::new(String::from("sk_live_xxx")))
    };

    let mut checkout = CheckoutServiceDyn::new(processor);
    let result = checkout.checkout(299.99, "EUR", "Formation Rust");
    println!("Résultat: {:?}\n", result);

    // Changement de processeur à l'exécution
    println!("--- Changement de processeur ---\n");
    checkout.set_processor(Box::new(PayPalProcessor::new(
        String::from("client_live"),
        false,
    )));
    let result = checkout.checkout(49.99, "EUR", "E-book");
    println!("Résultat: {:?}\n", result);

    // 3. Collection de processeurs
    println!("--- Collection de Processeurs ---\n");

    let processors: Vec<Box<dyn PaymentProcessor>> = vec![
        Box::new(StripeProcessor::new(String::from("sk_test"))),
        Box::new(PayPalProcessor::new(String::from("client_id"), true)),
        Box::new(MockProcessor::always_success()),
    ];

    for processor in &processors {
        println!("Processeur disponible: {}", processor.name());
        println!("  Supporte EUR: {}", processor.supports_currency("EUR"));
        println!("  Supporte BTC: {}", processor.supports_currency("BTC"));
    }
}
```

## 9. Tests avec Mock

L'inversion de dépendance facilite grandement les tests :

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::payments::MockProcessor;
    use crate::services::CheckoutService;

    #[test]
    fn test_checkout_success() {
        let mock = MockProcessor::always_success();
        let service = CheckoutService::new(mock);

        let result = service.checkout(100.0, "EUR", "Test");

        assert!(result.success);
        assert!(!result.transaction_id.is_empty());
    }

    #[test]
    fn test_checkout_failure() {
        let mock = MockProcessor::always_fail();
        let service = CheckoutService::new(mock);

        let result = service.checkout(100.0, "EUR", "Test");

        assert!(!result.success);
    }

    #[test]
    fn test_unsupported_currency() {
        let mock = MockProcessor::always_success();
        let service = CheckoutService::new(mock);

        // BTC n'est pas supporté par défaut
        let result = service.checkout(1.0, "BTC", "Test");

        assert!(!result.success);
        assert!(result.message.contains("n'est pas supportée"));
    }
}
```

## 10. Points Clés à Retenir

### L'Inversion de Dépendance

1. **Définir des abstractions** : Créez des traits qui définissent les comportements attendus
2. **Implémenter pour chaque cas** : Chaque implémentation concrète répond au contrat du trait
3. **Dépendre de l'abstraction** : Les modules de haut niveau acceptent des paramètres génériques ou des trait objects

### Quand Utiliser Quoi

| Situation | Approche |
|-----------|----------|
| Performance critique | Génériques `<T: Trait>` |
| Type connu à la compilation | Génériques `<T: Trait>` |
| Besoin de changer d'implémentation à l'exécution | `Box<dyn Trait>` |
| Collections hétérogènes | `Vec<Box<dyn Trait>>` |
| Tests avec mocks | Les deux fonctionnent |

### Bonnes Pratiques

- Gardez vos traits **focalisés** sur un seul comportement (Single Responsibility)
- Utilisez des **méthodes par défaut** pour éviter la duplication
- Créez des **mock/stub** pour les tests
- Documentez les traits car ils définissent le contrat

## 11. Exercices

### Exercice 1 : Système de Stockage

Créez un trait `Storage` avec les méthodes :
- `save(&self, key: &str, value: &str) -> Result<(), String>`
- `load(&self, key: &str) -> Result<String, String>`
- `delete(&self, key: &str) -> Result<(), String>`

Implémentez-le pour :
- `FileStorage` (fichiers sur disque)
- `MemoryStorage` (HashMap en mémoire)
- `MockStorage` (pour les tests)

### Exercice 2 : Système de Log

Créez un trait `Logger` avec :
- `log(&self, level: LogLevel, message: &str)`
- `info(&self, message: &str)` (méthode par défaut)
- `error(&self, message: &str)` (méthode par défaut)

Implémentez pour :
- `ConsoleLogger`
- `FileLogger`
- `NullLogger` (ne fait rien, pour désactiver les logs)

### Exercice 3 : Système de Cache

Créez un trait `Cache<T>` générique avec :
- `get(&self, key: &str) -> Option<T>`
- `set(&mut self, key: &str, value: T, ttl_seconds: u64)`
- `invalidate(&mut self, key: &str)`

Implémentez avec différentes stratégies d'éviction.

## Conclusion

Vous maîtrisez maintenant :
- Le principe d'inversion de dépendance en Rust
- La création de traits comme abstractions
- L'injection de dépendances avec génériques et trait objects
- Les cas d'usage de chaque approche
- Comment faciliter les tests avec des mocks

L'inversion de dépendance est essentielle pour créer des applications modulaires, testables et maintenables.

---

**Prochaine leçon** : Gestion des erreurs avancée avec `Result`, `Option`, et l'opérateur `?`
