//! Deterministic randomness
use std::ops::Range;

use parking_lot::Mutex;
use rand::distributions::uniform::SampleUniform;
use rand::{Rng, SeedableRng};
use std::sync::Arc;

/// A source of randomness that can be seeded to become deterministic
#[derive(Clone, Debug)]
pub struct DeterministicRandom {
    inner: Arc<Mutex<rand::rngs::SmallRng>>,
}

impl DeterministicRandom {
    /// create a deterministic random given a seed
    pub fn new_with_seed(seed: u64) -> Self {
        DeterministicRandom {
            inner: Arc::new(Mutex::new(rand::rngs::SmallRng::seed_from_u64(seed))),
        }
    }
    /// generate a random value between the range
    pub fn random_between<T: SampleUniform + PartialOrd>(&mut self, range: Range<T>) -> T {
        let mut rng = self.inner.lock();
        (*rng).gen_range(range)
    }

    /// generate a random boolean given a probability
    pub fn random_boolean(&mut self, probability: f64) -> bool {
        let mut rng = self.inner.lock();
        (*rng).gen_bool(probability)
    }

    /// generate a float between 0 and 1
    pub fn random_01(&mut self) -> f32 {
        self.random_between(0_f32..1_f32)
    }
}

#[cfg(test)]
mod tests {
    use crate::deterministic::random::DeterministicRandom;

    #[test]
    fn deterministic_random() {
        for seed in 0..9999 {
            let mut a = DeterministicRandom::new_with_seed(seed);
            let mut b = DeterministicRandom::new_with_seed(seed);
            for range in vec![0.0..1.0, 0.0..42.0, 0.0..999.0] {
                for _ in 0..999 {
                    let first: f64 = a.random_between(range.clone());
                    let second: f64 = b.random_between(range.clone());
                    assert!((first - second).abs() < f64::EPSILON);
                    assert!((a.random_01() - b.random_01()).abs() < f64::EPSILON as f32);
                    assert_eq!(a.random_boolean(0.5), b.random_boolean(0.5));
                }
            }
        }
    }
}
