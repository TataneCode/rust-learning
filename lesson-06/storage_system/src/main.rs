use crate::models::storage::{FileStorage, MemoryStorage, Storage};

mod models;

fn main() {
    println!("Hello, tests for traits !");

    let memory = MemoryStorage::new();
    let file = FileStorage::new(".".to_string());

    println!("*** Proceed with memory ***");
    proceed(memory);

    println!("*** Proceed with file ***");
    proceed(file);
}

fn proceed<T: Storage>(mut storage: T) {
    add_key(&mut storage, "1", "1");
    add_key(&mut storage, "2", "2");
    add_key(&mut storage, "1", "2");

    show_value(&storage, "1");
    show_value(&storage, "2");
    show_value(&storage, "3");

    delete_value(&mut storage, "1");
    delete_value(&mut storage, "3");
}

fn add_key<T: Storage>(storage: &mut T, key: &str, value: &str) {
    println!("Ajout valeur {} - {}", key, value);
    match storage.save(key, value) {
        Err(error) => println!("{}", error),
        Ok(_) => println!("Réussi"),
    }
}

fn show_value<T: Storage>(memory: &T, key: &str) {
    println!("Affichage de la valeur pour la clef '{}'", key);
    match memory.load(key) {
        Err(error) => println!("{}", error),
        Ok(value) => println!("==> '{}'", value),
    }
}

fn delete_value<T: Storage>(memory: &mut T, key: &str) {
    println!("Suppression de la valeur pour la clef '{}'", key);
    match memory.delete(key) {
        Err(error) => println!("{}", error),
        Ok(_) => println!("Réussi"),
    }
}
