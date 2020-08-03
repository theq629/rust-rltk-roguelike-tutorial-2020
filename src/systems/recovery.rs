use specs::prelude::*;
use crate::{RunState, Turn, Player, Monster, Resting, Health, Stamina, Poise};

pub struct RecoverySystem {}

impl<'a> System<'a> for RecoverySystem {
    type SystemData = (
        ReadExpect<'a, RunState>,
        ReadExpect<'a, Turn>,
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Resting>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Stamina>,
        WriteStorage<'a, Poise>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (
            runstate,
            turn,
            entities,
            players,
            monsters,
            mut resting,
            mut health,
            mut stamina,
            mut poise
        ) = data;

        let mut to_stop_resting = Vec::new();

        for (entity, mut health, mut stamina, mut poise) in (&entities, &mut health, &mut stamina, &mut poise).join() {
            if *runstate == RunState::PlayerTurn {
                if let None = players.get(entity) {
                    continue;
                }
            } else {
                if let None = monsters.get(entity) {
                    continue;
                }
            }

            if let Some(_) = resting.get(entity) {
                if *turn % 5 == 0 {
                    health.health = i32::min(health.max_health, health.health + 1);
                }
                stamina.stamina = i32::min(stamina.max_stamina, stamina.stamina + 1);
                to_stop_resting.push(entity);
            } else {
                if *turn % 10 == 0 {
                    health.health = i32::min(health.max_health, health.health + 1);
                }
                if *turn % 5 == 0 {
                    stamina.stamina = i32::min(stamina.max_stamina, stamina.stamina + 1);
                    poise.poise = i32::min(poise.max_poise, poise.poise + 1);
                }
            }
        }

        for entity in to_stop_resting {
            resting.remove(entity);
        }
    }
}
