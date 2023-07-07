use crate::fishes::Fish;
use crate::vectors::Vector2;

mod bodies;
mod fishes;
mod vectors;

pub fn main() {
    let fish_a = Fish::new(Vector2::new(1.0, 1.0), 10.0, 10.0);
    let fish_b = Fish::new(Vector2::new(1.0, 101.0), 10.0, 10.0);
    println!("{:?}", Vector2::random_in_radius(2.0));
}