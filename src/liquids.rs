use serde::{Serialize, Deserialize};

#[derive(PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Liquid {
    BLOOD,
    OIL
}

impl Liquid {
    pub fn name(self) -> String {
        match self {
            Liquid::BLOOD => "blood".to_string(),
            Liquid::OIL => "oil".to_string()
        }
    }
}

impl Eq for Liquid {}
