use std::{thread::sleep, time::Duration};

use kennel::Kennel;

mod collidable;
mod creature;
mod kennel;
mod step;
mod math;

fn main() {
    let mut rng = rand::rng();
    let width = 64.0;
    let height = 64.0;
    let creature_metadata: Vec<_> = (0..16)
        .into_iter()
        .map(|idx| creature::Metadatum::new(format!("id{}", idx), 1.0, 1.0))
        .collect();
    let mut kennel = Kennel::new(width, height, creature_metadata, &mut rng).unwrap();
    for _ in 0..20 {
        kennel.print();
        kennel = kennel.next(&mut rng).unwrap();
        sleep(Duration::from_secs(2));
    }
}
