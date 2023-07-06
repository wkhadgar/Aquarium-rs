use crate::vectors::Vector2;

mod bodies;
mod fishes;
mod vectors;

pub fn main() {
    println!("{:?}", Vector2::random_in_radius(2.0));
}