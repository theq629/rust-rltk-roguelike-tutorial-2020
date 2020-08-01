use specs::prelude::*;
use rltk::{Point};
use crate::{CombatStats, SufferDamage, Player, Name, gamelog::{GameLog, capitalize}, RunState, Position, Map, liquids::Liquid};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (WriteStorage<'a, CombatStats>,
                       WriteStorage<'a, SufferDamage>,
                       ReadStorage<'a, Position>,
                       WriteExpect<'a, Map>,
                       Entities<'a>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut stats, mut damage, positions, mut map, entities) = data;

        for (entity, mut stats, damage) in (&entities, &mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
            if let Some(pos) = positions.get(entity) {
                let idx = map.xy_idx(pos.x, pos.y);
                map.stains[idx].insert(Liquid::BLOOD);
            }
        }

        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let names = ecs.read_storage::<Name>();
        let positions = ecs.read_storage::<Position>();
        let entities = ecs.entities();
        let mut gamelog = ecs.write_resource::<GameLog>();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                let player = players.get(entity);
                match player {
                    None => {
                        if let Some(victim_name) = names.get(entity) {
                            if let Some(pos) = positions.get(entity) {
                                gamelog.at(Point::new(pos.x, pos.y), &format!("{} {} dead", capitalize(&victim_name.np), victim_name.verb("is", "are")));
                            }
                        }
                        dead.push(entity)
                    }
                    Some(_) => {
                        let mut runstate = ecs.write_resource::<RunState>();
                        *runstate = RunState::GameOver;
                    }
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete")
    }
}
