use kennel::Kennel;

mod collidable;
mod creature;
mod kennel;
mod vec2;

fn main() {
    let mut rng = rand::rng();
    let width = 64.0;
    let height = 64.0;
    let creature_metadata = vec![creature::Metadatum::new("id".to_string(), 8.0, 8.0)];
    let kennel = Kennel::new(width, height, creature_metadata, &mut rng).unwrap();
    kennel.print();
}
