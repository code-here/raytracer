use crate::vector::{Point, Vec4};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec4,
}

impl AsRef<Ray> for Ray {
    fn as_ref(&self) -> &Ray {
        self
    }
}

impl Ray {
    pub fn new(origin: Point, direction: Vec4) -> Self {
        Self { origin, direction }
    }
    // t -> time, distance, position of ray at some time, distance t
    pub fn position(&self, t: f64) -> Point {
        // multiply the direction vector with distance t and then add the point to vector
        &self.origin + &(t * &self.direction)
    }
}
