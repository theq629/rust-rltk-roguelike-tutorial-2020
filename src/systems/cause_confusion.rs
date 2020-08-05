use specs::prelude::*;
use crate::{gamelog::GameLog, text::capitalize, ItemUseInProgress, CausesConfusion, Confusion, Name};

pub struct CauseConfusionSystem {}

impl<'a> System<'a> for CauseConfusionSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, ItemUseInProgress>,
        ReadStorage<'a, CausesConfusion>,
        WriteStorage<'a, Confusion>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gamelog, entities, use_in_progress, causes_confusion, mut confused, names) = data;

        for (entity, useitem, name) in (&entities, &use_in_progress, &names).join() {
            let mut add_confusion = Vec::new();
            {
                if let Some(confusion) = causes_confusion.get(useitem.item) {
                    for mob in useitem.targets.iter() {
                        add_confusion.push((*mob, confusion.turns));
                        let mob_name = names.get(*mob).unwrap();
                        let item_name = names.get(useitem.item).unwrap();
                        gamelog.on(entity, &format!("{} {} {} on {}, confusing {}.", capitalize(&name.np), name.verb("uses", "use"), item_name.np, mob_name.np, mob_name.pronoun));
                    }
                }
            }
            for mob in add_confusion.iter() {
                confused.insert(mob.0, Confusion{ turns: mob.1 }).expect("Unable to insert status");
            }
        }
    }
}
