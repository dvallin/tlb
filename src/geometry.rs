use std::cmp::{ min, max };

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub trait Shape: Copy + IntoIterator<Item=Pos> {
    fn center(&self) -> Pos;
    fn bounding_box(&self) -> Rect;

    fn is_enclosed(&self, pos: Pos) -> bool;
    fn is_boundary(&self, pos: Pos) -> bool;
    fn is_interior(&self, pos: Pos) -> bool;
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

impl Shape for Rect {
    fn center(&self) -> Pos {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        Pos { x: center_x, y: center_y }
    }

    fn bounding_box(&self) -> Rect {
        *self
    }

    fn is_enclosed(&self, pos: Pos) -> bool {
        pos.x >= self.x1 && pos.x < self.x2 && pos.y >= self.y1 && pos.y < self.y2
    }

    fn is_boundary(&self, pos: Pos) -> bool {
        pos.x == self.x1 || pos.x == self.x2 - 1 || pos.y == self.y1 || pos.y == self.y2 - 1
    }

    fn is_interior(&self, pos: Pos) -> bool {
        pos.x > self.x1 && pos.x < self.x2 - 1 && pos.y > self.y1 && pos.y < self.y2 - 1
    }
}

impl IntoIterator for Line {
    type Item = Pos;
    type IntoIter = LineIter;
    fn into_iter(self) -> LineIter {
        LineIter::init(self)
    }
}

impl IntoIterator for Rect {
    type Item = Pos;
    type IntoIter = RectIter;
    fn into_iter(self) -> RectIter {
        RectIter { rect: self, pos: Pos { x: self.x1 - 1, y: self.y1 } }
    }
}

pub struct LineIter {
    p0: Pos,
    p1: Pos,
    dx: f32,
    dy: f32,
    done: bool,
    ex: f32,
    ey: f32,
}

impl LineIter {
    pub fn init(line: Line) -> Self {
        let p0 = Pos { x: min(line.x1, line.x2), y: min(line.y1, line.y2) };
        let p1 = Pos { x: max(line.x1, line.x2), y: max(line.y1, line.y2) };
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let dxe;
        let dye;
        if dy == 0 {
            dye = 0.0;
            dxe = 1.0;
        } else if dx == 0 {
            dye = 1.0;
            dxe = 0.0;
        } else {
            dxe = (dx as f32 / dy as f32).abs();
            dye = (dy as f32 / dx as f32).abs();
        }
        LineIter {
            p0: p0,
            p1: p1,
            dx: dxe,
            dy: dye,
            done: false,
            ex: dxe - 0.5,
            ey: dye - 0.5,
        }
    }

}

impl Iterator for LineIter {
    type Item = Pos;
    fn next(&mut self) -> Option<Pos> {
        if self.done {
            return None;
        }

        let result = Some(self.p0);
        if self.p0.x >= self.p1.x && self.p0.y >= self.p1.y {
            self.done = true;
        } else {
            self.ex += self.dx;
            if self.ex >= 0.5 {
                self.p0.x += 1;
                self.ex -= 1.0;
            }
            self.ey += self.dy;
            if self.ey >= 0.5 {
                self.p0.y += 1;
                self.ey -= 1.0;
            }
        }
        return result;
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
        if self.pos.x >= self.rect.x2 {
            self.pos.x = self.rect.x1;
            self.pos.y += 1;
        }
        if self.pos.y >= self.rect.y2 {
            None
        } else {
            Some(self.pos)
        }
    }
}
#[cfg(test)]
mod tests {
    use geometry::{ Line, Pos };
    use std::fmt::{ Display };

    fn assert_equals<T>(a: T, b: T)
        where T: PartialEq<T> + Display {
        assert!(a == b, "{} is not {}", a, b);
    }

    fn assert_equals_pos(a: &Pos, b: &Pos) {
        assert_equals(a.x, b.x);
        assert_equals(a.y, b.y);
    }

    #[test]
    fn single_pixel_lines() {
        assert!(Line::new(0,0,0,0).into_iter().count() == 1);
        let p = Line::new(0,0,0,0).into_iter().next().unwrap();
        assert!(p.x == 0 && p.y == 0);
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
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 0, y: 0});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 1, y: 1});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 2, y: 2});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 3, y: 2});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 4, y: 3});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 5, y: 4});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 6, y: 4});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 7, y: 5});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 8, y: 6});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 9, y: 6});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 10, y: 7});
    }

    #[test]
    fn steep_lines() {
        assert_equals(Line::new(0,0,7,10).into_iter().count(), 11);
        let mut iter = Line::new(0,0,7,10).into_iter();
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 0, y: 0});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 1, y: 1});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 2, y: 2});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 2, y: 3});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 3, y: 4});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 4, y: 5});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 4, y: 6});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 5, y: 7});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 6, y: 8});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 6, y: 9});
        assert_equals_pos(&iter.next().unwrap(), &Pos{ x: 7, y: 10});
    }
}
