use crate::{models::storage::{FileStorage, MemoryStorage}, services::processor::{IProcessor, Processor}};

mod models;
mod services;

fn main() {
    println!("Hello, tests for traits !");

    let mut memory_processor = Processor::new(MemoryStorage::new());
    let mut file_processor = Processor::new(FileStorage::new(".".to_string()));

    println!("*** Proceed with memory ***");
    memory_processor.proceed();

    println!("*** Proceed with file ***");
    file_processor.proceed();
}
