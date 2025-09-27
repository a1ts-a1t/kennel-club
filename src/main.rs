use std::{thread::sleep, time::Duration};

use kennel::Kennel;

mod creature;
mod kennel;
mod math;
mod physics;
mod sprite;

fn main() {
    let mut rng = rand::rng();
    let creature_metadata: Vec<_> = (0..64)
        .into_iter()
        .map(|idx| creature::Metadata::new(format!("id{}", idx), 0.1, 0.01, None))
        .collect();
    let mut kennel = Kennel::new(creature_metadata, &mut rng).unwrap();

    for _ in 1..usize::MAX {
        kennel.pretty_print();
        kennel = kennel.next(&mut rng).unwrap();
        sleep(Duration::from_secs(1));
    }
}
