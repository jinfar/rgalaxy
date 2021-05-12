#![allow(unused_mut)]

// use shar::step;
mod shar;
mod draw;

fn main() {
    let mut universe = shar::Uni{body : shar::generate(500)};
        for _ in 0..30 {
            let ris = draw::get_ris(universe.clone());
            for st in ris{
                println!("{}",st);
            }
            // std::thread::sleep(std::time::Duration::from_millis(100));
            universe.step();
            print!("\x1B[2J\x1B[1;1H");

        // print!("{}[2J", 27 as char);
        }
}
