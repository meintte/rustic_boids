use crate::*;

#[derive(Debug)]
pub struct Boid {
    pub(crate) position: na::Point2<f32>,
    pub(crate) velocity: na::Vector2<f32>,
    pub(crate) acceleration: na::Vector2<f32>,
}

impl Boid {
    pub fn new(position: na::Point2<f32>, velocity: na::Vector2<f32>) -> Self {
        Self {
            position,
            velocity,
            acceleration: na::Vector2::new(0.0, 0.0),
        }
    }

    pub fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        Self::new(
            rng.gen(),
            na::Vector2::new(
                rng.gen_range(config.boid_velocity_range.0..=config.boid_velocity_range.1),
                rng.gen_range(config.boid_velocity_range.2..=config.boid_velocity_range.3),
            ),
        )
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn velocity(&self) -> na::Vector2<f32> {
        self.velocity
    }

    pub fn update(&mut self, dt: f32) {
        // TODO REPALCE WITH TRAIT FROM BELOW
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
    }
}

pub trait ExplicitEuler {
    fn update(&mut self, dt: f32);
}

impl ExplicitEuler for Boid {
    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}

pub trait SemiImplicitEuler {
    fn update(&mut self, dt: f32);
}

impl SemiImplicitEuler for Boid {
    fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
    }
}

pub trait Verlet {
    fn update(&mut self, dt: f32);
}

impl Verlet for Boid {
    fn update(&mut self, dt: f32) {
        let old_position = self.position;
        self.position += self.velocity * dt + 0.5 * self.acceleration * dt * dt;
        self.velocity = (self.position - old_position) / dt;
    }
}

pub trait LeapFrog {
    fn update(&mut self, dt: f32);
}

impl LeapFrog for Boid {
    fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
    }
}
