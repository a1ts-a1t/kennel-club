use std::{fs::File, io::Write, path::PathBuf};

use image::ImageFormat;
use kennel_club::Kennel;
use rand::{SeedableRng, rngs::SmallRng};

static RNG_SEED: u64 = 1;

#[test]
fn test_kennel() {
    static STEP_COUNT: usize = 5_000;
    static SEED_COUNT: u64 = 16;

    for seed in 0..SEED_COUNT {
        let mut rng = SmallRng::seed_from_u64(seed);
        let dir = PathBuf::from("./data");
        let mut kennel = Kennel::load(&dir, &mut rng).expect("Error during kennel initialization");

        for step in 0..STEP_COUNT {
            kennel = kennel
                .next(&mut rng)
                .expect("Error during kennel iteration");

            let creatures = kennel.creatures();
            for (i, creature) in creatures.iter().enumerate() {
                let collidable = creature.as_collidable();
                assert!(
                    !collidable.is_out_of_unit_bounds(),
                    "seed {seed} step {step}: {} escaped the kennel at {:?}",
                    creature.id,
                    creature.position
                );

                for other in creatures.iter().skip(i + 1) {
                    let other_collidable = other.as_collidable();
                    assert!(
                        !collidable.is_colliding(&other_collidable),
                        "seed {seed} step {step}: {} and {} overlapped by {:.6} at {:?} vs {:?}",
                        creature.id,
                        other.id,
                        creature.radius + other.radius
                            - (&creature.position - &other.position).norm(),
                        creature.position,
                        other.position
                    );
                }
            }
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
