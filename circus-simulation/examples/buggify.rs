extern crate circus_simulation;

use circus_simulation::buggify::{buggifier, Buggifier};
use circus_simulation::deterministic::random::DeterministicRandom;
use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // let's create a buggifier
    let b = Buggifier::default();

    // init random with a seed
    let random = DeterministicRandom::new_with_seed(42);

    // enables buggify
    b.enable_buggify(random);

    for i in 0..10 {
        // this block has a 0.05% chance to be run
        // which is iteration 8 for seed 42
        if b.buggify() {
            tracing::info!("buggified at iteration {}", i);
        }
    }

    // buggify can also accept a probability
    if b.buggify_with_prob(1.0) {
        tracing::info!("buggified with a 100% probability!");
    }

    // you can also get a static buggifier that needs to be enabled
    buggifier().enable_buggify(DeterministicRandom::new_with_seed(42));
    if buggifier().buggify_with_prob(1.00) {
        tracing::info!("buggified with a 100% probability!");
    }
}
