#[derive(Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub trait Shape: Copy + IntoIterator<Item=Pos> {
    fn center(&self) -> Pos;
    fn intersects_with(&self, other: &Self) -> bool;
    fn bounding_box(&self) -> Rect;

    fn is_enclosed(&self, pos: Pos) -> bool;
    fn is_boundary(&self, pos: Pos) -> bool;
    fn is_interior(&self, pos: Pos) -> bool;
}

#[derive(Copy, Clone)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

impl Shape for Rect {
    fn center(&self) -> Pos {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        Pos { x: center_x, y: center_y }
    }

    fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2) && (self.x2 >= other.x1) &&
            (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }

    fn bounding_box(&self) -> Rect {
        *self
    }

    fn is_enclosed(&self, pos: Pos) -> bool {
        pos.x >= self.x1 && pos.x <= self.x2 && pos.y >= self.y1 && pos.y <= self.y2
    }

    fn is_boundary(&self, pos: Pos) -> bool {
        pos.x == self.x1 || pos.x == self.x2 || pos.y == self.y1 || pos.y == self.y2
    }

    fn is_interior(&self, pos: Pos) -> bool {
        pos.x > self.x1 && pos.x < self.x2 && pos.y > self.y1 && pos.y < self.y2
    }
}

impl IntoIterator for Rect {
    type Item = Pos;
    type IntoIter = RectIter;
    fn into_iter(self) -> RectIter {
        RectIter { rect: self, pos: Pos { x: self.x1 - 1, y: self.y1 } }
    }
}

pub struct RectIter {
    rect: Rect,
    pos: Pos,
}

impl Iterator for RectIter {
    type Item = Pos;
    fn next(&mut self) -> Option<Pos> {
        self.pos.x += 1;
        if self.pos.x > self.rect.x2 {
            self.pos.x = self.rect.x1;
            self.pos.y += 1;
        }
        if self.pos.y > self.rect.y2 {
            None
        } else {
            Some(self.pos)
        }
    }
}
