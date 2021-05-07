#![allow(unused_mut)]

// use shar::step;
mod shar;


fn main() {
    // println!("Hello, world!");
    let mut universe = shar::Uni{body : shar::generate(10)};
    universe.show();
    universe.step();
    universe.show();
}
