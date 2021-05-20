#![allow(unused_mut)]


// use shar::step;
mod shar;
mod draw;


// async fn inter()

fn main() {
    let mut universe = shar::Uni{body : shar::generate(500)};
    let mut kalazh = Vec::new();
        for _ in 0..30 {
            let ris = draw::get_ris(universe.clone());
            kalazh.push(ris);
            universe.step();
            
            // print!("{}[2J", 27 as char);
        }
        for ris in kalazh{
            for st in ris{
                println!("{}",st);
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
            print!("\x1B[2J\x1B[1;1H");
        }
}
