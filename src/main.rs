use std::path::PathBuf;
use std::{thread::sleep, time::Duration};

use kennel_club::Kennel;

fn main() {
    let mut rng = rand::rng();
    let mut kennel = Kennel::load(&PathBuf::from("./data"), &mut rng).unwrap();

    loop {
        kennel.pretty_print();
        kennel = kennel
            .next(&mut rng)
            .expect("Error creating the next kennel state");
        sleep(Duration::from_secs(1));
    }
}
