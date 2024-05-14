use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(40, 40, &mut rng);
        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,

    #[wasm_bindgen(getter_with_clone)]
    pub foods: Vec<Food>,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::World> for World {
    fn from(value: &sim::World) -> Self {
        Self {
            animals: value.animals().iter().map(Animal::from).collect(),
            foods: value.food().iter().map(Food::from).collect(),
        }
    }
}

impl From<&sim::Animal> for Animal {
    fn from(value: &sim::Animal) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
            rotation: value.rotation().angle(),
        }
    }
}

impl From<&sim::Food> for Food {
    fn from(value: &sim::Food) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
        }
    }
}
