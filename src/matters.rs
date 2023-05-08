use std::cmp::Ordering;

use crate::{
    matrix::Matrix,
    ray::Ray,
    vector::{Point, Vec4},
};

pub mod camera;
pub mod light;
pub mod material;
pub mod sphere;

pub trait Shape: Default + Clone {
    // create a shape with a transformation
    fn new(transformation_matrix: Matrix) -> Self {
        let mut shape = Self::default();
        shape.set_transformation(transformation_matrix);
        shape
    }
    //transform the ray from world space coordinate to object space coordinate by appling inverse of shape transformation (self.transformation) to the ray.
    // intersects a Shape with ray
    fn intersect(&self, ray: &Ray) -> Vec<Intersection<Self>>;
    // finds the normal at a point on a shape
    fn normal_at(&self, point: &Point) -> Vec4;
    // set transformation to the shape
    fn set_transformation(&mut self, transformation_matrix: Matrix);
}

#[derive(Debug, Clone)]
pub struct Intersection<T: Clone> {
    pub distance: f64,
    pub object: T,
}

pub struct PrerareComputation<T: Clone> {
    // distance at which ray intersected
    pub distance: f64,
    // object which is intersected
    pub object: T,
    // point at which ray intersected
    pub point: Point,
    // opposite of ray direction (eye vector)
    pub eyev: Vec4,
    // normal at intersection point
    pub normalv: Vec4,
    // if a normal is inside object or outside
    pub inside: bool,
    // for shadows
    pub over_point: Point,
}

pub trait Intersectable: Clone {
    fn hits(intersections: &[Intersection<Self>]) -> Option<Intersection<Self>>;

    fn prepare_computation(
        intersection: &Intersection<Self>,
        ray: &Ray,
    ) -> PrerareComputation<Self>;
}

impl<T: Clone> Intersection<T> {
    pub fn new(distance: f64, object: T) -> Self {
        Self { distance, object }
    }
}

impl<T: Clone> Ord for Intersection<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance == other.distance {
            Ordering::Equal
        } else if self.distance < other.distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl<T: Clone> PartialOrd for Intersection<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.distance == other.distance {
            Some(Ordering::Equal)
        } else if self.distance < other.distance {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<T: Clone> PartialEq for Intersection<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T: Clone> Eq for Intersection<T> {}
