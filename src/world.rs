use crate::{
    canvas::Color,
    matrix::Matrix,
    matters::{light::Light, sphere::Sphere, Intersectable, Intersection, PrerareComputation},
    ray::Ray,
    vector::Point,
};

pub struct World {
    pub light: Option<Light>,
    pub otherlights: Option<Vec<Light>>,
    pub spheres: Option<Vec<Sphere>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            light: None,
            spheres: None,
            otherlights: None,
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
            otherlights: None,
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

    pub fn shade_hits_sphere(&self, precomps: &mut PrerareComputation<Sphere>) -> Color {
        let mut col = precomps.object.material.clone().lighting(
            self.light.as_ref().unwrap(),
            &precomps.point,
            &precomps.eyev,
            &precomps.normalv,
        );
        if let Some(lights) = self.otherlights.as_ref() {
            for light in lights {
                col = col
                    + precomps.object.material.lighting(
                        &light,
                        &precomps.point,
                        &precomps.eyev,
                        &precomps.normalv,
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
}

// NOTES:
// - hsize is the horizontal size (in pixels) of the canvas that the picture will be rendered to.
// - vsize is the canvas’s vertical size (in pixels).
// - field_of_view is an angle that describes how much the camera can see. When the field of view is small, the view will be “zoomed in,” magnifying a smaller area of the scene.
// - transform is a matrix describing how the world should be oriented relative to the camera. This is usually a view transformation like you implemented in the previous section.

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    // in radians
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let mut camera = Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity_4x4(),
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
        };
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        (camera.half_width, camera.half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        camera.pixel_size = (camera.half_width * 2.0) / camera.hsize as f64;
        camera
    }

    // gives a ray starting for a pixel on camera and passing through a point (x,y) on canvas
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        todo!()
    }
}
