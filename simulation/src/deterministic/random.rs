use std::ops::{Deref, Range};

use parking_lot::Mutex;
use rand::distributions::uniform::SampleUniform;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use std::sync::Arc;

/// A deterministic source of randomness, initialized by a seed
#[derive(Clone)]
pub struct DeterministicRandom {
    inner: Arc<Mutex<rand::rngs::SmallRng>>,
}

impl Default for DeterministicRandom {
    fn default() -> Self {
        Self::new()
    }
}

impl DeterministicRandom {
    pub fn new() -> Self {
        DeterministicRandom {
            inner: Arc::new(Mutex::new(rand::rngs::SmallRng::from_entropy())),
        }
    }
    pub fn new_with_seed(seed: u64) -> Self {
        DeterministicRandom {
            inner: Arc::new(Mutex::new(rand::rngs::SmallRng::seed_from_u64(seed))),
        }
    }
    pub fn random_between<T: SampleUniform + PartialOrd>(&mut self, range: Range<T>) -> T {
        let mut rng = self.inner.lock();
        (*rng).gen_range(range)
    }

    pub fn random_boolean(&mut self) -> bool {
        let mut rng = self.inner.lock();
        (*rng).gen()
    }

    pub fn random_01(&mut self) -> f32 {
        self.random_between(0_f32..1_f32)
    }
}

struct Inner {
    rng: SmallRng,
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
                    assert_eq!(
                        a.random_between(range.clone()),
                        b.random_between(range.clone())
                    );
                }
            }
        }
    }
}
