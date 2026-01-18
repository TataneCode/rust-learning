use crate::models::storage::{MemoryStorage, Storage};

mod models;

fn main() {
    println!("Hello, tests for traits !");

    let memory = MemoryStorage::new();

    proceed(memory);
}

fn proceed(mut memory: MemoryStorage) {
    add_key(&mut memory, "1", "1");
    add_key(&mut memory, "2", "2");
    add_key(&mut memory, "1", "2");

    show_value(&memory, "1");
    show_value(&memory, "2");
    show_value(&memory, "3");

    delete_value(&mut memory, "1");
    delete_value(&mut memory, "3");
}

fn add_key<T: Storage>(memory: &mut T, key: &str, value: &str) {
    println!("Ajout valeur {} - {}", key, value);
    match memory.save(key, value) {
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
