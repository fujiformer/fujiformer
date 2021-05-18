use std::fmt::Debug;

use crate::{NonNegativeFloat, NonNegativeGeomUnit};

#[derive(Copy, Clone)]
pub struct Size<T> {
    width: T,
    height: T,
}

impl<T: Debug> Debug for Size<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}x{:?}", self.width, self.height)
    }
}

impl<T: NonNegativeGeomUnit> Size<T> {
    pub fn new(x: T, y: T) -> Self {
        Size {
            width: x,
            height: y,
        }
    }

    pub fn width(self) -> T {
        self.width
    }

    pub fn height(self) -> T {
        self.height
    }
}

pub type IntSize = Size<u32>;
pub type FloatSize = Size<NonNegativeFloat>;
