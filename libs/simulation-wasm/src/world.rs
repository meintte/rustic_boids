use crate::*;

#[derive(Debug, Clone, Serialize)]
pub struct World {
    pub circles: Vec<Boid>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let circles = world.boids().iter().map(Boid::from).collect();
        Self { circles }
    }
}
