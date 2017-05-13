use std::ops::{ AddAssign, Add };
use specs::{ Component, VecStorage };
use geometry::{ Rect, Pos, Shape, RectIter };

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

impl Add<Vector> for Position {
    type Output = Position;
    fn add(self, rhs: Vector) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
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

    pub fn transform(self: &Self, p: Pos) -> Pos {
        Pos { x: p.x - self.r.left(), y: p.y - self.r.top() }
    }

    pub fn into_iter(self: &Self) -> RectIter {
        self.r.into_iter()
    }

    pub fn visible(self: &Self, p: Pos) -> bool {
        self.r.is_enclosed(p)
    }
}
