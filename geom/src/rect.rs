use std::fmt::Debug;

use crate::{GeomUnit, NonNegativeFloat, NonNegativeGeomUnit, Point, Size};

#[derive(Debug, Copy, Clone)]
pub struct Rect<T, U> {
    position: Point<T>,
    size: Size<U>,
}

impl<T: GeomUnit, U: NonNegativeGeomUnit> Rect<T, U> {
    pub fn new(position: Point<T>, size: Size<U>) -> Self {
        Rect { position, size }
    }
}

impl<T: Copy, U> Rect<T, U> {
    pub fn position(&self) -> Point<T> {
        self.position
    }
}

impl<T, U: Copy> Rect<T, U> {
    pub fn size(&self) -> Size<U> {
        self.size
    }
}

pub type IntRect = Rect<i32, u32>;
pub type FloatRect = Rect<f32, NonNegativeFloat>;
