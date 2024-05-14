use std::ops::Index;

use rand::{seq::SliceRandom, Rng, RngCore};

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosomes: Chromosome) -> Self;
}

pub struct Genetics<S> {
    selection: S,
    crossover: Box<dyn CrossOver>,
    mutation: Box<dyn Mutation>,
}

impl<S: Selection> Genetics<S> {
    pub fn new(
        selection: S,
        crossover: impl CrossOver + 'static,
        mutation: impl Mutation + 'static,
    ) -> Self {
        Self {
            selection,
            crossover: Box::new(crossover),
            mutation: Box::new(mutation),
        }
    }

    pub fn evolve<I: Individual>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection.select(rng, population).chromosome();
                let parent_b = self.selection.select(rng, population).chromosome();

                let mut child = self.crossover.crossover(rng, parent_a, parent_b);
                self.mutation.mutate(rng, &mut child);

                I::create(child)
            })
            .collect()
    }
}

pub trait Selection {
    fn select<'a, I: Individual>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I;
}

// implement linear rank selection
// pub struct LinearRankSelection;

pub struct FitnessProportionateSelection;

impl Selection for FitnessProportionateSelection {
    fn select<'a, I: Individual>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Empty population bruv")
    }
}

#[derive(Debug, Clone)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    // making clippy happy
    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

pub trait CrossOver {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parents_a: &Chromosome,
        parents_b: &Chromosome,
    ) -> Chromosome;
}

#[derive(Debug, Clone)]
pub struct UniformCrossOver;

impl CrossOver for UniformCrossOver {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parents_a: &Chromosome,
        parents_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parents_a.len(), parents_b.len());

        parents_a
            .iter()
            .zip(parents_b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

pub trait Mutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

pub struct GaussianMutation {
    /// chance of mutation
    /// range: 0.0 - 1.0
    /// 0.5 - half of all the genes will be mutated
    /// 1.0 all of the genes will be mutated
    chance: f32,
    /// degree of mutation
    /// range 0.0 - 3.0
    /// 0.0 - none of selected genes will be mutated
    /// 3,0 - touched genes woild be mutated +/- by a factor of 3
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        Self { chance, coeff }
    }
}

impl Mutation for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        child.iter_mut().for_each(|gene| {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
            if rng.gen_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        })
    }
}
