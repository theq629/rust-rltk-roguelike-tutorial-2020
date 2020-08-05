use specs::prelude::*;
use crate::{RunState, Health, Poise};

pub fn check_lose(ecs: &mut World) {
    let player = ecs.fetch::<Entity>();
    let health = ecs.read_storage::<Health>();
    let poise = ecs.read_storage::<Poise>();

    if let Some(health) = health.get(*player) {
        if health.health <= 0 {
            let mut runstate = ecs.write_resource::<RunState>();
            *runstate = RunState::GameOver {
                reason: "You died.".to_string()
            };
        }
    }

    if let Some(poise) = poise.get(*player) {
        if poise.poise <= 0 {
            let mut runstate = ecs.write_resource::<RunState>();
            *runstate = RunState::GameOver {
                reason: "You flee in shame.".to_string()
            };
        }
    }
}
