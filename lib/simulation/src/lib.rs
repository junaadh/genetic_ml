mod animal;
mod eye;
mod food;
mod world;

pub use self::{animal::*, eye::*, food::*, world::*};
use lib_brain as brain;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(no_food: usize, no_animal: usize, rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(no_food, no_animal, rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    fn process_movement(&mut self) {
        self.world.animals.iter_mut().for_each(|animal| {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        })
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        self.world.animals.iter().for_each(|animal| {
            self.world.foods.iter_mut().for_each(|food| {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            })
        })
    }

    fn process_brains(&mut self) {
        self.world.animals.iter_mut().for_each(|animal| {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.brain.propagate(vision);
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        })
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movement();
    }
}
