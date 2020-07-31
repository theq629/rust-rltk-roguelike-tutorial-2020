use serde::{Serialize, Deserialize};
use rltk::Point;
use crate::{systems::effects::Effect};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Dance {
    JITTER,
    CIRCLE
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Step {
    pub direction: Point,
    pub effect: Option<Effect>
}

impl Dance {
    pub fn steps(&self) -> Vec<Step> {
        match self {
            Dance::JITTER => vec![
                    step(-1, 0),
                    step_with_effect(1, 0, Effect::AWESOMENESS { awe: 2, reason: "dancing".to_string(), range: 3 })
            ],
            Dance::CIRCLE => vec![
                    step(1, 0),
                    step(0, 1),
                    step(-1, 0),
                    step_with_effect(0, -1, Effect::AWESOMENESS { awe: 2, reason: "dancing".to_string(), range: 5 })
            ]
        }
    }
}

fn step(dx: i32, dy: i32) -> Step {
    Step {
        direction: Point::new(dx, dy),
        effect: None
    }
}

fn step_with_effect(dx: i32, dy: i32, effect: Effect) -> Step {
    Step {
        direction: Point::new(dx, dy),
        effect: Some(effect)
    }
}
