use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

mod world;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(
        generation_length: usize,
        animals: usize,
        foods: usize,
    ) -> Self
    {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(
            &mut rng,
            generation_length,
            animals,
            foods,
        );

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = world::World::from(self.sim.world());

        JsValue::from_serde(&world)
            .unwrap()
    }

    pub fn step(&mut self) {
        self.sim
            .step(&mut self.rng);
    }

    pub fn fast_fwd(&mut self) {
        self.sim
            .fast_fwd_generation(&mut self.rng);
    }
}
