#![allow(unused_mut)]

// use shar::step;
mod shar;
mod draw;

fn main() {
    // println!("Hello, world!");
    let mut universe = shar::Uni{body : shar::generate(10)};
    universe.show();
    universe.step();
    universe.show();
    // for _ in 0..0{
    //     // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@")
    //     for _ in 0..70{
    //         // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@")
    //         print!("{}", draw::CHARSET[4])
    //     }
    //     print!("\n")
    // }
    // for i in 0..4 {
    //     draw::pr_plot(draw::CHARSET[i]);
    //     std::thread::sleep(std::time::Duration::from_millis(500));
    //     print!("\x1B[2J\x1B[1;1H");
        // print!("{}[2J", 27 as char);
    // }
}

// fn tr(per: Option<u32>){
//     match per{
//         Some(z) => println!("OK"),
//         None => println!("notOK"),
//     }
// }