

struct Shar {
    massa: f64,
    x: f64,
    y: f64,
    velx: f64,
    vely: f64,
    accx: f64,
    accy: f64,
}

impl Shar{
    
    fn new(massa: f64, x: f64, y: f64, velx: f64, vely: f64, accx: f64, accy: f64) -> Shar{
        Shar{
            massa: massa,
            x: x,
            y: y,
            velx: velx,
            vely: vely,
            accx: accx,
            accy: accy,
        }
    }

    fn pull_by(&mut self, other: &Shar){
        let g = 1.0;
        let dist = ((self.x - other.x).powi(2)+(self.y - other.y).powi(2)).powf(0.5);
        if dist != 0.0 {
            self.accx += g*other.massa*(other.x - self.x)/dist/dist/dist;
            self.accy += g*other.massa*(other.y - self.y)/dist/dist/dist;
        }
    }

    fn update(&mut self, time: f64){
        self.velx += time*self.accx;
        self.vely += time*self.accy;
        self.x += time*self.velx;
        self.y += time*self.vely;
        self.accx = 0.0;
        self.accy = 0.0;    
    }

    fn show(&self) -> (f64, f64, f64, f64, f64, f64 , f64){
        (self.massa, self.x, self.y, self.velx, self.vely, self.accy, self.accy)
    }



}

pub fn run(){
    let sun = &mut Shar::new(20.0, 400.0, 400.0, 0.0, 0.0, 0.0, 0.0);
    let luna = Shar::new(20.0, 410.0, 420.0, 0.0, 0.0, 0.0, 0.0);
    println!("massa solnca {:?}", sun.show());
    println!("massa solnca {:?}", luna.show());
    sun.pull_by(&luna);
    println!("massa solnca {:?}", sun.show());
    println!("massa solnca {:?}", luna.show());
    sun.update(1.0);
    println!("massa solnca {:?}", sun.show());
    println!("massa solnca {:?}", luna.show());
}
