use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub world_boid_count: u32,
    pub world_boundary_condition: BoundaryCondition,

    pub boid_velocity_range: (f32, f32, f32, f32),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            world_boid_count: 20,
            world_boundary_condition: BoundaryCondition::Reflect,
            boid_velocity_range: (-0.1, 0.1, -0.1, 0.1),
        }
    }
}
