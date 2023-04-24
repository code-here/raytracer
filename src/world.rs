use crate::{
    canvas::Color,
    matrix::Matrix,
    matters::{light::Light, sphere::Sphere, Object},
    vector::Point,
};

pub struct World {
    pub light: Option<Light>,
    pub objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            light: None,
            objects: vec![],
        }
    }
}

impl Default for World {
    fn default() -> Self {
        // defalt world will have a light source and two concentric spheres
        let light = Light::new(Point::new(-10.0, 10.0, -10.0), Color::white());
        let mut objects: Vec<Box<dyn Object>> = Vec::new();
        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        objects.push(Box::new(s1));
        let s2 = Sphere::new(Matrix::scaling_mat_4x4(0.5, 0.5, 0.5));
        objects.push(Box::new(s2));
        Self {
            light: Some(light),
            objects,
        }
    }
}
