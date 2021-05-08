// const ascii_list: [char; 4] = [" ", ".", ", ", ":"];
#![allow(unused)]
pub const CHARSET: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
pub const CHARSET_HD: [char; 70] = [' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'];

pub fn to_char(tochki_X: Vec<f64>, tochki_Y: Vec<f64>, massa: Vec<f64>) -> Vec<char>{
    let mut s = Vec::new();
    s.push(CHARSET[0]);
    s
    // return s
}

pub fn pr_plot(ch: char){
    for _ in 0..30{
        for _ in 0..70{
            print!("{}", ch)
        }
        print!("\n")
    }
}
