//! Inject failure with buggify
use crate::deterministic::random::Random;
use parking_lot::Mutex;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use std::ops::Deref;
use std::panic::Location;

/// Buggifier is providing the buggify methods.
/// `buggify` allow you to cooperate with the simulator to inject failures.
/// It has the following rules:
/// 1. it only ever evaluates to true when run in simulation.
/// 1. The first time each `buggify` use is evaluated, it is either enabled or disabled for the entire simulation run.
/// 1. Enabled uses of `buggify` have a 5% chance of evaluating to true
///
/// A good blogpost about buggify can be found [here](https://transactional.blog/simulation/buggify.html).
///
#[derive(Debug)]
pub struct Buggifier {
    buggified_lines: Mutex<HashMap<String, bool>>,
    random: Mutex<Option<Random>>,
}

impl Buggifier {
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
                    && deterministic_random.random_boolean(probability)
                {
                    tracing::info!("buggifying line {}", line);
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
    pub fn enable_buggify(&self, r: Random) {
        tracing::info!("enabling buggify");
        let mut data = self.random.lock();
        *data = Some(r);
    }

    /// disable buggify
    pub fn disable_buggify(&self) {
        tracing::info!("disabling buggify");
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
            random: Default::default(),
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

#[cfg(test)]
mod tests {
    use crate::buggify::{buggifier, Buggifier};
    use crate::deterministic::random::Random;
    use tracing::Level;

    #[test]
    fn buggify() {
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

        let random = Random::new_with_seed(42);
        b.enable_buggify(random);
        assert!(b.is_buggify_enabled(), "should be activated");

        for i in 0..100 {
            let result = if i == 8 { true } else { false };
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
                assert!(key.starts_with(&format!("{}", file!())));
            }
            for value in (*map).values() {
                assert!(value);
            }
        }

        b.disable_buggify();
        assert!(!b.buggify_with_prob(1.0), "should not buggified");
    }

    #[test]
    fn static_buggify() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_test_writer()
            .try_init();

        assert!(!buggifier().is_buggify_enabled());
        assert!(!buggifier().buggify_with_prob(1.0), "should not buggified");

        let random = Random::new_with_seed(42);
        buggifier().enable_buggify(random);
        assert!(buggifier().is_buggify_enabled(), "should be activated");

        for i in 0..100 {
            let result = if i == 8 { true } else { false };
            assert_eq!(
                buggifier().buggify(),
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
                assert!(key.starts_with(&format!("{}", file!())));
            }
            for value in (*map).values() {
                assert!(value);
            }
        }

        buggifier().disable_buggify();
        assert!(!buggifier().buggify_with_prob(1.0), "should not buggified");
    }
}
