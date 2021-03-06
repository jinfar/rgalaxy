mod shar;


fn main() {
    println!("Hello, world!");
    // shar::run();
    let w = shar::generate(3);
    println!("{:?}", w);
}
