use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonPlayer {
    pub name: String,
    pub id: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonEdge {
    pub winner_id: u8,
    pub loser_id: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonGraph {
    pub players: Vec<JsonPlayer>,
    pub edges: Vec<JsonEdge>,
}
