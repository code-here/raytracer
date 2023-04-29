use std::io::Write;

use crate::{
    canvas::Color,
    matrix::Matrix,
    matters::{
        camera::Camera, light::Light, material::Material, sphere::Sphere, Intersectable,
        Intersection, PrerareComputation,
    },
    ray::Ray,
    vector::{Point, Vec4},
};

pub struct World {
    pub light: Option<Light>,
    pub otherlights: Option<Vec<Light>>,
    pub spheres: Option<Vec<Sphere>>,
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
            otherlights: None,
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            light: None,
            spheres: None,
            otherlights: None,
        }
    }
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

    pub fn shade_hits_sphere(&self, precomps: &mut PrerareComputation<Sphere>) -> Color {
        let is_shadowed = self.is_shadowed(precomps.over_point.as_ref());
        let mut col = precomps.object.material.clone().lighting(
            self.light.as_ref().unwrap(),
            &precomps.point,
            &precomps.eyev,
            &precomps.normalv,
            is_shadowed,
        );
        if let Some(lights) = self.otherlights.as_ref() {
            for light in lights {
                col = col
                    + precomps.object.material.lighting(
                        &light,
                        &precomps.point,
                        &precomps.eyev,
                        &precomps.normalv,
                        is_shadowed,
                    );
            }
        }
        col
    }

    pub fn color_at_sphere(&self, ray: &Ray) -> Color {
        let intersections = self.world_intersect(ray);
        if let Some(hit) = Sphere::hits(&intersections) {
            let mut precomps = Sphere::prepare_computation(&hit, ray);
            self.shade_hits_sphere(&mut precomps)
        } else {
            Color::black()
        }
    }

    pub fn is_shadowed(&self, point: &Point) -> bool {
        if let Some(light) = self.light.as_ref() {
            let v_from_light_to_point = &light.position - point;
            let distance = v_from_light_to_point.magnitude();
            let v_from_light_to_point_normalized = v_from_light_to_point.normalize();
            let ray = Ray::new(point.clone(), v_from_light_to_point_normalized.clone());
            let intersections = self.world_intersect(&ray);
            if let Some(hit) = Sphere::hits(&intersections) {
                if hit.distance < distance {
                    return true;
                }
            }
        }
        // check for other light sources (if there are any)
        if let Some(lights) = self.otherlights.as_ref() {
            for light in lights {
                let v_from_light_to_point = point - &light.position;
                let distance = v_from_light_to_point.magnitude();
                let v_from_light_to_point_normalized = v_from_light_to_point.normalize();
                let ray = Ray::new(point.clone(), v_from_light_to_point_normalized.clone());
                let intersections = self.world_intersect(&ray);
                if let Some(hit) = Sphere::hits(&intersections) {
                    if hit.distance < distance {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn world_to_ppm(&mut self) {
        let mut floor = Sphere::default();
        floor.transformation = Matrix::scaling_mat_4x4(10.0, 0.01, 10.0);
        floor.material = Material::default();
        floor.material.color = Color::new(1.0, 0.9, 0.9);
        floor.material.specular = 0.0;

        let mut left_wall = Sphere::default();
        left_wall.transformation = Matrix::identity_4x4()
            .scaling_mat_4x4_chain(10.0, 0.01, 10.0)
            .rotation_x_mat_4x4_chain(std::f64::consts::FRAC_PI_2)
            .rotation_y_mat_4x4_chain(-std::f64::consts::FRAC_PI_4)
            .translation_mat_4x4_chain(0.0, 0.0, 5.0);
        left_wall.material = floor.material.clone();

        let mut right_wall = Sphere::default();
        right_wall.transformation = Matrix::identity_4x4()
            .scaling_mat_4x4_chain(10.0, 0.01, 10.0)
            .rotation_x_mat_4x4_chain(std::f64::consts::FRAC_PI_2)
            .rotation_y_mat_4x4_chain(std::f64::consts::FRAC_PI_4)
            .translation_mat_4x4_chain(0.0, 0.0, 5.0);
        right_wall.material = floor.material.clone();

        let mut middle = Sphere::default();
        middle.transformation = Matrix::translation_mat_4x4(-0.5, 1.0, 0.5);
        middle.material = Material::default();
        middle.material.color = Color::new(0.1, 1.0, 0.5);
        middle.material.diffuse = 0.7;
        middle.material.specular = 0.3;
        let mut right = Sphere::default();
        right.transformation =
            Matrix::scaling_mat_4x4(0.5, 0.5, 0.5).translation_mat_4x4_chain(1.5, 0.5, -0.5);
        right.material = Material::default();
        right.material.color = Color::new(0.5, 1.0, 0.1);
        right.material.diffuse = 0.7;
        right.material.specular = 0.3;

        let mut left = Sphere::default();
        left.transformation =
            Matrix::scaling_mat_4x4(0.33, 0.33, 0.33).translation_mat_4x4_chain(-1.5, 0.33, -0.75);
        left.material = Material::default();
        left.material.color = Color::new(1.0, 0.8, 0.1);
        left.material.diffuse = 0.7;
        left.material.specular = 0.3;

        self.light = Some(Light::new(Point::new(-10.0, 10.0, -10.0), Color::white()));
        self.spheres = Some(vec![floor, left_wall, right_wall, middle, right, left]);
        let mut camera = Camera::new(300, 150, std::f64::consts::FRAC_PI_3);

        camera.transform = Matrix::view_transformation(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 1.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0),
        );

        let image = camera.render(&self);
        let path = std::path::Path::new(".\\first_world.ppm");
        if !path.exists() {
            std::fs::File::create(&path).unwrap();
        }
        let mut file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
        file.write_all(image.to_ppm().as_bytes()).unwrap();
    }

    // working on this
    pub fn shadow_dog_to_ppm(&mut self) {
        let mut background = Sphere::default();
        background.transformation = Matrix::scaling_mat_4x4(10.0, 10.0, 0.01)
            .rotation_y_mat_4x4_chain(std::f64::consts::FRAC_PI_4)
            .translation_mat_4x4_chain(3.0, 0.0, 3.0);
        background.material.color = Color::new(0.3, 0.3, 0.3);
        background.material.specular = 0.0;
        let mut s1 = Sphere::default();
        s1.transformation = Matrix::translation_mat_4x4(-2.0, 0.0, 0.0);
        s1.material = Material::default();
        s1.material.color = Color::new(1.0, 0.9, 0.9);

        let mut s2 = Sphere::new(Matrix::translation_mat_4x4(0.0, 1.0, 0.0));
        s2.material = Material::default();
        s2.material.color = Color::new(0.5, 0.9, 0.9);

        let mut f1 = Sphere::default();
        f1.transformation =
            Matrix::scaling_mat_4x4(0.3, 1.0, 0.3).translation_mat_4x4_chain(0.0, 1.0, 0.0);
        f1.material = s1.material.clone();

        let mut f2 = Sphere::default();
        f2.transformation = Matrix::scaling_mat_4x4(1.0, 0.3, 0.3)
            .rotation_z_mat_4x4_chain(std::f64::consts::FRAC_PI_2)
            .translation_mat_4x4_chain(0.0, 2.0, 0.0);
        f2.material = s1.material.clone();

        let mut f3 = Sphere::default();
        f3.transformation = Matrix::scaling_mat_4x4(1.0, 0.3, 0.3)
            .rotation_z_mat_4x4_chain(std::f64::consts::FRAC_PI_2)
            .translation_mat_4x4_chain(0.0, -2.0, 0.0);
        f3.material = s1.material.clone();

        self.light = Some(Light::new(Point::new(-10.0, 0.0, -5.0), Color::white()));
        self.spheres = Some(vec![s1, s2, f1, f2, f3, background]);
        let mut camera = Camera::new(300, 150, std::f64::consts::FRAC_PI_3);

        camera.transform = Matrix::view_transformation(
            Point::new(0.0, 3.0, -8.0),
            Point::new(0.0, 1.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0),
        );

        let image = camera.render(&self);
        let path = std::path::Path::new(".\\dog_world.ppm");
        if !path.exists() {
            std::fs::File::create(&path).unwrap();
        }
        let mut file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
        file.write_all(image.to_ppm().as_bytes()).unwrap();
    }
}
