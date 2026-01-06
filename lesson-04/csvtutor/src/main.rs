//tutorial-setup-01.rs
use config::Config;
use serde::Deserialize;
use std::fs::File;

// Define a struct to hold our configuration
//
// The #[derive(...)] attribute automatically implements traits for our struct.
// Think of traits like interfaces in other languages - they define behavior.
//
// #[derive(Debug, Deserialize)] means:
// - Debug: Automatically implements the Debug trait, which allows us to print
//   the struct using {:?} or {:#?} format specifiers. Very useful for debugging.
// - Deserialize: Automatically implements the Deserialize trait from serde,
//   which allows this struct to be deserialized (converted) from formats like
//   TOML, JSON, YAML, etc. The serde library generates code that knows how to
//   read configuration data and populate our struct fields.
//
// Without #[derive], we would need to manually implement these traits, which
// would be tedious and error-prone. The derive macro generates the code for us.
#[derive(Debug, Deserialize)]
struct AppConfig {
    app: AppSettings,
}

// Same derive attributes here - we need Debug and Deserialize on nested structs too.
// When deserializing AppConfig, serde also needs to deserialize the nested AppSettings.
#[derive(Debug, Deserialize)]
struct AppSettings {
    csv_file_path: String,
}

// The `main` function is where your program starts executing.
fn main() {
    // Load configuration from config.toml
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("Failed to load configuration");

    // The Deserialize trait (derived above) allows us to convert the config
    // file into our AppConfig struct automatically.
    let app_config: AppConfig = settings
        .try_deserialize()
        .expect("Failed to deserialize configuration");

    // Example of Debug trait in action - we can print the entire config struct:
    println!("Loaded config: {:#?}", app_config);
    // The {:#?} formatter uses the Debug trait to pretty-print the struct

    println!("Reading CSV from: {}", app_config.app.csv_file_path);

    // Open the file specified in the configuration
    let file = File::open(&app_config.app.csv_file_path)
        .expect("Failed to open CSV file");

    // Create a CSV parser that reads data from the file.
    let mut rdr = csv::Reader::from_reader(file);

    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        // Print a debug version of the record.
        println!("{:?}", record);
    }
}

