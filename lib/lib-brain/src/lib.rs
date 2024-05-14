use rand::{thread_rng, Rng, RngCore};
use std::vec::Vec;

#[derive(Debug)]
pub struct NLTopology {
    pub neurons: usize,
}

impl NLTopology {
    pub fn new(neurons: usize) -> Self {
        Self { neurons }
    }

    pub fn from(layers: Vec<usize>) -> Vec<Self> {
        layers.iter().map(|&layer| Self::new(layer)).collect()
    }
}

#[macro_export]
macro_rules! network {
    ($($x:expr),+ $(,)?) => {{
        let layers = vec![$($x),*];
        $crate::NLTopology::from(layers)
    }};
}

#[derive(Debug)]
pub struct NeuralNetwork {
    pub(crate) layers: Vec<Layer>,
}

impl NeuralNetwork {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(rng: &mut dyn RngCore, layers: Vec<NLTopology>) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layer| Layer::random(rng, layer[0].neurons, layer[1].neurons))
            .collect();
        Self { layers }
    }
}

#[derive(Debug)]
pub(crate) struct Layer {
    pub(crate) neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, train_data: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&train_data))
            .collect()
    }

    fn random(rng: &mut dyn RngCore, input: usize, output: usize) -> Self {
        let neurons = (0..output).map(|_| Neuron::random(rng, input)).collect();
        Self { neurons }
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn propagate(&self, data: &[f32]) -> f32 {
        assert_eq!(self.weights.len(), data.len());

        let out = data
            .iter()
            .zip(&self.weights)
            .map(|(&input, &weight)| input * weight)
            .sum::<f32>();
        (self.bias + out).max(0.0)
    }

    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self { bias, weights }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
        );
    }

    #[test]
    fn propagate_neuron() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
        );
    }
}
