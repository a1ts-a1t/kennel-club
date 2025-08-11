use kennel::Kennel;

mod collidable;
mod creature;
mod kennel;
mod vec2;
mod step;

fn main() {
    let mut rng = rand::rng();
    let width = 64.0;
    let height = 64.0;
    let creature_metadata: Vec<_> = (0..16).into_iter()
        .map(|idx| creature::Metadatum::new(format!("id{}", idx), 1.0, 1.0))
        .collect();
    let kennel = Kennel::new(width, height, creature_metadata, &mut rng).unwrap();
    kennel.print();
}
