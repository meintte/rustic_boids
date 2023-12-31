pub use self::{boid::*, world::*};

mod boid;
mod world;

use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Self {
        let config: sim::Config = serde_wasm_bindgen::from_value(config).unwrap();

        let mut rng = thread_rng();
        let sim = sim::Simulation::random(config, &mut rng);

        Self { rng, sim }
    }

    pub fn default_config() -> JsValue {
        serde_wasm_bindgen::to_value(&sim::Config::default()).unwrap()
    }

    pub fn config(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.sim.config()).unwrap()
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        serde_wasm_bindgen::to_value(&world).unwrap()
    }

    pub fn step(&mut self, dt: f32) {
        self.sim.step(dt);
    }
}
