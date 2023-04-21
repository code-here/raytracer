use std::cmp::Ordering;

use crate::{ray::Ray, vector::Point};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub origin: Point,
}

impl Sphere {
    pub fn new(origin: Point) -> Self {
        Self::default()
    }
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection<Sphere>> {
        //because there is gona be 2 point at most
        let mut intersections = Vec::with_capacity(2);
        // vector from sphere center to ray
        let sphere_to_ray = ray.origin.as_ref() - self.origin.as_ref();
        // finding discriminant
        let a = ray.direction.as_ref().dot(ray.direction.as_ref());
        let b = 2.0 * ray.direction.as_ref().dot(sphere_to_ray.as_ref());
        let c = sphere_to_ray.as_ref().dot(sphere_to_ray.as_ref()) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        // if discriminant < 0 the not intersection
        // if discriminant = 0 one intersection
        // if discriminant > 0 two discriminants
        if discriminant >= 0.0 {
            // send the same point twice even if one intraction
            intersections.push(Intersection::new(
                (-b - discriminant.sqrt()) / 2.0 * a,
                self.clone(),
            ));
            intersections.push(Intersection::new(
                (-b + discriminant.sqrt()) / 2.0 * a,
                self.clone(),
            ));
        }
        intersections
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            origin: Point::new(0.0, 0.0, 0.0),
        }
    }
}

impl Matter for Sphere {}

pub trait Matter: PartialEq {}

#[derive(Debug, Clone)]
pub struct Intersection<T: Matter + Clone> {
    pub distance: f64,
    pub object: T,
}

impl<T: Matter + Clone> Intersection<T> {
    pub fn new(distance: f64, object: T) -> Self {
        Self { distance, object }
    }
    pub fn hits(intersections: &[Intersection<T>]) -> Option<Intersection<T>> {
        // if distance is negative than the object is behind the ray so exclude those intersections in hits
        let mut positive_intersections = intersections
            .iter()
            .filter(|&i| i.distance >= 0.0)
            .collect::<Vec<&Intersection<T>>>();
        if positive_intersections.is_empty() {
            // if objects are not in front of ray
            None
        } else {
            positive_intersections.sort();
            Some(positive_intersections[0].clone())
        }
    }
}

impl<T: Matter + Clone> Ord for Intersection<T> {
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

impl<T: Matter + Clone> PartialOrd for Intersection<T> {
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

impl<T: Matter + Clone> PartialEq for Intersection<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T: Matter + Clone> Eq for Intersection<T> {}
