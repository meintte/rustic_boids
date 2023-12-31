pub use self::{boid::*, config::*, world::*};

mod boid;
mod config;
mod world;

use nalgebra as na;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

pub struct Simulation {
    config: Config,
    world: World,
    time: f32,
}

impl Simulation {
    pub fn random(config: Config, rng: &mut dyn RngCore) -> Self {
        let world = World::random(&config, rng);
        Self {
            config,
            world,
            time: 0.0,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, dt: f32) {
        self.time += dt;
        self.world.boids.iter_mut().for_each(|b| b.update(dt));

        // periodic boundaries
        self.world.boids.iter_mut().for_each(|boid| {
            let p = boid.position();
            boid.position.x = na::wrap(p.x, 0.0, 1.0);
            boid.position.y = na::wrap(p.y, 0.0, 1.0);
        });
    }
}
