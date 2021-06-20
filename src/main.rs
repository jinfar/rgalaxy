#![allow(unused_mut)]

// use shar::step;
mod draw;
mod shar;

// async fn inter()
fn play_episode(uni: &mut shar::Uni, cadri: u32) -> Vec<Vec<[f32; 3]>> {
    let mut itog = vec![];
    for _ in 0..cadri {
        itog.push(draw::conv(uni.body.clone()));
        uni.step();
    }
    itog
}

fn main() {
    let mut universe = shar::Uni {
        body: shar::generate(300),
    };

    let ris = play_episode(&mut universe, 390);
    draw::narisui_gif(ris);
    // let mut kalazh = Vec::new();
    // for _ in 0..30 {
    // let ris = draw::get_ris(universe.clone());
    // kalazh.push(ris);
    // universe.step();

    // print!("{}[2J", 27 as char);
    // }
    // for ris in kalazh {
    // for st in ris {
    // println!("{}", st);
    // }
    // std::thread::sleep(std::time::Duration::from_millis(500));
    // print!("\x1B[2J\x1B[1;1H");
    // }
}
