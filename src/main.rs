#![allow(unused_mut)]

// use shar::step;
mod shar;
mod draw;

fn main() {
    let mut universe = shar::Uni{body : shar::generate(1000)};
        for _ in 0..30 {
            draw::ris(universe.clone());
            universe.step();
            std::thread::sleep(std::time::Duration::from_millis(1000));
            print!("\x1B[2J\x1B[1;1H");
        // print!("{}[2J", 27 as char);
        }
}
