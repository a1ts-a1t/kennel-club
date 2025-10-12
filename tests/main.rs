use std::path::PathBuf;

use kennel_club::Kennel;
use rand::{SeedableRng, rngs::SmallRng};

#[test]
fn test_kennel() {
    let mut rng = SmallRng::seed_from_u64(0);
    let dir = PathBuf::from("./data");
    let mut kennel = Kennel::load(&dir, &mut rng).expect("Error during kennel initialization");

    for _ in 0..1000000 {
        kennel = kennel
            .next(&mut rng)
            .expect("Error during kennel iteration");
    }
}
