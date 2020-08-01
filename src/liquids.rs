use serde::{Serialize, Deserialize};

#[derive(PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Liquid {
    BLOOD,
    OIL
}

impl Eq for Liquid {}
