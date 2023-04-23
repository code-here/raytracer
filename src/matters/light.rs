use crate::{canvas::Color, vector::Point};

// a point light source
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
