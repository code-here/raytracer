// NOTES:
// - hsize is the horizontal size (in pixels) of the canvas that the picture will be rendered to.
// - vsize is the canvas’s vertical size (in pixels).
// - field_of_view is an angle that describes how much the camera can see. When the field of view is small, the view will be “zoomed in,” magnifying a smaller area of the scene.
// - transform is a matrix describing how the world should be oriented relative to the camera. This is usually a view transformation like you implemented in the previous section.

use crate::{canvas::Canvas, matrix::Matrix, ray::Ray, vector::Point, world::World};

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

    // gives a ray starting for a pixel on camera and passing through a point (px,py) on canvas
    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        //  the untransformed coordinates of the pixel in world space.
        //  (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        //  using the camera matrix, transform the canvas point and the origin,
        //  and then compute the ray's direction vector.
        //  (remember that the canvas is at z=-1)
        let pixel = self.transform.inverse_4x4().unwrap() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse_4x4().unwrap() * Point::origin();
        let direction = (&pixel - &origin).normalize();
        Ray::new(origin, direction)
    }
    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize as u64, self.vsize as u64);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at_sphere(&ray);
                image.write_pixel((x, y), &color);
            }
        }
        image
    }
}
