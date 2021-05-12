// const ascii_list: [char; 4] = [" ", ".", ", ", ":"];
#![allow(unused)]
// mod shar;

use crate::shar::{Shar, Uni};
pub const CHARSET: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
pub const CHARSET_HD: [char; 70] = [' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'];
const MAX_MASS: f32 = 10f32;

pub fn conv(data:Vec<Shar>) -> Vec<[f32; 3]>{
    let mut temp:[f32; 3] = [0.0,0.0,0.0];
    let mut itog: Vec<[f32; 3]> = vec![];

    for item in data.clone(){
        temp[0] = item.x;
        temp[1] = item.y;
        temp[2] = item.massa;
        itog.push(temp);
    }
    itog
}

pub fn get_brightness(data: Vec<[f32; 3]>, mal_gr_x: f32, bol_gr_x: f32, mal_gr_y: f32, bol_gr_y: f32) -> f32{
    let mut itog = 0f32;
    for item in data{
        if item[0] >= mal_gr_x && item[0] < bol_gr_x && item[1] >= mal_gr_y && item[1] < bol_gr_y {
            itog += item[2];
        } 
    }
    itog
}

pub fn get_char(brightness: f32) -> char{
    let char_set = CHARSET;
    if brightness == 0.0 {return char_set[0]}
    let shag = (MAX_MASS/char_set.len() as f32).floor();
    let nom = (brightness / shag).floor();
    if (nom as usize) >= char_set.len() - 1 {return char_set[char_set.len() - 1]}
    char_set[(nom as usize )+ 1]
}

pub fn get_ris(data: Uni) -> Vec<String>{
    let mut itog: Vec<String> = vec![];
    let plot_h_px = 400f32;
    let plot_w_px = 400f32;
    let plot_h_ch = 200u32;
    let plot_w_ch = 50u32;
    let min_y_gr  = 0f32;
    let min_x_gr  = 0f32;
    let mut br = 0f32;
    let mut ch;
    for row in 1..=plot_w_ch{
        let mut bukvi: Vec<char> = vec![];
        for col in 1..=plot_h_ch{
            let mal_gr_y = row  as f32 * plot_w_px / plot_w_ch as f32;
            let bol_gr_y = (row + 1)  as f32 * plot_w_px / plot_w_ch as f32;
            let mal_gr_x = col  as f32 * plot_h_px / plot_h_ch as f32;
            let bol_gr_x = (col + 1)  as f32 * plot_h_px / plot_h_ch as f32;
            ch = conv(data.body.clone());

            br = get_brightness( ch, mal_gr_x+400f32, bol_gr_x+400f32, mal_gr_y+400f32, bol_gr_y+400f32);
            let cha = get_char(br);
            bukvi.push(cha);
        }
        let d = format!("{}", bukvi.into_iter().collect::<String>());
        itog.push(d);
    }
    itog
}