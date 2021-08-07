use circus_test::with_random_seed;
use circus_test::with_seed;

#[with_random_seed]
#[test]
fn random_seed(seed: u64) {
    println!("{}", seed);
}

#[with_seed(42)]
#[test]
fn with_seed(seed: u64) {
    assert_eq!(42, seed);
}

#[with_random_seed]
#[test]
#[ignore]
fn ignored_test(_seed: u64) {
    assert!(false);
}
