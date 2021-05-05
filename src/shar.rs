#![allow(dead_code)]
// #![allow(unused_mut)]
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Shar {
    pub massa: f64,
    pub x: f64,
    pub y: f64,
    pub velx: f64,
    vely: f64,
    pub accx: f64,
    accy: f64,
}

impl Shar{
        
    pub fn pull_by(&mut self, other: &Shar) -> Shar{
        let g = 1.0;
        let dist = ((self.x - other.x).powi(2)+(self.y - other.y).powi(2)).powf(0.5);
        if dist != 0.0 {
            self.accx += g*other.massa*(other.x - self.x)/dist/dist/dist;
            self.accy += g*other.massa*(other.y - self.y)/dist/dist/dist;
        }
        *self
    }

    pub fn update(&mut self, time: f64) -> Shar{
        self.velx += time*self.accx;
        self.vely += time*self.accy;
        self.x += time*self.velx;
        self.y += time*self.vely;
        self.accx = 0.0;
        self.accy = 0.0;    
        *self
    }
    
    pub fn show(&self) -> (f64, f64, f64, f64, f64, f64 , f64){
        (self.massa, self.x, self.y, self.velx, self.vely, self.accy, self.accy)
    }
}

pub fn step(universe: Vec<Shar>) -> Vec<Shar>{
    let temp = &universe.clone();
    let mut newverse: Vec<Shar> = vec![];
    for mut sun in universe {
        for luna in temp {
            sun.pull_by(luna);
        }
        newverse.push(sun);
    }
    let mut newverse2: Vec<Shar> = vec![];
    for mut sun in newverse {
        sun.update(1.0);
        newverse2.push(sun);
    }    
    return newverse2
}

pub fn generate(times: i32) -> Vec<Shar> {
    let mut universe = Vec::new();
    let mut rng = rand::thread_rng();
    let distr = rand_distr::Normal::new(0.0, 1.0).unwrap();
    for _i in 0..times {
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
