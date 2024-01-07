pub use self::{boid::*, boundary_condition::*, config::*, world::*};

mod boid;
mod boundary_condition;
mod config;
mod world;

use nalgebra as na;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

pub struct Simulation {
    config: Config,
    world: World,
    running: bool,
    time: f32,
}

impl Simulation {
    pub fn random(config: Config, rng: &mut dyn RngCore) -> Self {
        let world = World::random(&config, rng);
        Self {
            config,
            world,
            running: true,
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
        if !self.running {
            return;
        }

        self.time += dt;
        self.world.boids.iter_mut().for_each(|b| {
            b.update(dt);
            self.world.boundary_condition.apply(b);
        });
    }
}
