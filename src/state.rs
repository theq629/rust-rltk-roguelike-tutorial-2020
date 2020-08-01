use specs::prelude::*;
use rltk::{Point};
use super::{systems, gamelog, spawner, stuff};
use super::map::{Map};
use super::components::*;

pub struct State {
    pub ecs: World,
    dispatcher: Box<dyn systems::UnifiedDispatcher + 'static>
}

impl State {
    pub fn new() -> Self {
        State {
            ecs: World::new(),
            dispatcher: systems::build()
        }
    }

    pub fn run_systems(&mut self) {
        self.dispatcher.run_now(&mut self.ecs);
        self.ecs.maintain();
    }

    pub fn setup_world(&mut self) {
        let map;
        {
            let mut rng = self.ecs.write_resource::<rltk::RandomNumberGenerator>();
            map = Map::new(1, &mut rng);
        }

        let (player_x, player_y) = map.rooms[0].centre();
        let player_entity = stuff::player(&mut self.ecs, player_x, player_y);
        self.ecs.insert(Point::new(player_x, player_y));
        self.ecs.insert(player_entity);

        for room in map.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.ecs, room, 1);
        }

        self.ecs.insert(map);
    }

    pub fn goto_next_level(&mut self) {
        let to_delete = self.entities_to_remove_on_level_change();
        for target in to_delete {
            self.ecs.delete_entity(target).expect("Unable to delete entity");
        }

        let worldmap;
        let current_depth;
        {
            let mut rng = self.ecs.write_resource::<rltk::RandomNumberGenerator>();
            let mut worldmap_resource = self.ecs.write_resource::<Map>();
            current_depth = worldmap_resource.depth;
            *worldmap_resource = Map::new(current_depth + 1, &mut rng);
            worldmap = worldmap_resource.clone();
        }

        for room in worldmap.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.ecs, room, current_depth+1);
        }

        let (player_x, player_y) = worldmap.rooms[0].centre();
        let mut player_position = self.ecs.write_resource::<Point>();
        *player_position = Point::new(player_x, player_y);
        let mut position_components = self.ecs.write_storage::<Position>();
        let player_entity = self.ecs.fetch::<Entity>();
        let player_pos_comp = position_components.get_mut(*player_entity);
        if let Some(player_pos_comp) = player_pos_comp {
            player_pos_comp.x = player_x;
            player_pos_comp.y = player_y;
        }

        let mut viewshed_components = self.ecs.write_storage::<Viewshed>();
        let vs = viewshed_components.get_mut(*player_entity);
        if let Some(vs) = vs {
            vs.dirty = true;
        }

        let mut player_log = self.ecs.fetch_mut::<gamelog::PlayerLog>();
        player_log.insert(&"You descend to the next level, and take a moment to heal.");
        let mut player_health_store = self.ecs.write_storage::<CombatStats>();
        let player_health = player_health_store.get_mut(*player_entity);
        if let Some(player_health) = player_health {
            player_health.hp = i32::max(player_health.hp, player_health.max_hp / 2);
        }
    }

    pub fn game_over_cleanup(&mut self) {
        let mut to_delete = Vec::new();
        for e in self.ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            self.ecs.delete_entity(*del).expect("Deletion failed");
        }

        let worldmap;
        {
            let mut rng = self.ecs.write_resource::<rltk::RandomNumberGenerator>();
            let mut worldmap_resource = self.ecs.write_resource::<Map>();
            *worldmap_resource = Map::new(1, &mut rng);
            worldmap = worldmap_resource.clone();
        }

        for room in worldmap.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.ecs, room, 1);
        }

        let (player_x, player_y) = worldmap.rooms[0].centre();
        let player_entity = stuff::player(&mut self.ecs, player_x, player_y);
        let mut player_position = self.ecs.write_resource::<Point>();
        *player_position = Point::new(player_x, player_y);
        let mut position_components = self.ecs.write_storage::<Position>();
        let mut player_entity_writer = self.ecs.write_resource::<Entity>();
        *player_entity_writer = player_entity;
        let player_pos_comp = position_components.get_mut(player_entity);
        if let Some(player_pos_comp) = player_pos_comp {
            player_pos_comp.x = player_x;
            player_pos_comp.y = player_y;
        }

        let mut viewshed_components = self.ecs.write_storage::<Viewshed>();
        let vs = viewshed_components.get_mut(player_entity);
        if let Some(vs) = vs {
            vs.dirty = true;
        }
    }

    fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity> {
        let entities = self.ecs.entities();
        let player = self.ecs.read_storage::<Player>();
        let backpack = self.ecs.read_storage::<InBackpack>();
        let player_entity = self.ecs.fetch::<Entity>();
        let equipped = self.ecs.read_storage::<Equipped>();

        let mut to_delete: Vec<Entity> = Vec::new();
        for entity in entities.join() {
            let mut should_delete = true;
            if let Some(_) = player.get(entity) {
                should_delete = false;
            }
            if let Some(bp) = backpack.get(entity) {
                if bp.owner == *player_entity {
                    should_delete = false;
                }
            }
            if let Some(eq) = equipped.get(entity) {
                if eq.owner == *player_entity {
                    should_delete = true;
                }
            }
            if should_delete {
                to_delete.push(entity);
            }
        }

        to_delete
    }
}
