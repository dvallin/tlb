use components::space::{ Vector, Position };
use std::cmp::{ min, max };

pub trait Shape: Copy + IntoIterator<Item=(i32, i32)> {
    fn center(&self) -> (i32, i32);
    fn bounding_box(&self) -> Rect;

    fn is_enclosed(&self, pos: (i32, i32)) -> bool;
    fn is_boundary(&self, pos: (i32, i32)) -> bool;
    fn is_interior(&self, pos: (i32, i32)) -> bool;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line {
    p1: (i32, i32),
    p2: (i32, i32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    p1: (i32, i32),
    p2: (i32, i32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Triangle {
    p1: (i32, i32),
    p2: (i32, i32),
    p3: (i32, i32),
}

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        assert!(x1 != y1 || x2 != y2);
        Line { p1: (x1, y1), p2: (x2, y2) }
    }
}

impl Ray {
    pub fn new(p1: (i32, i32), p2: (i32, i32)) -> Self {
        assert!(p1.0 != p2.0 || p1.1 != p2.1);
        Ray { p1: p1, p2: p2 }
    }
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

    pub fn center_at(self: &mut Rect, x: i32, y: i32) {
        let w = self.x2 - self.x1;
        let h = self.y2 - self.y1;
        self.x1 = x - w/2;
        self.x2 = self.x1 + w;
        self.y1 = y - h/2;
        self.y2 = self.y1 + h;
    }

    pub fn grow(self: &Rect, s: i32) -> Self {
        Rect {
            x1: self.x1 - s,
            x2: self.x2 + s,
            y1: self.y1 - s,
            y2: self.y2 + s,
        }
    }

    pub fn top(self: &Rect) -> i32 {
        self.y1
    }

    pub fn left(self: &Rect) -> i32 {
        self.x1
    }

    pub fn bottom(self: &Rect) -> i32 {
        self.y2 - 1
    }

    pub fn right(self: &Rect) -> i32 {
        self.x2 - 1
    }
}

impl Triangle {
    pub fn new(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> Self {
        Triangle { p1: p1, p2: p2, p3: p3 }
    }
}

impl Shape for Rect {
    fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    fn bounding_box(&self) -> Rect {
        *self
    }

    fn is_enclosed(&self, pos: (i32, i32)) -> bool {
        pos.0 >= self.x1 && pos.0 < self.x2 && pos.1 >= self.y1 && pos.1 < self.y2
    }

    fn is_boundary(&self, pos: (i32, i32)) -> bool {
        pos.0 == self.x1 || pos.0 == self.x2 - 1 || pos.1 == self.y1 || pos.1 == self.y2 - 1
    }

    fn is_interior(&self, pos: (i32, i32)) -> bool {
        pos.0 > self.x1 && pos.0 < self.x2 - 1 && pos.1 > self.y1 && pos.1 < self.y2 - 1
    }
}

impl Shape for Triangle {
    fn center(&self) -> (i32, i32) {
        let center_x = (self.p1.0 + self.p2.0 + self.p3.0) / 3;
        let center_y = (self.p1.1 + self.p2.1 + self.p3.1) / 3;
        (center_x, center_y)
    }

    fn bounding_box(&self) -> Rect {
        Rect {
            x1: min(min(self.p1.0, self.p2.0), self.p3.0),
            y1: min(min(self.p1.1, self.p2.1), self.p3.1),
            x2: max(max(self.p1.0, self.p2.0), self.p3.0),
            y2: max(max(self.p1.1, self.p2.1), self.p3.1),
        }
    }

    fn is_enclosed(&self, pos: (i32, i32)) -> bool {
        let p1 = Position { x: self.p1.0 as f32, y: self.p1.1 as f32 };
        let p2 = Position { x: self.p2.0 as f32, y: self.p2.1 as f32 };
        let p3 = Position { x: self.p3.0 as f32, y: self.p3.1 as f32 };
        let ps = Position { x: pos.0 as f32, y: pos.1 as f32 };
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let v3 = ps - p1;
        let d11 = v1.dot(&v1);
        let d12 = v1.dot(&v2);
        let d13 = v1.dot(&v3);
        let d22 = v2.dot(&v2);
        let d23 = v2.dot(&v3);

        let inv_denominator = 1.0 / (d11 * d22 - d12 * d12);
        let u = (d22 * d13 - d12 * d23) * inv_denominator;
        let v = (d11 * d23 - d12 * d13) * inv_denominator;

        u >= 0.0 && v >= 0.0 && (u + v) < 1.0
    }

    fn is_boundary(&self, pos: (i32, i32)) -> bool {
        false
    }

    fn is_interior(&self, pos: (i32, i32)) -> bool {
        self.is_enclosed(pos)
    }
}

impl IntoIterator for Triangle {
    type Item = (i32, i32);
    type IntoIter = ShapeIter<Triangle>;
    fn into_iter(self) -> ShapeIter<Triangle> {
        ShapeIter { shape: Box::new(self), rect: self.bounding_box().into_iter() }
    }
}

impl IntoIterator for Line {
    type Item = (i32, i32);
    type IntoIter = BresenhamIter;
    fn into_iter(self) -> BresenhamIter {
        BresenhamIter::init(self.p1, self.p2, false)
    }
}

impl IntoIterator for Ray {
    type Item = (i32, i32);
    type IntoIter = BresenhamIter;
    fn into_iter(self) -> BresenhamIter {
        BresenhamIter::init(self.p1, self.p2, true)
    }
}

impl IntoIterator for Rect {
    type Item = (i32, i32);
    type IntoIter = RectIter;
    fn into_iter(self) -> RectIter {
        RectIter { rect: self, pos: (self.x1 - 1, self.y1) }
    }
}

pub struct ShapeIter<T> where T: Shape {
    shape: Box<T>,
    rect: RectIter,
}

impl<T> Iterator for ShapeIter<T> where T: Shape {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<(i32, i32)> {
        let mut result = None;
        loop {
            if let Some(next) = self.rect.next() {
                if self.shape.is_enclosed(next) {
                    result = Some(next);
                    break;
                }
            } else {
                break;
            }
        };
        result
    }
}

pub struct BresenhamIter {
    index: i32,
    start: (i32, i32),
    sign: (i32, i32),
    delta: (f32, f32),
    swap: bool,
    d: f32,
    overshoot: bool,
    done: bool,
}

impl BresenhamIter {
    pub fn init(p1: (i32, i32), p2: (i32, i32), overshoot: bool) -> Self {
        let mut delta = ((p2.0 as f32 - p1.0 as f32).abs(),
                     (p2.1 as f32 - p1.1 as f32).abs());
        let sign = ((p2.0 - p1.0).signum(), (p2.1 - p1.1).signum());
        let swap;
        if delta.1 > delta.0 {
            delta = (delta.1, delta.0);
            swap = true;
        } else {
            swap = false;
        }
        let d = 2.0 * delta.1 - delta.0;
        BresenhamIter {
            index: 0,
            start: p1,
            sign: sign,
            delta: delta,
            d: d,
            swap: swap,
            done: false,
            overshoot: overshoot,
        }
    }

}

impl Iterator for BresenhamIter {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<(i32, i32)> {
        if self.done {
            return None;
        }

        let result = Some(self.start);
        if !self.overshoot && self.index as f32 >= self.delta.0 {
            self.done = true;
        } else {
            while self.d >= 0.0 {
                self.d -= 2.0 * self.delta.0;
                if self.swap {
                    self.start.0 += self.sign.0;
                } else {
                    self.start.1 += self.sign.1;
                }
            }
            self.d += 2.0 * self.delta.1;
            if self.swap {
                self.start.1 += self.sign.1;
            } else {
                self.start.0 += self.sign.0;
            }
        }
        self.index += 1;
        return result;
    }
}

pub struct RectIter {
    rect: Rect,
    pos: (i32, i32),
}

impl Iterator for RectIter {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<(i32, i32)> {
        self.pos.0 += 1;
        if self.pos.0 >= self.rect.x2 {
            self.pos.0 = self.rect.x1;
            self.pos.1 += 1;
        }
        if self.pos.1 >= self.rect.y2 {
            None
        } else {
            Some(self.pos)
        }
    }
}
#[cfg(test)]
mod tests {
    use geometry::{ Line };
    use std::fmt::{ Display };

    fn assert_equals<T>(a: T, b: T)
        where T: PartialEq<T> + Display {
        assert!(a == b, "{} is not {}", a, b);
    }

    fn assert_equals_pos(a: (i32, i32), b: (i32, i32)) {
        assert_equals(a.0, b.0);
        assert_equals(a.1, b.1);
    }

    #[test]
    fn single_pixel_lines() {
        assert!(Line::new(0,0,0,0).into_iter().count() == 1);
        let p = Line::new(0,0,0,0).into_iter().next().unwrap();
        assert!(p.0 == 0 && p.1 == 0);
    }
    #[test]
    fn all_directions_lines() {
        assert_equals_pos(Line::new(0,0, 1, 0).into_iter().nth(1).unwrap(), (1, 0));
        assert_equals_pos(Line::new(0,0, 1,-1).into_iter().nth(1).unwrap(), (1, -1));
        assert_equals_pos(Line::new(0,0, 0,-1).into_iter().nth(1).unwrap(), (0, -1));
        assert_equals_pos(Line::new(0,0,-1,-1).into_iter().nth(1).unwrap(), (-1, -1));
        assert_equals_pos(Line::new(0,0,-1, 0).into_iter().nth(1).unwrap(), (-1, 0));
        assert_equals_pos(Line::new(0,0,-1, 1).into_iter().nth(1).unwrap(), (-1, 1));
        assert_equals_pos(Line::new(0,0, 0, 1).into_iter().nth(1).unwrap(), (0, 1));
        assert_equals_pos(Line::new(0,0, 1, 1).into_iter().nth(1).unwrap(), (1, 1));
    }

    #[test]
    fn straight_lines() {
        assert_equals(Line::new(0,0,1,0).into_iter().count(), 2);
        assert_equals(Line::new(0,0,2,0).into_iter().count(), 3);
        assert_equals(Line::new(0,0,9,0).into_iter().count(), 10);
        assert_equals(Line::new(0,0,0,1).into_iter().count(), 2);
        assert_equals(Line::new(0,0,0,2).into_iter().count(), 3);
        assert_equals(Line::new(0,0,0,9).into_iter().count(), 10);
    }

    fn diagonal_lines() {
        assert_equals(Line::new(0,0,1,1).into_iter().count(), 2);
        assert_equals(Line::new(0,0,2,2).into_iter().count(), 3);
        assert_equals(Line::new(0,0,9,9).into_iter().count(), 10);
        assert_equals(Line::new(0,0,-1,1).into_iter().count(), 2);
        assert_equals(Line::new(0,0,-2,2).into_iter().count(), 3);
        assert_equals(Line::new(0,0,-9,9).into_iter().count(), 10);
    }

    #[test]
    fn flat_lines() {
        assert_equals(Line::new(0,0,10,7).into_iter().count(), 11);
    }

    #[test]
    fn steep_lines() {
        assert_equals(Line::new(0,0,7,10).into_iter().count(), 11);
    }
}
