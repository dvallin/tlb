use std::ops::{ AddAssign };
use specs::{ Component, VecStorage };

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

pub fn mul(p: Vector, s: f32) -> Vector {
    Vector { x: p.x * s, y: p.y * s }
}

impl Vector {
    pub fn length(self) -> f32 {
        (self.x*self.x + self.y*self.y).sqrt()
    }

    pub fn norm(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            self
        } else {
            mul(self, 1.0/len)
        }
    }
}

impl AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Component for Position {
    type Storage = VecStorage<Position>;
}
