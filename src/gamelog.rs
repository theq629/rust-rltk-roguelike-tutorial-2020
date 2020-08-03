use specs::prelude::{Entity};
use serde::{Serialize, Deserialize};
use rltk::{Point};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerLog {
    pub entries: Vec<String>
}

pub struct GameLog {
    pub entries: Vec<Item>
}

pub enum Scope {
    GLOBAL,
    AT { at: Point },
    ON { on: Entity }
}

pub struct Item {
    pub scope: Scope,
    pub message: String
}

impl PlayerLog {
    pub fn new() -> Self {
        PlayerLog {
            entries: Vec::new()
        }
    }

    pub fn insert<S: ToString>(&mut self, message: &S) {
        self.entries.push(message.to_string());
    }
}

impl GameLog {
    pub fn new() -> Self {
        GameLog {
            entries: Vec::new()
        }
    }

    pub fn global<S: ToString>(&mut self, message: &S) {
        self.entries.push(Item {
            scope: Scope::GLOBAL,
            message: message.to_string()
        });
    }

    pub fn at<S: ToString>(&mut self, at: Point, message: &S) {
        self.entries.push(Item {
            scope: Scope::AT { at: at },
            message: message.to_string()
        });
    }

    pub fn on<S: ToString>(&mut self, on: Entity, message: &S) {
        self.entries.push(Item {
            scope: Scope::ON { on: on },
            message: message.to_string()
        });
    }
}
