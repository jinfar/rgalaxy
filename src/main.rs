#![allow(unused_mut)]

// use shar::step;
mod shar;


fn main() {
    println!("Hello, world!");
    let mut universe = shar::generate(4);
    dbg!(universe[0].accx);
    dbg!(universe[0].velx);
    dbg!(universe[0].x);
    dbg!(universe[0].y);
    universe = shar::step(universe);
    dbg!(universe[0].accx);
    dbg!(universe[0].velx);
    dbg!(universe[0].x);
    dbg!(universe[0].y);
    
}
