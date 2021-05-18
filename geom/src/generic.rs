use std::fmt::Debug;

#[derive(Copy, Clone)]
pub struct NonNegativeFloat(f32);

impl Debug for NonNegativeFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}|", self.0)
    }
}

pub struct NegativeFloatError;

impl NonNegativeFloat {
    pub fn new(x: f32) -> Result<Self, NegativeFloatError> {
        if x.is_normal() && x >= 0.0 {
            Ok(NonNegativeFloat(x))
        } else {
            Err(NegativeFloatError)
        }
    }
}

pub trait GeomUnit {}

impl GeomUnit for f32 {}
impl GeomUnit for NonNegativeFloat {}
impl GeomUnit for i32 {}
impl GeomUnit for u32 {}

pub trait NonNegativeGeomUnit: GeomUnit {}

impl NonNegativeGeomUnit for NonNegativeFloat {}
impl NonNegativeGeomUnit for u32 {}
