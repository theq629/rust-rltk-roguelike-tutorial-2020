use specs::prelude::*;
use rltk::{RGB};
use crate::{Rltk, ParticleLifetime, Position, Renderable};

pub struct ParticleSpawnSystem {}

struct ParticleRequest {
    x: i32,
    y: i32,
    fg: RGB,
    glyph: rltk::FontCharType,
    lifetime: f32
}

pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>
}

pub fn cull_dead_particles(ecs: &mut World, ctx: &Rltk) {
    let mut dead_particles: Vec<Entity> = Vec::new();
    {
        let mut particles = ecs.write_storage::<ParticleLifetime>();
        let entities = ecs.entities();
        for (entity, mut particle) in (&entities, &mut particles).join() {
            particle.lifetime_ms -= ctx.frame_time_ms;
            if particle.lifetime_ms < 0.0 {
                dead_particles.push(entity);
            }
        }
    }
    for dead in dead_particles.iter() {
        ecs.delete_entity(*dead).expect("Particle will not die");
    }
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder{ requests: Vec::new() }
    }

    pub fn request(&mut self, x: i32, y: i32, fg: RGB, glyph: rltk::FontCharType, lifetime: f32) {
        self.requests.push(
            ParticleRequest{ x, y, fg, glyph, lifetime }
        )
    }
}

impl<'a> System<'a> for ParticleSpawnSystem {
    type SystemData = (Entities<'a>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, Renderable>,
                       WriteStorage<'a, ParticleLifetime>,
                       WriteExpect<'a, ParticleBuilder>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut positions, mut renderables, mut particles, mut particle_builder) = data;
        for new_particle in particle_builder.requests.iter() {
            let p = entities.create();
            positions.insert(p, Position{ x: new_particle.x, y: new_particle.y }).expect("Unable to insert position");
            renderables.insert(p, Renderable{ fg: new_particle.fg, glyph: new_particle.glyph, render_order: 0 }).expect("Unable to insert renderable");
            particles.insert(p, ParticleLifetime{ lifetime_ms: new_particle.lifetime }).expect("Unable to insert lifetime");
        }
        particle_builder.requests.clear();
    }
}
