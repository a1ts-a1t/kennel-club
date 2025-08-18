use std::{thread::sleep, time::Duration};

use kennel::Kennel;

mod creature;
mod kennel;
mod math;
mod physics;

fn main() {
    let mut rng = rand::rng();
    let creature_metadata: Vec<_> = (0..32)
        .into_iter()
        .map(|idx| creature::Metadatum::new(format!("id{}", idx), 0.1, 0.01))
        .collect();
    let mut kennel = Kennel::new(creature_metadata, &mut rng).unwrap();

    // I'm pretty sure there are issues with numerical imprecision stacking on itself
    // but..... let's just integ test the hell out of it and call it a day :)
    for _ in 1..usize::MAX {
        kennel.print();
        kennel = kennel.next(&mut rng).unwrap();
        sleep(Duration::from_secs(1));
    }
}
