use serde::{Serialize, Deserialize};
use web_sys::window;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum OpponentType {
    Human,
    Computer,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Opponent {
    pub name: String,
    pub id: String,
    pub opponent_type: OpponentType,
}

impl Opponent {
    pub fn new(name: String, opponent_type: OpponentType) -> Self {
        Opponent {
            id: format!("{}_{}", 
                match opponent_type {
                    OpponentType::Human => "human",
                    OpponentType::Computer => "cpu",
                },
                name.to_lowercase().replace(" ", "_")
            ),
            name,
            opponent_type,
        }
    }
}

pub fn delete_opponent(id: &str) -> Result<(), serde_json::Error> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let mut saved_opponents = load_opponents().unwrap_or_default();
    saved_opponents.retain(|o| o.id != id);
    let json = serde_json::to_string(&saved_opponents)?;
    storage.set_item("saved_opponents", &json).unwrap();
    Ok(())
}

pub fn save_opponent(opponent: Opponent) -> Result<Vec<Opponent>, serde_json::Error> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    
    // Load existing opponents first
    let mut saved_opponents = load_opponents().unwrap_or_default();
    
    // Only add if not already present
    if !saved_opponents.iter().any(|o| o.id == opponent.id) {
        saved_opponents.push(opponent);
        let json = serde_json::to_string(&saved_opponents)?;
        storage.set_item("saved_opponents", &json).unwrap();
    }

    Ok(saved_opponents)
}

pub fn load_opponents() -> Option<Vec<Opponent>> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let data = storage.get_item("saved_opponents").ok()??;
    serde_json::from_str(&data).ok()
}