use serde::{Serialize, Deserialize};

#[derive(PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Faction {
    PLAYER,
    ENEMIES
}
