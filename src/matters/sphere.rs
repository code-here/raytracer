use std::io::Write;

use crate::{
    canvas::{Canvas, Color},
    matrix::Matrix,
    ray::Ray,
    vector::{Point, Vec4},
};

use super::{light::Light, material::Material, Intersectable, Intersection, PrerareComputation};

/// NOTES:
/// 1. to bring some point/vector from world space to object space multiply the inverse of transformation matrix of object(sphere) with the point/vector i.e transformation.inverse() * point/vector
///2. to bring some point/vector from object space to world space,
/// normally we have to multiply the transformation matrix of object(sphere) with the point/vector i.e transformation * point/vector,
/// but sometime it does not work like in case of normals when a sphere is squased the normal is not from the center of sphere to point on surface so we can use transformation.inverse().transpose() * point/vector
///
///
///

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    // it will always be origin
    origin: Point,
    // identity matrix as default transformation, it can be changed so making it public
    pub transformation: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new(transformation_matrix: Matrix) -> Self {
        let mut sphere = Self::default();
        sphere.set_transformation(transformation_matrix);
        sphere
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
            z: 10.0,
            width: 8.0,
            height: 8.0,
        };
        let half_wall_size = wall.width * 0.5;

        // we will fill the canvas from top left, left to right, so in first loop/row y remains constant
        for y in 0..canvas.height {
            // compute the world y coordinate (top = +half, bottom = -half)
            let world_y = half_wall_size - (wall.height / canvas.height as f64) * y as f64;
            for x in 0..canvas.width {
                let world_x = -half_wall_size + (wall.width / canvas.width as f64) * x as f64;
                // get the point on the wall, then get the ray from eye to that point on wall
                let point_on_wall = Point::new(world_x, world_y, wall.z);
                // ray from torch to point on wall, origin is the torch and vector is from torch to point on wall
                let ray = Ray::new(torch.clone(), (point_on_wall - torch.clone()).normalize());
                let intersections = self.intersect(&ray);
                if intersections.is_empty() {
                    canvas.write_pixel((x as usize, y as usize), &black);
                } else {
                    if Sphere::hits(&intersections).is_some() {
                        canvas.write_pixel((x as usize, y as usize), &red);
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

    pub fn normal_at(&self, point: &Point) -> Vec4 {
        let world_point = point;
        let inverse_transformation = self.transformation.inverse_4x4().unwrap();
        // bring the world point (which we have to find normal to) into object space before calculating the normal
        // since normal is the vector from center of sphere and point of it's surface
        // the center of sphere (origin) is in object space
        let object_point = inverse_transformation.as_ref() * world_point;
        // normal in object space
        let object_normal = object_point.as_ref() - &self.origin;
        // convert the normal in object space to world space
        // normally we only have to multiply sphere transfomation matrix to bring object space normal to world space
        // not in this case
        let mut world_normal = inverse_transformation.transpose() * object_normal;
        // since the sphere will always be unit sphere we explicitly don't have to normalize it
        world_normal.3 = 0.0;
        world_normal.normalize()
    }

    pub fn sphere_with_lighting_to_canvas(&mut self) {
        self.material.color = Color::new(1.0, 0.2, 1.0);
        let mut canvas = Canvas::new(300, 300);
        let black = Color::black();
        let light_position = Point::new(-10.0, 10.0, -10.0);
        let light_color = Color::white();
        let light = Light::new(light_position, light_color);
        let wall = Wall {
            z: 10.0,
            width: 8.0,
            height: 8.0,
        };
        let eye_position = Point::new(0.0, 0.0, -5.0);
        // let wall_half_size = wall.width / 2.0;
        let half_wall_size = wall.width * 0.5;

        // we will fill the canvas from top left, left to right, so in first loop/row y remains constant
        for y in 0..canvas.height {
            // compute the world y coordinate (top = +half, bottom = -half)
            let world_y = half_wall_size - (wall.height / canvas.height as f64) * y as f64;
            for x in 0..canvas.width {
                let world_x = -half_wall_size + (wall.width / canvas.width as f64) * x as f64;
                // get the point on the wall, then get the ray from eye to that point on wall
                let point_on_wall = Point::new(world_x, world_y, wall.z);
                // ray from eye position to point on wall
                let ray = Ray::new(
                    eye_position.clone(),
                    (point_on_wall - eye_position.clone()).normalize(),
                );
                let intersections = self.intersect(&ray);
                if intersections.is_empty() {
                    canvas.write_pixel((x as usize, y as usize), &black);
                } else {
                    if let Some(mut hit) = Sphere::hits(&intersections) {
                        let point = ray.position(hit.distance);
                        let normal_vector = hit.object.normal_at(&point);
                        let eye_vector = -ray.direction;
                        let lighting = hit.object.material.lighting(
                            &light,
                            &point,
                            &eye_vector,
                            &normal_vector,
                            false,
                        );
                        canvas.write_pixel((x as usize, y as usize), &lighting);
                    }
                }
            }
        }
        let path = std::path::Path::new(".\\lighted_sphere.ppm");
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
            material: Material::default(),
        }
    }
}

pub struct Wall {
    z: f64,
    width: f64,
    height: f64,
}

impl Intersectable for Sphere {
    fn hits(intersections: &[Intersection<Sphere>]) -> Option<Intersection<Sphere>> {
        // if distance is negative than the object is behind the ray so exclude those intersections in hits
        let mut positive_intersections = intersections
            .iter()
            .filter(|&i| i.distance >= 0.0)
            .collect::<Vec<&Intersection<Sphere>>>();
        if positive_intersections.is_empty() {
            // if objects are not in front of ray
            None
        } else {
            positive_intersections.sort();
            Some(positive_intersections[0].clone())
        }
    }

    fn prepare_computation(
        intersection: &Intersection<Sphere>,
        ray: &Ray,
    ) -> PrerareComputation<Sphere> {
        let point = ray.position(intersection.distance);
        let normalv = Sphere::normal_at(&intersection.object, &point);
        let eyev = -ray.direction.clone();
        let (normalv, inside) = if normalv.dot(&eyev) < 0.0 {
            (-normalv, true)
        } else {
            (normalv, false)
        };
        let over_point = &point + &(0.000000001 * &normalv);
        PrerareComputation {
            distance: intersection.distance,
            normalv,
            object: intersection.object.clone(),
            point,
            eyev,
            inside,
            over_point,
        }
    }
}
