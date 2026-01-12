mod auteur;
mod livre;
mod menu;
mod persistence;

pub use auteur::*;
pub use livre::*;
pub use menu::*;
pub use persistence::*;

use std::sync::{Arc, Mutex};
use crate::services::Bibliotheque;

pub type SharedBibliotheque = Arc<Mutex<Bibliotheque>>;
