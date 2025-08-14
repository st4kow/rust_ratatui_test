
#[derive(Default)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64
}

impl Position {


    pub fn new() -> Position { Position{ x : 0.0, y : 0.0, z : 0.0 } } 


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

