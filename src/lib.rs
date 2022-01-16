
/// The Rect struct represents an area, such as an area of the screen,
/// by its minimum x and y and maximum x and y.
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub minx: f32,
    pub miny: f32,
    pub maxx: f32,
    pub maxy: f32,
}

impl Rect {
    /// Create a new Rect.
    pub fn new(minx: f32, miny: f32, maxx: f32, maxy: f32) -> Rect {
        return Rect { minx, miny, maxx, maxy };
    }

    /// Cut out the left of the rect, returning the left piece and modifying the original Rect.
    pub fn cut_left(&mut self, a: f32) -> Rect {
        let minx: f32 = self.minx;
        if self.maxx < self.minx + a {
            self.minx = self.maxx;
        } else {
            self.minx = self.minx + a;
        }
        return Rect::new(minx, self.miny, self.minx, self.maxy);
    }

    /// Cut out the right of the rect, returning the right piece and modifying the original Rect.
    pub fn cut_right(&mut self, a: f32) -> Rect {
        let maxx: f32 = self.maxx;
        if self.minx > self.maxx - a {
            self.maxx = self.minx;
        } else {
            self.maxx = self.maxx - a;
        }
        return Rect::new(self.maxx, self.miny, maxx, self.maxy);
    }

    /// Cut out the top of the rect, returning the top piece and modifying the original Rect.
    pub fn cut_top(&mut self, a: f32) -> Rect {
        let miny: f32 = self.miny;
        if self.maxy < self.miny + a {
            self.miny = self.maxy;
        } else {
            self.miny = self.miny + a;
        }
        return Rect::new(self.minx, miny, self.maxx, self.miny);
    }

    /// Cut out the bottom of the rect, returning the bottom piece and modifying the original Rect.
    pub fn cut_bottom(&mut self, a: f32) -> Rect {
        let maxy: f32 = self.maxy;
        if self.miny > self.maxy - a {
            self.maxy = self.miny;
        } else {
            self.maxy = self.maxy - a;
        }
        return Rect::new(self.minx, self.maxy, self.maxx, maxy);
    }

    /// Cut out the left of the rect, leaving the original unmodified.
    pub fn get_left(&self, a: f32) -> Rect {
        let maxx;
        if self.maxx < self.minx + a {
            maxx = self.maxx;
        } else {
            maxx = self.minx + a;
        }
        return Rect::new(self.minx, self.miny, maxx, self.maxy);
    }

    /// Cut out the right of the rect, leaving the original unmodified.
    pub fn get_right(&self, a: f32) -> Rect {
        let minx;
        if self.minx > self.maxx - a {
            minx = self.minx;
        } else {
            minx = self.maxx - a;
        }
        return Rect::new(minx, self.miny, self.maxx, self.maxy);
    }

    /// Cut out the top of the rect, leaving the original unmodified.
    pub fn get_top(&self, a: f32) -> Rect {
        let maxy;
        if self.maxy < self.miny + a {
            maxy = self.maxy;
        } else {
            maxy = self.miny + a;
        }
        return Rect::new(self.minx, self.miny, self.maxx, maxy);
    }

    /// Cut out the bottom of the rect, leaving the original unmodified.
    pub fn get_bottom(&self, a: f32) -> Rect {
        let miny;
        if self.miny > self.maxy - a {
            miny = self.miny;
        } else {
            miny = self.maxy - a;
        }
        return Rect::new(self.minx, miny, self.maxx, self.maxy);
    }

    /// Create a rect that extends the given rect out to the left,
    /// leaving the original unmodified.
    pub fn add_left(&self, a: f32) -> Rect {
        return Rect::new(self.minx - a, self.miny, self.minx, self.maxy);
    }

    /// Create a rect that extends the given rect out to the right,
    /// leaving the original unmodified.
    pub fn add_right(&self, a: f32) -> Rect {
        return Rect::new(self.maxx, self.miny, self.maxx + a, self.maxy);
    }

    /// Create a rect that extends the given rect out to the top,
    /// leaving the original unmodified.
    pub fn add_top(&self, a: f32) -> Rect {
        return Rect::new(self.minx, self.miny - a, self.maxx, self.miny);
    }

    /// Create a rect that extends the given rect out to the bottom,
    /// leaving the original unmodified.
    pub fn add_bottom(&self, a: f32) -> Rect {
        return Rect::new(self.minx, self.maxy, self.maxx, self.maxy + a);
    }

    /// Extend the rect out in all directions, leaving the original unmodified.
    pub fn extend(&self, a: f32) -> Rect {
        return Rect::new(self.minx - a, self.miny - a, self.maxx + a, self.maxy + a);
    }

    /// Conctract the rect in all directions, leaving the original unmodified.
    pub fn contract(&self, a: f32) -> Rect {
        return Rect::new(self.minx + a, self.miny + a, self.maxx - a, self.maxy - a);
    }
}

#[test]
pub fn test_cut_left() {
    let mut rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let left = rect.cut_left(1.0);

    assert_eq!(0.0, left.minx);
    assert_eq!(1.0, left.maxx);
    assert_eq!(rect.miny, left.miny);
    assert_eq!(rect.maxy, left.maxy);

    assert_eq!(1.0, rect.minx);
    assert_eq!(10.0, rect.maxx);
    assert_eq!(0.0, rect.miny);
    assert_eq!(10.0, rect.maxy);
}

#[test]
pub fn test_cut_right() {
    let mut rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let right = rect.cut_right(1.0);

    assert_eq!(9.0, right.minx);
    assert_eq!(10.0, right.maxy);
    assert_eq!(rect.miny, right.miny);
    assert_eq!(rect.maxy, right.maxy);

    assert_eq!(0.0, rect.minx);
    assert_eq!(9.0, rect.maxx);
    assert_eq!(0.0, rect.miny);
    assert_eq!(10.0, rect.maxy);
}

