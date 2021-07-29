use crate::deterministic::random::DeterministicRandom;
use parking_lot::lock_api::MutexGuard;
use parking_lot::{Mutex, RawMutex};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Deref;
use tracing::{event, instrument, span, Level};

lazy_static! {
    static ref BUGGIFY_CACHE: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
    static ref BUGGIFY_RANDOM: Mutex<Option<DeterministicRandom>> = Mutex::new(None);
}

pub fn is_buggify_enabled() -> bool {
    BUGGIFY_RANDOM.lock().is_some()
}

pub fn enable_buggify(r: DeterministicRandom) {
    let mut data = BUGGIFY_RANDOM.lock();
    *data = Some(r.clone());
}

pub fn disable_buggify() {
    let mut data = BUGGIFY_RANDOM.lock();
    *data = None;
}

fn clear_buggify_cache() {
    let mut data = BUGGIFY_CACHE.lock();
    (*data).clear();
}

#[macro_export]
macro_rules! buggify {
    () => {{
        let mut lock = BUGGIFY_RANDOM.lock();

        match (*lock).as_mut() {
            None => false,
            Some(deterministic_random) => {
                let line = format!("{}:{},{}", file!(), line!(), column!());
                let mut already_buggified = BUGGIFY_CACHE.lock();

                if !already_buggified.deref().contains_key(&line)
                    && deterministic_random.random_boolean()
                {
                    span!(Level::TRACE, "buggified", %line);
                    already_buggified.insert(line, true);
                    true
                } else {
                    false
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter, Registry};

    static LOGGER_INIT: Once = Once::new();

    use crate::buggify::*;
    use crate::deterministic::random::DeterministicRandom;
    use parking_lot::Once;
    use std::sync::{MutexGuard, PoisonError};

    pub(crate) fn install_tracing() {
        LOGGER_INIT.call_once(|| {
            let fmt_layer = fmt::layer().with_target(false);
            let filter_layer = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("trace"))
                .unwrap();

            tracing_subscriber::registry()
                .with(filter_layer)
                .with(fmt_layer)
                .init();
        })
    }
    #[test]
    fn test_buggify_enabled() {
        install_tracing();
        disable_buggify();
        clear_buggify_cache();

        assert!(!is_buggify_enabled());
        for i in 0..9999 {
            assert!(!buggify!(), "should not buggified");
        }

        let mut random = DeterministicRandom::new_with_seed(42);
        enable_buggify(random);

        assert!(is_buggify_enabled(), "should be activated");

        for i in 0..100 {
            let result = if i == 3 { true } else { false };
            assert_eq!(buggify!(), result);
        }
    }
}
