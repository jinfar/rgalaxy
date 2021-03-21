#![allow(unused_mut)]
mod shar;


fn main() {
    println!("Hello, world!");
    // shar::run();
    let mut w = shar::generate(4);
    // println!("{:?}", w[1].show());
    // let mut tmp = &w[1];
    // let mut tmp2 = &w[2];
    // w[1].pull_by(&w[2]);
    // w[1].pull_by(tmp2);
    println!("{:?}", w[0].massa);
    println!("{:?}", w[1].massa);
    println!("{:?}", w[2].massa);
}
