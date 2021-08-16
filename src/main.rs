#![allow(unused_mut)]
use macroquad::{prelude::{WHITE, draw_circle, BLUE, next_frame}};
use macroquad::prelude::clear_background;
const KOL_ZVEZD : i32 = 1500;
// use shar::step;
mod shar;

fn risui_risui(uni: shar::Uni){
    for shar in uni.body {
        draw_circle(shar.x-100.0, shar.y-200., shar.massa.powf(0.3), WHITE);
    } 
}


#[macroquad::main("Stas")]
async fn main() {
     
    let mut universe = shar::Uni{body : shar::generate(KOL_ZVEZD)};
    loop{
        clear_background(BLUE);
        risui_risui(universe.clone());
        universe.step();
        next_frame().await;
    }
}
