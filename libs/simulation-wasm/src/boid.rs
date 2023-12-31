use crate::*;

#[derive(Debug, Clone, Serialize)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Boid> for Boid {
    fn from(boid: &sim::Boid) -> Self {
        let p = boid.position();
        Self { x: p.x, y: p.y }
    }
}
