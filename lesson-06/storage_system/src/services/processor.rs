use std::vec;

use crate::{models::storage::Storage};

pub trait IProcessor {
    fn proceed(&mut self) -> Result<(), String>;
}

pub struct Processor<T: Storage> {
    storage:T,
}

impl<T: Storage> Processor<T> {
    pub fn new(storage: T) -> Self { Processor {
        storage,
    } }

    fn add_key(storage: &mut T, key: &str, value: &str) -> Result<String, String> {
        println!("Ajout valeur {} - {}", key, value);
        match storage.save(key, value) {
            Err(error) => Err(error),
            Ok(_) => Ok("Réussi".to_string()),
        }
    }

    fn show_value(memory: &T, key: &str) -> Result <String, String> {
        println!("Affichage de la valeur pour la clef '{}'", key);
        match memory.load(key) {
            Err(error) => Err(error),
            Ok(value) => Ok(format!("==> '{}'", value)),
        }
    }

    fn delete_value(memory: &mut T, key: &str) -> Result<String, String> {
        println!("Suppression de la valeur pour la clef '{}'", key);
        match memory.delete(key) {
            Err(error) => Err(error),
            Ok(_) => Ok("Réussi".to_string()),
        }
    }
}

impl<T: Storage> IProcessor for Processor<T> {
    fn proceed(&mut self) -> Result<(), String> {
        let mut results: Vec<Result<String, String>> = Vec::new();

        results.push(Self::add_key(&mut self.storage, "1", "1"));
        results.push(Self::add_key(&mut self.storage, "2", "2"));
        results.push(Self::add_key(&mut self.storage, "1", "2"));

        results.push(Self::show_value(&self.storage, "1"));
        results.push(Self::show_value(&self.storage, "2"));
        results.push(Self::show_value(&self.storage, "3"));

        results.push(Self::delete_value(&mut  self.storage, "1"));
        results.push(Self::delete_value(&mut self.storage, "3"));

        if results.into_iter().all(|res| res.is_ok()) {
            Ok(())
        } else {
            Err("Tout ne s'est pas bien déroulé !".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::storage::MockStorage, services::processor::{IProcessor, Processor}};

    #[test]
    fn first_test() {
        let storage = MockStorage::new(true);
        let mut processor = Processor::new(storage);
        let result = processor.proceed();
        
        assert!(result.is_ok());
    }
}
