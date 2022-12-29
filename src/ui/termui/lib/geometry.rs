use std::ops::{Add, AddAssign};

/**
 * The X-Y location of something on the screen.
 */
#[derive(Clone, Copy)]
pub struct Loc {
    pub x: u16,
    pub y: u16,
}

impl Loc {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl Add<Self> for Loc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

/**
 * The size of something on the screen.
 */
#[derive(Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl Add<Self> for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl AddAssign<Self> for Size {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}
