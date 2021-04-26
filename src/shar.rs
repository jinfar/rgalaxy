#![allow(dead_code)]
#![allow(unused_mut)]
use rand::Rng;

pub struct Shar {
    pub massa: f64,
    x: f64,
    y: f64,
    velx: f64,
    vely: f64,
    accx: f64,
    accy: f64,
}

impl Shar{
        
    pub fn pull_by(&mut self, other: &Shar){
        let g = 1.0;
        let dist = ((self.x - other.x).powi(2)+(self.y - other.y).powi(2)).powf(0.5);
        if dist != 0.0 {
            self.accx += g*other.massa*(other.x - self.x)/dist/dist/dist;
            self.accy += g*other.massa*(other.y - self.y)/dist/dist/dist;
        }
    }

    pub fn update(&mut self, time: f64){
        self.velx += time*self.accx;
        self.vely += time*self.accy;
        self.x += time*self.velx;
        self.y += time*self.vely;
        self.accx = 0.0;
        self.accy = 0.0;    
    }
    
    pub fn show(&self) -> (f64, f64, f64, f64, f64, f64 , f64){
        (self.massa, self.x, self.y, self.velx, self.vely, self.accy, self.accy)
    }
    
}




pub fn run(){
    let mut luna = Shar{massa: 20.0, x: 410.0, y: 410.0, velx: 0.0, vely: 0.0, accx: 0.0, accy: 0.0};
    let mut sun = Shar{massa: 20.0, x: 400.0, y: 400.0, velx: 0.0, vely: 0.0, accx: 0.0, accy: 0.0};
    println!("massa solnca {:?}", sun.show());
    println!("massa solnca {:?}", luna.show());
    sun.pull_by(&luna);
    luna.pull_by(&sun);
    println!("massa solnca {:?}", sun.show());
    println!("massa solnca {:?}", luna.show());
    sun.update(1.0);
    luna.update(1.0);
    println!("massa solnca {:?}", sun.show());
    println!("massa solnca {:?}", luna.show());
}

// pub fn generate(times: i32) -> Vec<&'static mut Shar>{
//     let Universe: Vec<&mut Shar>;
//     for i in 1..times {
//         let sun = &mut Shar::new(20.0, 400.0, 400.0, 0.0, 0.0, 0.0, 0.0);
//         Universe.push(sun);
//         println!("{:?}", i);
//     }
//     return Universe
// }

// fn fill () -> Shar{
//     Shar{}
// }

pub fn generate(times: i32) -> Vec<Shar> {
    let mut universe = Vec::new();
    let mut rng = rand::thread_rng();
    let distr = rand_distr::Normal::new(0.0, 1.0).unwrap();
    for i in 0..times {
        // let massa = rng.gen_range(15.0..25.0);
        // println!("{:?}", rng.sample(distr));
        let mut sun = Shar{
            massa: rng.gen_range(15.0..25.0), 
            x: 400.0 + 20.0 * rng.sample(distr), 
            y: 400.0 + 20.0 * rng.sample(distr), 
            velx: rng.sample(distr), 
            vely: rng.sample(distr), 
            accx: 0.0, 
            accy: 0.0
        };
        universe.push(sun);
    }
    return universe
}