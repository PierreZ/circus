extern crate circus_buggify;

use circus_buggify::{buggify_with_prob, enable_buggify, Buggifier};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // let's create a buggifier
    let b = Buggifier::default();

    // enables buggify with a seed
    b.enable_buggify(SmallRng::seed_from_u64(42));

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
    enable_buggify(SmallRng::seed_from_u64(42));
    if buggify_with_prob(1.00) {
        tracing::info!("buggified with a 100% probability!");
    }
}
