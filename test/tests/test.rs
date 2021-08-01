use circus_test::with_random_seed;

#[with_random_seed]
#[test]
fn random_seed(seed: u64) {
    println!("{}", seed);
}

#[with_random_seed]
#[test]
#[ignore]
fn ignored_test(seed: u64) {
    assert!(false);
}
