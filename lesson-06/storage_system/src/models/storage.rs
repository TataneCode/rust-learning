use std::collections::HashMap;

pub trait Storage {
    fn save(&mut self, key: &str, value: &str) -> Result<(), String>;
    fn load(&self, key: &str) -> Result<String, String>;
    fn delete(&mut self, key: &str) -> Result<(), String>;
}

pub struct MemoryStorage {
    map: HashMap<String, String>,
}

pub struct  FileStorage {

}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            map: HashMap::new(),
        }
    }
}

impl FileStorage {
    pub fn new() -> Self {
        FileStorage {}
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
        todo!()
    }

    fn load(&self, key: &str) -> Result<String, String> {
        todo!()
    }

    fn delete(&mut self, key: &str) -> Result<(), String> {
        todo!()
    }
}
