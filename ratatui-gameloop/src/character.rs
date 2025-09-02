use crate::position::Position;

pub struct Character {
    pub position: Position,
    pub speed: f64, // speed in units/event
    pub width: f64,
    pub height: f64
}

impl Character {
    pub fn new() -> Character { 
        Character { position: Position::default(), speed : 1.0, width : 5.0 , height: 5.0  }
    }
    pub fn move_up(&mut self ) -> &mut Self {
        self.position.decr_y(self.speed);
        self
    }
    pub fn move_down(&mut self ) -> &mut Self {
        self.position.incr_y(self.speed);
        self
    }
    pub fn move_left(&mut self ) -> &mut Self {
        self.position.decr_x(self.speed);
        self
    }
    pub fn move_right(&mut self ) -> &mut Self {
        self.position.incr_x(self.speed);
        self
    }
}