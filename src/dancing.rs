use serde::{Serialize, Deserialize};
use rltk::Point;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Step {
    pub direction: Point
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Dance {
    pub steps: Vec<Step>
}

pub fn jiggle() -> Dance {
    Dance {
        steps: vec![
            Step { direction: Point::new(-1, 0) },
            Step { direction: Point::new(1, 0) },
        ]
    }
}

pub fn circle() -> Dance {
    Dance {
        steps: vec![
            Step { direction: Point::new(1, 0) },
            Step { direction: Point::new(0, 1) },
            Step { direction: Point::new(-1, 0) },
            Step { direction: Point::new(0, -1) },
        ]
    }
}
