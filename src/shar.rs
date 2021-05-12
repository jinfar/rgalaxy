#![allow(dead_code)]
// #![allow(unused_mut)]
use rand::Rng;
// use std::marker::Copy;

#[derive(Copy, Clone)]
pub struct Shar {
    pub massa: f32,
    pub x: f32,
    pub y: f32,
    pub velx: f32,
    pub vely: f32,
    pub accx: f32,
    pub accy: f32,
}

impl Shar{
    pub fn pull_by(&mut self, other: &Shar) {
        let g = 1.0;
        let dist = ((self.x - other.x).powi(2)+(self.y - other.y).powi(2)).powf(0.5);
        if dist != 0.0 {
            self.accx += g*other.massa*(other.x - self.x)/dist/dist/dist;
            self.accy += g*other.massa*(other.y - self.y)/dist/dist/dist;
        }
    }

    pub fn update(&mut self, time: f32) {
        // let mut mtime: f32;
        // match  time{
        //     Some(time)  => mtime = time,
        //     None => mtime = 1.0 as f32
        // }
        self.velx += time*self.accx;
        self.vely += time*self.accy;
        self.x += time*self.velx;
        self.y += time*self.vely;
        self.accx = 0.0;
        self.accy = 0.0;    
    }
    
    pub fn show(&self){
        println!("mas: {:.2}", self.massa);
        println!("pos: {:.2}, {:.2}", self.x, self.y);
        println!("vel: {:.2}, {:.2}", self.velx, self.vely);
        println!("acc: {:.2}, {:.2}", self.accx, self.accy);
    }
}
// #[derive(Clone)]
pub struct Uni {
    pub body: Vec<Shar>
}

// impl Copy for Uni { }


impl Uni{
    pub fn step_o(&mut self) {
        self.body = step_f(self.body.clone());
        }
    
    pub fn step(&mut self){
        let temp = &self.body.clone();
        for sun in &mut self.body {
            for luna in temp {
                sun.pull_by(luna);
            }
        }
        for sun in &mut self.body {
            sun.update(1.0);
        }    
    }

    // pub fn step_m(&mut self){
    //     let temp_body = self.body.clone();
    //     let m: Vec<Shar> = temp_body.into_iter().map(|sun| {
    //         for luna in &temp_body{
    //             sun.pull_by(luna);
    //         }
    //     }).collect::<Vec<Shar>>();
    // }
    
    pub fn reinit(&mut self) {
        self.body = generate(10);
    }

    pub fn show(&self){
        let mut count = 1;
        for sun in self.body.clone(){
            println!("Planet {}:", count);
            sun.show();
            // println!("  ");
            count += 1;
        }
    }
}

pub fn step_f(universe: Vec<Shar>) -> Vec<Shar>{
    let temp = &universe.clone();
    let mut newverse: Vec<Shar> = vec![];
    for mut sun in universe {
        for luna in temp {
            sun.pull_by(luna);
        }
        newverse.push(sun);
    }
    for sun in &mut newverse {
        sun.update(1.0);
    }    
    newverse
}

pub fn generate(times: i32) -> Vec<Shar> {
    let mut universe = vec![];
    let mut rng = rand::thread_rng();
    let distr = rand_distr::Normal::new(0.0, 1.0).unwrap();
    for _ in 0..times {
        let mut sun = Shar{
            massa: rng.gen_range(15.0..35.0), 
            x: 400.0 + 20.0 * rng.sample(distr), 
            y: 400.0 + 20.0 * rng.sample(distr), 
            velx: rng.sample(distr), 
            vely: rng.sample(distr), 
            accx: 0.0, 
            accy: 0.0
        };
        universe.push(sun);
    }
    universe
}
