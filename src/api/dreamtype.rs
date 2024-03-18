use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum DreamType {
    Normal,
    Lucid,
    Nightmare
}