#[test]
pub fn test_cut_top() {
    let mut rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let top = rect.cut_top(1.0);

    assert_eq!(0.0, top.minx);
    assert_eq!(10.0, top.maxx);
    assert_eq!(0.0, top.miny);
    assert_eq!(1.0, top.maxy);

    assert_eq!(0.0, rect.minx);
    assert_eq!(10.0, rect.maxx);
    assert_eq!(1.0, rect.miny);
    assert_eq!(10.0, rect.maxy);
}

#[test]
pub fn test_cut_bottom() {
    let mut rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let bottom = rect.cut_bottom(1.0);

    assert_eq!(0.0, bottom.minx);
    assert_eq!(10.0, bottom.maxx);
    assert_eq!(9.0, bottom.miny);
    assert_eq!(10.0, bottom.maxy);

    assert_eq!(0.0, rect.minx);
    assert_eq!(10.0, rect.maxx);
    assert_eq!(0.0, rect.miny);
    assert_eq!(9.0, rect.maxy);
}

#[test]
pub fn test_get_left() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let left = rect.get_left(1.0);

    assert_eq!(0.0, left.minx);
    assert_eq!(1.0, left.maxx);
    assert_eq!(0.0, left.miny);
    assert_eq!(10.0, left.maxy);
}

#[test]
pub fn test_get_right() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let right = rect.get_right(1.0);

    assert_eq!(9.0, right.minx);
    assert_eq!(10.0, right.maxx);
    assert_eq!(0.0, right.miny);
    assert_eq!(10.0, right.maxy);
}

#[test]
pub fn test_get_top() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let top = rect.get_top(1.0);

    assert_eq!(0.0, top.minx);
    assert_eq!(10.0, top.maxx);
    assert_eq!(0.0, top.miny);
    assert_eq!(1.0, top.maxy);
}

#[test]
pub fn test_get_bottom() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let bottom = rect.get_bottom(1.0);

    assert_eq!(0.0, bottom.minx);
    assert_eq!(10.0, bottom.maxx);
    assert_eq!(9.0, bottom.miny);
    assert_eq!(10.0, bottom.maxy);
}

#[test]
pub fn test_add_left() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let left = rect.add_left(1.0);

    assert_eq!(-1.0, left.minx);
    assert_eq!(0.0, left.maxx);
    assert_eq!(0.0, left.miny);
    assert_eq!(10.0, left.maxy);
}

#[test]
pub fn test_add_right() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let right = rect.add_right(1.0);

    assert_eq!(10.0, right.minx);
    assert_eq!(11.0, right.maxx);
    assert_eq!(0.0, right.miny);
    assert_eq!(10.0, right.maxy);
}

#[test]
pub fn test_add_top() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let top = rect.add_top(1.0);

    assert_eq!(0.0, top.minx);
    assert_eq!(10.0, top.maxx);
    assert_eq!(-1.0, top.miny);
    assert_eq!(0.0, top.maxy);
}

#[test]
pub fn test_add_bottom() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let bottom = rect.add_bottom(1.0);

    assert_eq!(0.0, bottom.minx);
    assert_eq!(10.0, bottom.maxx);
    assert_eq!(10.0, bottom.miny);
    assert_eq!(11.0, bottom.maxy);
}

#[test]
pub fn test_extend() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);

    let extended = rect.extend(1.0);

    assert_eq!(-1.0, extended.minx);
    assert_eq!(11.0, extended.maxx);
    assert_eq!(-1.0, extended.miny);
    assert_eq!(11.0, extended.maxy);
}

#[test]
pub fn test_contract() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);

    let contract = rect.contract(1.0);

    assert_eq!(1.0, contract.minx);
    assert_eq!(9.0, contract.maxx);
    assert_eq!(1.0, contract.miny);
    assert_eq!(9.0, contract.maxy);
}

/// A RectCutSide represents a side of the rectangle. This allows
/// the user to choose a side dynamically using a RectCut.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum RectCutSide {
    Left,
    Right,
    Top,
    Bottom,
}

/// A RectCut wraps a Rect along with a side. This allows the
/// user to fix the side and pass the rect-and-side together
/// into other code.
#[derive(Copy, Clone, Debug)]
struct RectCut {
    pub rect: Rect,
    pub side: RectCutSide,
}

impl RectCut {
    /// Create a new RectCut.
    pub fn new(rect: Rect, side: RectCutSide) -> RectCut {
        return RectCut { rect, side };
    }

    /// Cut out from the RectCut, returning the new Rect
    /// and modifying the internal Rect.
    pub fn cut(&mut self, a: f32) -> Rect {
        match self.side {
            RectCutSide::Left =>   return self.rect.cut_left(a),
            RectCutSide::Right =>  return self.rect.cut_right(a),
            RectCutSide::Top =>    return self.rect.cut_top(a),
            RectCutSide::Bottom => return self.rect.cut_bottom(a),
        }
    }
}

#[test]
fn test_rectcut() {
    let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
    let mut rectcut = RectCut::new(rect, RectCutSide::Left);
    let left = rectcut.cut(1.0);
    assert_eq!(0.0, left.minx);
    assert_eq!(0.0, left.miny);
    assert_eq!(1.0, left.maxx);
    assert_eq!(10.0, left.maxy);

    assert_eq!(1.0, rectcut.rect.minx);
    assert_eq!(0.0, rectcut.rect.miny);
    assert_eq!(10.0, rectcut.rect.maxx);
    assert_eq!(10.0, rectcut.rect.maxy);
}

