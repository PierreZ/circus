//! Inject failure with buggify
//! `buggify` allow you to cooperate with the simulator to inject failures.
//! It has the following rules:
//! 1. it only ever evaluates to true when run in simulation.
//! 1. The first time each `buggify` use is evaluated, it is either enabled or disabled for the entire simulation run.
//! 1. Enabled uses of `buggify` have a 5% chance of evaluating to true
//!
//! A good blogpost about buggify can be found [here](https://transactional.blog/simulation/buggify.html).
//! ```rust
//! use circus_buggify::{buggify_with_prob, enable_buggify, Buggifier};
//! use rand::rngs::SmallRng;
//! use rand::SeedableRng;
//!
//! // let's create a buggifier
//! let b = Buggifier::default();
//!
//! // enables buggify with a seed
//! b.enable_buggify(SmallRng::seed_from_u64(42));
//!
//! for i in 0..10 {
//!     // this block has a 0.05% chance to be run
//!     // which is iteration 8 for seed 42
//!     if b.buggify() {
//!         println!("buggified at iteration {}", i);
//!     }
//! }
//!
//! // buggify can also accept a probability
//! if b.buggify_with_prob(1.0) {
//!     println!("buggified with a 100% probability!");
//! }
//!
//! // you can also get a static buggifier that needs to be enabled
//! enable_buggify(SmallRng::seed_from_u64(42));
//! if buggify_with_prob(1.00) {
//!     println!("buggified with a 100% probability!");
//! }
//!```

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
use parking_lot::Mutex;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use rand::rngs::SmallRng;
use rand::Rng;
use std::ops::Deref;
use std::panic::Location;

/// Buggifier's definition
#[derive(Debug)]
pub struct Buggifier {
    buggified_lines: Mutex<HashMap<String, bool>>,
    random: Mutex<Option<SmallRng>>,
}

impl Buggifier {
    /// create a new Buggifier
    pub fn new(r: SmallRng) -> Self {
        Buggifier {
            buggified_lines: Mutex::new(HashMap::new()),
            random: Mutex::new(Some(r)),
        }
    }

    #[track_caller]
    /// `buggify` will returns true only once per execution with a probability of 0.05.
    pub fn buggify(&self) -> bool {
        let location = Location::caller();
        self.handle_buggify(format!("{}:{}", location.file(), location.line()), 0.05)
    }

    /// `buggify` version where you can choose the probability.
    pub fn buggify_with_prob(&self, probability: f64) -> bool {
        let location = Location::caller();
        self.handle_buggify(
            format!("{}:{}", location.file(), location.line()),
            probability,
        )
    }

    fn handle_buggify(&self, line: String, probability: f64) -> bool {
        let mut lock = self.random.lock();

        match (*lock).as_mut() {
            None => false,
            Some(deterministic_random) => {
                let mut already_buggified = self.buggified_lines.lock();
                if !already_buggified.contains_key(&line)
                    && deterministic_random.gen_bool(probability)
                {
                    already_buggified.insert(line, true);
                    return true;
                }
                false
            }
        }
    }

    /// checks if buggify is enabled
    pub fn is_buggify_enabled(&self) -> bool {
        self.random.lock().is_some()
    }

    /// enables buggify by giving a random source
    pub fn enable_buggify(&self, r: SmallRng) {
        let mut data = self.random.lock();
        *data = Some(r);
    }

    /// disable buggify
    pub fn disable_buggify(&self) {
        let mut data = self.random.lock();
        *data = None;
        let mut map = self.buggified_lines.lock();
        map.clear();
    }
}

impl Default for Buggifier {
    /// Create a new Buggifier
    fn default() -> Self {
        Buggifier {
            buggified_lines: Mutex::new(HashMap::new()),
            random: Mutex::new(None),
        }
    }
}

// static instance of buggifier
#[doc(hidden)]
static BUGGIFIER_INSTANCE: Lazy<Buggifier> = Lazy::new(Buggifier::default);

/// retrieves the static buggifier
pub fn buggifier() -> &'static Buggifier {
    BUGGIFIER_INSTANCE.deref()
}

#[track_caller]
/// `buggify` will returns true only once per execution with a probability of 0.05.
pub fn buggify() -> bool {
    let location = Location::caller();
    buggifier().handle_buggify(format!("{}:{}", location.file(), location.line()), 0.05)
}

#[track_caller]
/// `buggify` version where you can choose the probability.
pub fn buggify_with_prob(probability: f64) -> bool {
    let location = Location::caller();
    buggifier().handle_buggify(
        format!("{}:{}", location.file(), location.line()),
        probability,
    )
}

/// checks if buggify is enabled
pub fn is_buggify_enabled() -> bool {
    buggifier().is_buggify_enabled()
}

/// enables buggify by giving a random source
pub fn enable_buggify(r: SmallRng) {
    buggifier().enable_buggify(r)
}

/// disable buggify
pub fn disable_buggify() {
    buggifier().disable_buggify()
}

#[cfg(test)]
mod tests {
    use crate::{
        buggifier, buggify, buggify_with_prob, disable_buggify, enable_buggify, is_buggify_enabled,
        Buggifier,
    };
    use rand::rngs::SmallRng;
    use rand::SeedableRng;
    use tracing::Level;

    #[test]
    fn test_buggifier() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_test_writer()
            .try_init();

        let b = Buggifier::default();
        assert!(!b.is_buggify_enabled());
        assert!(!b.buggify_with_prob(1.0), "should not buggified");

        {
            let data = b.random.lock();
            assert!((*data).is_none());

            let map = b.buggified_lines.lock();
            assert!((*map).is_empty());
        }

        let random = SmallRng::seed_from_u64(42);
        b.enable_buggify(random);
        assert!(b.is_buggify_enabled(), "should be activated");

        for i in 0..100 {
            let result = i == 8;
            assert_eq!(
                b.buggify(),
                result,
                "iteration {} should have been {}",
                i,
                result
            );
        }

        {
            let data = b.random.lock();
            assert!((*data).is_some());

            let map = b.buggified_lines.lock();
            assert_eq!((*map).len(), 1);
            for key in (*map).keys() {
                assert!(key.starts_with(&file!().to_string()));
            }
            for value in (*map).values() {
                assert!(value);
            }
        }

        b.disable_buggify();
        assert!(!b.buggify_with_prob(1.0), "should not buggified");
    }

    #[test]
    fn test_static_buggify() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_test_writer()
            .try_init();

        // reset any previous test
        disable_buggify();

        assert!(!is_buggify_enabled());
        assert!(!buggify_with_prob(1.0), "should not buggified");

        enable_buggify(SmallRng::seed_from_u64(42));
        assert!(is_buggify_enabled(), "should be activated");

        for i in 0..100 {
            let result = i == 8;
            assert_eq!(
                buggify(),
                result,
                "iteration {} should have been {}",
                i,
                result
            );
        }
        {
            let data = buggifier().random.lock();
            assert!((*data).is_some());

            let map = buggifier().buggified_lines.lock();
            assert_eq!((*map).len(), 1);
            for key in (*map).keys() {
                assert!(key.starts_with(&file!().to_string()));
            }
            for value in (*map).values() {
                assert!(value);
            }
        }

        buggifier().disable_buggify();
        assert!(!buggifier().buggify_with_prob(1.0), "should not buggified");
    }
}
