use specs::prelude::*;
use rltk::{Point};
use crate::{Health, SufferDamage, Name, gamelog::GameLog, text::capitalize, Position, Map, liquids::Liquid, Stamina};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (WriteStorage<'a, Health>,
                       WriteStorage<'a, Stamina>,
                       WriteStorage<'a, SufferDamage>,
                       ReadStorage<'a, Position>,
                       WriteExpect<'a, Map>,
                       Entities<'a>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut health, mut stamina, mut damage, positions, mut map, entities) = data;

        for (entity, mut health, mut stamina, damage) in (&entities, &mut health, &mut stamina, &damage).join() {
            health.health -= damage.amount.iter().sum::<i32>();
            stamina.stamina = i32::max(0, stamina.stamina - 1);
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
        let health = ecs.read_storage::<Health>();
        let names = ecs.read_storage::<Name>();
        let positions = ecs.read_storage::<Position>();
        let entities = ecs.entities();
        let mut gamelog = ecs.write_resource::<GameLog>();
        for (entity, health) in (&entities, &health).join() {
            if health.health < 1 {
                if let Some(victim_name) = names.get(entity) {
                    if let Some(pos) = positions.get(entity) {
                        gamelog.at(Point::new(pos.x, pos.y), &format!("{} {} dead", capitalize(&victim_name.np), victim_name.verb("is", "are")));
                    }
                }
                dead.push(entity)
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete")
    }
}
