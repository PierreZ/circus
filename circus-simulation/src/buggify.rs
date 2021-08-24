//! `buggify!` macro
use crate::deterministic::random::Random;
use parking_lot::Mutex;
use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug)]
#[doc(hidden)]
pub struct Buggifier {
    pub buggified_lines: Mutex<HashMap<String, bool>>,
    pub random: Mutex<Option<Random>>,
}

#[doc(hidden)]
pub static BUGGIFIER_INSTANCE: Lazy<Buggifier> = Lazy::new(|| Buggifier::default());

#[doc(hidden)]
pub fn handle_buggify(line: String, probability: f64) -> bool {
    let mut lock = BUGGIFIER_INSTANCE.random.lock();

    match (*lock).as_mut() {
        None => false,
        Some(deterministic_random) => {
            let mut already_buggified = BUGGIFIER_INSTANCE.buggified_lines.lock();

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

impl Default for Buggifier {
    /// Create a new Buggifier
    fn default() -> Self {
        Buggifier {
            buggified_lines: Mutex::new(HashMap::new()),
            random: Default::default(),
        }
    }
}
/// checks if buggify is enabled
pub fn is_buggify_enabled() -> bool {
    BUGGIFIER_INSTANCE.random.lock().is_some()
}

/// enables buggify by giving a random source
pub fn enable_buggify(r: Random) {
    tracing::info!("enabling buggify");
    let mut data = BUGGIFIER_INSTANCE.random.lock();
    *data = Some(r);
}

/// disable buggify
pub fn disable_buggify() {
    tracing::info!("disabling buggify");
    let mut data = BUGGIFIER_INSTANCE.random.lock();
    *data = None;
    let mut map = BUGGIFIER_INSTANCE.buggified_lines.lock();
    map.clear();
}

#[macro_export]
/// `buggify!` allow you to cooperate with the simulator to inject failures.
/// It has the following rules:
/// 1. it only ever evaluates to true when run in simulation.
/// 1. The first time each `buggify!` use is evaluated, it is either enabled or disabled for the entire simulation run.
/// 1. Enabled uses of `buggify!` have a 5% chance of evaluating to true
///
/// A good blogpost about buggify can be found [here](https://transactional.blog/simulation/buggify.html).
///
/// ## Getting started with `buggify!`
///
/// ```rust
/// extern crate circus_simulation;
///
/// use circus_simulation::buggify;
/// use circus_simulation::buggify::*;
/// use circus_simulation::deterministic::random::Random;
/// use tracing::Level;
///
/// fn main() {
///    // init random with a seed
///    let random = Random::new_with_seed(42);
///
///    tracing_subscriber::fmt()
///        .with_max_level(Level::DEBUG)
///        .init();
///
///    // enables buggify
///    enable_buggify(random);
///
///    for i in 0..10 {
///        // this block has a 0.05% chance to be run
///        // which is iteration 8 for seed 42
///        if buggify!() {
///            tracing::info!("buggified at iteration {}", i);
///        }
///    }
///
///    // buggify can also accept a probability
///    if buggify!(1.0) {
///        tracing::info!("buggified with a 100% probability!");
///    }
/// }
/// ```
///
macro_rules! buggify {
    ($probability:expr) => {{
        let line = format!("{}:{}", file!(), line!());
        handle_buggify(line, $probability)
    }};
    () => {{
        let line = format!("{}:{}", file!(), line!());
        handle_buggify(line, 0.05)
    }};
}
#[cfg(test)]
mod tests {

    use crate::buggify::BUGGIFIER_INSTANCE;
    use crate::buggify::*;
    use crate::deterministic::random::Random;

    #[test]
    fn test_macro() {
        assert!(!is_buggify_enabled());
        assert!(!buggify!(1.0), "should not buggified");

        {
            let data = BUGGIFIER_INSTANCE.random.lock();
            assert!((*data).is_none());

            let map = BUGGIFIER_INSTANCE.buggified_lines.lock();
            assert!((*map).is_empty());
        }

        let random = Random::new_with_seed(42);
        enable_buggify(random);

        assert!(is_buggify_enabled(), "should be activated");

        for i in 0..100 {
            let result = if i == 8 { true } else { false };
            assert_eq!(buggify!(), result, "{} should have been {}", i, result);
        }

        {
            dbg!(&BUGGIFIER_INSTANCE);
            let data = BUGGIFIER_INSTANCE.random.lock();
            assert!((*data).is_some());

            let map = BUGGIFIER_INSTANCE.buggified_lines.lock();
            assert_eq!((*map).len(), 1);
            for key in (*map).keys() {
                assert!(key.starts_with("simulation/src/buggify.rs:"));
            }
            for value in (*map).values() {
                assert!(value);
            }
        }

        disable_buggify();
        assert!(!buggify!(1.0), "should not buggified");
    }
}
