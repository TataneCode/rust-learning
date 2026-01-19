use std::{collections::HashMap, fs};

/// Trait storage
pub trait Storage {
    fn save(&mut self, key: &str, value: &str) -> Result<(), String>;
    fn load(&self, key: &str) -> Result<String, String>;
    fn delete(&mut self, key: &str) -> Result<(), String>;
}

pub struct MemoryStorage {
    map: HashMap<String, String>,
}

pub struct FileStorage {
    base_directory: String,
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            map: HashMap::new(),
        }
    }
}

impl FileStorage {
    pub fn new(directory: String) -> Self {
        FileStorage {
            base_directory: directory,
        }
    }

    fn get_path(&self, key: &str) -> String {
        format!("{}/{}.txt", self.base_directory, key)
    }
}

impl Storage for MemoryStorage {
    fn save(&mut self, key: &str, value: &str) -> Result<(), String> {
        if self.map.contains_key(key) {
            Err(format!("Clé '{}' existe déjà", key))
        } else {
            self.map.insert(key.to_string(), value.to_string());
            Ok(())
        }
    }

    fn load(&self, key: &str) -> Result<String, String> {
        match self.map.get(key) {
            Some(value) => Ok(value.to_string()),
            None => Err(format!("Clef '{}' non trouvée !", key)),
        }
    }

    fn delete(&mut self, key: &str) -> Result<(), String> {
        match self.map.remove(key) {
            Some(_) => Ok(()),
            None => Err(format!("Clef '{}' non trouvée !", key)),
        }
    }
}

impl Storage for FileStorage {
    fn save(&mut self, key: &str, value: &str) -> Result<(), String> {
        let path = self.get_path(key);
        fs::write(&path, value).map_err(|error| format!("Sauvegarde en erreur : {}", error))
    }

    fn load(&self, key: &str) -> Result<String, String> {
        let path = self.get_path(key);
        fs::read_to_string(path).map_err(|error| format!("Impossible de lire : {}", error))
    }

    fn delete(&mut self, key: &str) -> Result<(), String> {
        let path = self.get_path(key);
        fs::remove_file(path).map_err(|error| format!("Impossible de supprimer : {}", error))
    }
}
