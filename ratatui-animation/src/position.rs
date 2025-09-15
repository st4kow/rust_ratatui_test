#[derive(Default)]
#[derive(Clone,Copy)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64
}

#[allow(dead_code)]
impl Position {


    pub fn new() -> Position { Self::default() } 


    pub fn incr_x(&mut self, val: f64) -> &mut Self {
        self.x += val;
        self
    }
    pub fn decr_x(&mut self, val: f64) -> &mut Self {
        self.x -= val;
        self
    }
    pub fn incr_y(&mut self, val: f64) -> &mut Self {
        self.y += val;
        self
    }
    pub fn decr_y(&mut self, val: f64) -> &mut Self {
        self.y -= val;
        self
    }
    pub fn incr_z(&mut self, val: f64) -> &mut Self {
        self.z += val;
        self
    }
    pub fn decr_z(&mut self, val: f64) -> &mut Self {
        self.z -= val;
        self
    }


    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn get_y(&self) -> f64 {
        self.y
    }
    pub fn get_z(&self) -> f64 {
        self.z
    }


    pub fn set_x(&mut self, val: f64) -> &mut Self {
        self.x = val;
        self
    }
    pub fn set_y(&mut self, val: f64) -> &mut Self {
        self.y = val;
        self
    }
    pub fn set_z(&mut self, val: f64) -> &mut Self {
        self.z = val;
        self
    }

    pub fn set(&mut self, val: [f64; 3] ) -> &mut Self {
        self.set_x(val[0]).set_y(val[1]).set_z(val[2])
    }
}

