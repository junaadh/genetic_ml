use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub fn random(no_food: usize, no_animals: usize, rng: &mut dyn RngCore) -> Self {
        Self {
            animals: (0..no_animals).map(|_| Animal::random(rng)).collect(),
            foods: (0..no_food).map(|_| Food::random(rng)).collect(),
        }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }
    pub fn food(&self) -> &[Food] {
        &self.foods
    }
}
