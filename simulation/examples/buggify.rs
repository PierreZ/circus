extern crate circus_simulation;

use circus_simulation::buggify;
use circus_simulation::buggify::*;
use circus_simulation::deterministic::random::Random;
use tracing::Level;

fn main() {
    // init random with a seed
    let random = Random::new_with_seed(42);

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // enables buggify
    enable_buggify(random);

    for i in 0..10 {
        // this block has a 0.05% chance to be run
        // which is iteration 8 for seed 42
        if buggify!() {
            tracing::info!("buggified at iteration {}", i);
        }
    }

    // buggify can also accept a probability
    if buggify!(1.0) {
        tracing::info!("buggified with a 100% probability!");
    }
}
