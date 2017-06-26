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
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
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
        Line { x1: x1, y1: y1, x2: x2, y2: y2 }
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
    type IntoIter = LineIter;
    fn into_iter(self) -> LineIter {
        LineIter::init(self)
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

pub struct LineIter {
    start: (i32, i32),
    end: (i32, i32),
    delta: (f32, f32),
    error: (f32, f32),
    done: bool,
}

impl LineIter {
    pub fn init(line: Line) -> Self {
        let start = (min(line.x1, line.x2), min(line.y1, line.y2));
        let end = (max(line.x1, line.x2), max(line.y1, line.y2));
        let delta = (end.0 as f32 - start.0 as f32,
                     end.1 as f32 - start.1 as f32);
        let error;
        if delta.1 == 0.0 {
            error = (1.0, 0.0);
        } else if delta.0 == 0.0 {
            error = (0.0, 1.0);
        } else {
            error = ((delta.0 as f32 / delta.1 as f32).abs(),
                    (delta.1 as f32 / delta.0 as f32).abs());
        }
        LineIter {
            start: start,
            end: end,
            delta: error, // yes, that is correct ;)
            error: (error.0 - 0.5, error.1 - 0.5),
            done: false,
        }
    }

}

impl Iterator for LineIter {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<(i32, i32)> {
        if self.done {
            return None;
        }

        let result = Some(self.start);
        if self.start.0 >= self.end.0 && self.start.1 >= self.end.1 {
            self.done = true;
        } else {
            self.error.0 += self.delta.0;
            if self.error.0 >= 0.5 {
                self.start.0 += 1;
                self.error.0 -= 1.0;
            }
            self.error.1 += self.delta.1;
            if self.error.1 >= 0.5 {
                self.start.1 += 1;
                self.error.1 -= 1.0;
            }
        }
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
    fn straight_lines() {
        assert_equals(Line::new(0,0,1,0).into_iter().count(), 2);
        assert_equals(Line::new(0,0,2,0).into_iter().count(), 3);
        assert_equals(Line::new(0,0,9,0).into_iter().count(), 10);
        assert_equals(Line::new(0,0,0,1).into_iter().count(), 2);
        assert_equals(Line::new(0,0,0,2).into_iter().count(), 3);
        assert_equals(Line::new(0,0,0,9).into_iter().count(), 10);
    }

    #[test]
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
        let mut iter = Line::new(0,0,10,7).into_iter();
        assert_equals_pos(iter.next().unwrap(), (0, 0));
        assert_equals_pos(iter.next().unwrap(), (1, 1));
        assert_equals_pos(iter.next().unwrap(), (2, 2));
        assert_equals_pos(iter.next().unwrap(), (3, 2));
        assert_equals_pos(iter.next().unwrap(), (4, 3));
        assert_equals_pos(iter.next().unwrap(), (5, 4));
        assert_equals_pos(iter.next().unwrap(), (6, 4));
        assert_equals_pos(iter.next().unwrap(), (7, 5));
        assert_equals_pos(iter.next().unwrap(), (8, 6));
        assert_equals_pos(iter.next().unwrap(), (9, 6));
        assert_equals_pos(iter.next().unwrap(), (10, 7));
    }

    #[test]
    fn steep_lines() {
        assert_equals(Line::new(0,0,7,10).into_iter().count(), 11);
        let mut iter = Line::new(0,0,7,10).into_iter();
        assert_equals_pos(iter.next().unwrap(), (0, 0));
        assert_equals_pos(iter.next().unwrap(), (1, 1));
        assert_equals_pos(iter.next().unwrap(), (2, 2));
        assert_equals_pos(iter.next().unwrap(), (2, 3));
        assert_equals_pos(iter.next().unwrap(), (3, 4));
        assert_equals_pos(iter.next().unwrap(), (4, 5));
        assert_equals_pos(iter.next().unwrap(), (4, 6));
        assert_equals_pos(iter.next().unwrap(), (5, 7));
        assert_equals_pos(iter.next().unwrap(), (6, 8));
        assert_equals_pos(iter.next().unwrap(), (6, 9));
        assert_equals_pos(iter.next().unwrap(), (7, 10));
    }
}
