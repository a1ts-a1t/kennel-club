use std::{fs::File, io::Write, path::PathBuf};

use image::ImageFormat;
use kennel_club::Kennel;
use rand::{SeedableRng, rngs::SmallRng};

static RNG_SEED: u64 = 1;

#[test]
fn test_kennel() {
    let mut rng = SmallRng::seed_from_u64(RNG_SEED);
    let dir = PathBuf::from("./data");
    let mut kennel = Kennel::load(&dir, &mut rng).expect("Error during kennel initialization");

    for _ in 0..1000000 {
        kennel = kennel
            .next(&mut rng)
            .expect("Error during kennel iteration");
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
