use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use serde::Serialize;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());

        JsValue::from_serde(&world)
            .unwrap()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
    animals: Vec<Animal>,
    food: Vec<Food>
}

impl From<&sim::world::World> for World {
    fn from(world: &sim::world::World) -> Self {
        let animals = world
            .animals()
            .iter()
            .map(Animal::from)
            .collect();

        let food = world
            .food()
            .iter()
            .map(Food::from)
            .collect();

        Self { animals, food }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32
}

impl From<&sim::world::Animal> for Animal {
    fn from(animal: &sim::world::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32
}

impl From<&sim::world::Food> for Food {
    fn from(food: &sim::world::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
