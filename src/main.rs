use std::{thread::sleep, time::Duration};

use kennel::Kennel;

use crate::creature::Creature;

mod creature;
mod env;
mod kennel;
mod math;
mod physics;
mod sprite;

fn main() {
    let mut rng = rand::rng();
    let json = std::fs::read_to_string("./data/creature_metadata.json")
        .expect("Unable to read metadata file");

    let creatures: Vec<Creature> = serde_json::from_str::<Vec<creature::Metadata>>(&json)
        .expect("Unable to deserialize creature metadata")
        .into_iter()
        .map(Creature::from)
        .collect();

    let mut kennel = Kennel::new(creatures, &mut rng).unwrap();

    loop {
        kennel.pretty_print();
        kennel = kennel
            .next(&mut rng)
            .expect("Error creating the next kennel state");
        sleep(Duration::from_secs(1));
    }
}
