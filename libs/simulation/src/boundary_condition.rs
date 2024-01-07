use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BoundaryCondition {
    Periodic,
    Reflect,
}

impl Default for BoundaryCondition {
    fn default() -> Self {
        Self::Periodic
    }
}

// TODO: Generalize such that any object with a position and velocity can be used (not just boids)
impl BoundaryCondition {
    pub fn apply(&self, boid: &mut Boid) {
        match self {
            Self::Periodic => {
                boid.position.x = na::wrap(boid.position.x, 0.0, 1.0);
                boid.position.y = na::wrap(boid.position.y, 0.0, 1.0);
            }
            Self::Reflect => {
                if boid.position.x < 0.0 {
                    boid.position.x = 0.0;
                    boid.velocity.x = -boid.velocity.x;
                } else if boid.position.x > 1.0 {
                    boid.position.x = 1.0;
                    boid.velocity.x = -boid.velocity.x;
                }
                if boid.position.y < 0.0 {
                    boid.position.y = 0.0;
                    boid.velocity.y = -boid.velocity.y;
                } else if boid.position.y > 1.0 {
                    boid.position.y = 1.0;
                    boid.velocity.y = -boid.velocity.y;
                }
            }
        }
    }
}
