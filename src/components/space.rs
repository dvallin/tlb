use std::ops::{ AddAssign, Add, Sub };
use specs::{ Component, VecStorage };
use geometry::{ Rect, Shape, RectIter };

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Spawn {
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

    pub fn dot(self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Vector> for Position {
    type Output = Position;
    fn add(self, rhs: Vector) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Position> for Position {
    type Output = Vector;
    fn sub(self, rhs: Position) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    pub fn approx_equal(&self, other: &Position) -> bool {
        (self.x - other.x).abs() < 1.0e-6 && (self.y - other.y).abs() < 1.0e-6
    }
}

impl Component for Spawn {
    type Storage = VecStorage<Spawn>;
}

impl Component for Position {
    type Storage = VecStorage<Position>;
}

pub struct Viewport {
    r: Rect,
}

impl Viewport {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Viewport {
            r: Rect::new(x, y, w, h)
        }
    }

    pub fn center_at(self: &mut Self, p: Position) {
        self.r.center_at(p.x as i32, p.y as i32);
    }

    pub fn transform(self: &Self, p: (i32, i32)) -> (i32, i32) {
        (p.0 - self.r.left(), p.1 - self.r.top())
    }

    pub fn inv_transform(self: &Self, p: (i32, i32)) -> (i32, i32) {
        (p.0 + self.r.left(), p.1 + self.r.top())
    }

    pub fn into_iter(self: &Self) -> RectIter {
        self.r.into_iter()
    }

    pub fn visible(self: &Self, p: (i32, i32)) -> bool {
        self.r.is_enclosed(p)
    }
}
