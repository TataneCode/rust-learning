use crate::{models::storage::Storage};

pub trait IProcessor {
    fn proceed(&mut self) -> bool;
}

pub struct Processor<T: Storage> {
    storage:T,
}

impl<T: Storage> Processor<T> {
    pub fn new(storage: T) -> Self { Processor {
        storage,
    } }

    fn add_key(storage: &mut T, key: &str, value: &str) {
        println!("Ajout valeur {} - {}", key, value);
        match storage.save(key, value) {
            Err(error) => println!("{}", error),
            Ok(_) => println!("Réussi"),
        }
    }

    fn show_value(memory: &T, key: &str) {
        println!("Affichage de la valeur pour la clef '{}'", key);
        match memory.load(key) {
            Err(error) => println!("{}", error),
            Ok(value) => println!("==> '{}'", value),
        }
    }

    fn delete_value(memory: &mut T, key: &str) {
        println!("Suppression de la valeur pour la clef '{}'", key);
        match memory.delete(key) {
            Err(error) => println!("{}", error),
            Ok(_) => println!("Réussi"),
        }
    }
}

impl<T: Storage> IProcessor for Processor<T> {
    fn proceed(&mut self) -> bool {
        Self::add_key(&mut self.storage, "1", "1");
        Self::add_key(&mut self.storage, "2", "2");
        Self::add_key(&mut self.storage, "1", "2");

        Self::show_value(&self.storage, "1");
        Self::show_value(&self.storage, "2");
        Self::show_value(&self.storage, "3");

        Self::delete_value(&mut  self.storage, "1");
        Self::delete_value(&mut self.storage, "3");

        return true;
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
        assert!(result);
    }
}
