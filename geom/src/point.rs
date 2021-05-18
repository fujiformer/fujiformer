use std::fmt::Debug;

use crate::GeomUnit;

#[derive(Copy, Clone)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T: Debug> Debug for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<T: GeomUnit> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn x(self) -> T {
        self.x
    }

    pub fn y(self) -> T {
        self.y
    }
}

pub type IntPoint = Point<i32>;
pub type FloatPoint = Point<f32>;
