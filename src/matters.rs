use std::{cmp::Ordering, io::Write};

use crate::{
    canvas::{Canvas, Color},
    matrix::Matrix,
    ray::Ray,
    vector::{Point, Vec4},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    // it will always be origin
    origin: Point,
    // identity matrix as default transformation, it can be changed so making it public
    pub transformation: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection<Sphere>> {
        //transform the ray from world space coordinate to object space coordinate by appling inverse of sphere transformation to the ray
        let transformed_ray = ray.transform(self.transformation.inverse_4x4().unwrap());
        //because there is gona be 2 point at most
        let mut intersections = Vec::with_capacity(2);
        // vector from sphere center to ray
        let sphere_to_ray = transformed_ray.origin.as_ref() - self.origin.as_ref();
        // finding discriminant
        let a = transformed_ray
            .direction
            .as_ref()
            .dot(transformed_ray.direction.as_ref());
        let b = 2.0
            * transformed_ray
                .direction
                .as_ref()
                .dot(sphere_to_ray.as_ref());
        let c = sphere_to_ray.as_ref().dot(sphere_to_ray.as_ref()) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        // if discriminant < 0 the not intersection
        // if discriminant = 0 one intersection
        // if discriminant > 0 two discriminants
        if discriminant >= 0.0 {
            // send the same point twice even if one intraction
            intersections.push(Intersection::new(
                (-b - discriminant.sqrt()) / (2.0 * a),
                self.clone(),
            ));
            intersections.push(Intersection::new(
                (-b + discriminant.sqrt()) / (2.0 * a),
                self.clone(),
            ));
        }
        intersections
    }

    pub fn set_transformation(&mut self, transformation_matrix: Matrix) {
        self.transformation = transformation_matrix;
    }

    pub fn simple_sphere_to_canvas(&self) {
        let mut canvas = Canvas::new(300, 300);
        let red = Color::new(1.0, 0.0, 0.0);
        let black = Color::black();
        let torch = Point::new(0.0, 0.0, -5.0);
        let wall = Wall {
            z: 5.0,
            width: 8.0,
            height: 8.0,
        };
        for r in 0..canvas.height {
            let world_x = wall.width / 2.0 - (wall.width / canvas.width as f64) * r as f64;
            for c in 0..canvas.width {
                // get the point on the wall, then get the ray from torch to that point on wall
                let point_on_wall = Point::new(
                    world_x,
                    wall.height / 2.0 - (wall.height / canvas.height as f64) * c as f64,
                    wall.z,
                );
                // ray from torch to point on wall, origin is the torch and vector is from torch to point on wall
                let ray = Ray::new(torch.clone(), (point_on_wall - torch.clone()).normalize());
                let intersections = self.intersect(&ray);
                if intersections.is_empty() {
                    canvas.write_pixel((r as usize, c as usize), &black);
                } else {
                    if Intersection::hits(&intersections).is_some() {
                        canvas.write_pixel((r as usize, c as usize), &red);
                    }
                }
            }
        }
        let path = std::path::Path::new(".\\sphere.ppm");
        if !path.exists() {
            std::fs::File::create(&path).unwrap();
        }
        let mut file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
        file.write_all(canvas.to_ppm().as_bytes()).unwrap();
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            origin: Point::new(0.0, 0.0, 0.0),
            transformation: Matrix::identity_4x4(),
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

pub struct Wall {
    z: f64,
    width: f64,
    height: f64,
}
