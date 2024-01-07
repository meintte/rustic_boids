use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) boids: Vec<Boid>,
    pub(crate) boundary_condition: BoundaryCondition,
}

impl World {
    pub(crate) fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        Self {
            boids: (0..config.world_boid_count)
                .map(|_| Boid::random(config, rng))
                .collect(),
            boundary_condition: config.world_boundary_condition,
        }
    }

    pub fn boids(&self) -> &[Boid] {
        &self.boids
    }
}
