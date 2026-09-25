use std::{fs::File, io::Write, path::PathBuf};

use image::ImageFormat;
use kennel_club::{Collider, Kennel, signed_distance};
use rand::{SeedableRng, rngs::SmallRng};

const RNG_SEED: u64 = 1;

#[test]
fn test_kennel() {
    const STEP_COUNT: usize = 5_000;
    const SEED_COUNT: u64 = 16;
    // regression (freeze-lock): pairs that come into contact used to
    // lock up, taking zero-length steps forever. a fully-frozen tick
    // streak can only come from every creature being idle or asleep at
    // once; healthy runs stay well under this bound, a lockup runs for
    // hundreds of ticks
    const MAX_FROZEN_STREAK: usize = 64;

    for seed in 0..SEED_COUNT {
        let mut rng = SmallRng::seed_from_u64(seed);
        let dir = PathBuf::from("./data");
        let mut kennel = Kennel::load(&dir, &mut rng).expect("Error during kennel initialization");
        let mut frozen_streak = 0;

        for step in 0..STEP_COUNT {
            let before: Vec<(f64, f64)> = kennel
                .creatures()
                .iter()
                .map(|creature| (creature.position.x, creature.position.y))
                .collect();

            kennel = kennel
                .next(&mut rng)
                .expect("Error during kennel iteration");

            let creatures = kennel.creatures();
            for creature in &creatures {
                let (x, y, r) = (creature.position.x, creature.position.y, creature.radius);
                assert!(
                    x >= r && x <= 1.0 - r && y >= r && y <= 1.0 - r,
                    "seed {seed} step {step}: {} escaped the kennel at ({x}, {y})",
                    creature.id
                );
            }

            for (i, creature) in creatures.iter().enumerate() {
                let collider = Collider::new_circle(creature.position, creature.radius);
                for other in creatures.iter().skip(i + 1) {
                    let other_collider = Collider::new_circle(other.position, other.radius);
                    let gap = signed_distance(&collider, &other_collider)
                        .expect("gap must exist between circles");
                    assert!(
                        gap > 0.0,
                        "seed {seed} step {step}: {} and {} overlapped by {:.6} at {:?} vs {:?}",
                        creature.id,
                        other.id,
                        -gap,
                        creature.position,
                        other.position
                    );
                }
            }

            let any_moved = kennel
                .creatures()
                .iter()
                .zip(before)
                .any(|(creature, position)| (creature.position.x, creature.position.y) != position);
            frozen_streak = if any_moved { 0 } else { frozen_streak + 1 };
            assert!(
                frozen_streak <= MAX_FROZEN_STREAK,
                "seed {seed} froze for {frozen_streak} ticks (up to step {step})"
            );
        }
    }
}

#[test]
fn test_image() {
    let mut rng = SmallRng::seed_from_u64(RNG_SEED);
    let dir = PathBuf::from("./data");
    let kennel = Kennel::load(&dir, &mut rng).expect("Error during kennel initialization");

    let image_data = kennel
        .get_image(1024, 1024, ImageFormat::Png)
        .expect("Error during image processing");
    let mut file = File::create("test_kennel_image.png").expect("Error creating test image file");
    file.write_all(&image_data)
        .expect("Error during image writing");
}
