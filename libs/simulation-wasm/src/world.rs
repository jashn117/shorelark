use lib_simulation as sim;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) food: Vec<Food>
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
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) rotation: f32
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
    pub(crate) x: f32,
    pub(crate) y: f32
}

impl From<&sim::world::Food> for Food {
    fn from(food: &sim::world::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y
        }
    }
}
