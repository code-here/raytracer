use crate::{
    canvas::Color,
    matrix::Matrix,
    matters::{light::Light, sphere::Sphere, Intersection},
    ray::Ray,
    vector::Point,
};

pub struct World {
    pub light: Option<Light>,
    pub spheres: Option<Vec<Sphere>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            light: None,
            spheres: None,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        // defalt world will have a light source and two concentric spheres
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Color::white());
        let mut spheres = Vec::new();
        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        spheres.push(s1);
        let s2 = Sphere::new(Matrix::scaling_mat_4x4(0.5, 0.5, 0.5));
        spheres.push(s2);
        Self {
            light: Some(light),
            spheres: Some(spheres),
        }
    }
}

impl World {
    pub fn world_intersect(&self, ray: &Ray) -> Vec<Intersection<Sphere>> {
        let mut xs = self
            .spheres
            .as_ref()
            .unwrap()
            .iter()
            .map(|sphere| sphere.intersect(&ray))
            .flatten()
            .collect::<Vec<Intersection<Sphere>>>();
        xs.sort();
        xs
    }
}
