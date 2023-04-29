use crate::{
    canvas::Color,
    matrix::Matrix,
    matters::camera::Camera,
    vector::{Point, Vec4},
    world::World,
};

#[test]
fn constructing_a_camera() {
    let camera = Camera::new(160, 120, std::f64::consts::FRAC_PI_2);
    assert_eq!(camera.hsize, 160);
    assert_eq!(camera.vsize, 120);
    assert_eq!(camera.field_of_view, std::f64::consts::FRAC_PI_2);
    assert_eq!(camera.transform, Matrix::identity_4x4());
}

#[test]
fn the_pixel_size_for_a_horizontal_canvas() {
    let camera = Camera::new(200, 125, std::f64::consts::FRAC_PI_2);
    assert_eq!(camera.pixel_size, 0.009999999999999998);
}

#[test]
fn the_pixel_size_for_a_vertical_canvas() {
    let camera = Camera::new(125, 200, std::f64::consts::FRAC_PI_2);
    assert_eq!(camera.pixel_size, 0.009999999999999998);
}

#[test]
fn constructing_a_ray_through_the_center_of_the_canvas() {
    let camera = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(
        (ray.origin, ray.direction),
        (Point::origin(), Vec4::new(0.0, 0.0, -1.0))
    );
}

#[test]
fn constructing_a_ray_through_a_corner_of_the_canvas() {
    let camera = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
    let ray = camera.ray_for_pixel(0, 0);
    assert_eq!(
        (ray.origin, ray.direction),
        (Point::origin(), Vec4::new(0.66519, 0.33259, -0.66851))
    );
}
#[test]
fn constructing_a_ray_when_the_camera_is_transformed() {
    let mut camera = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
    camera.transform = camera
        .transform
        .translation_mat_4x4_chain(0.0, -2.0, 5.0)
        .rotation_y_mat_4x4_chain(std::f64::consts::FRAC_PI_4);
    let ray = camera.ray_for_pixel(100, 50);
    assert_eq!(
        (ray.origin, ray.direction),
        (
            Point::new(0.0, 2.0, -5.0),
            Vec4::new(2.0f64.sqrt() / 2.0, 0.0, -2.0f64.sqrt() / 2.0)
        )
    );
}

#[test]
fn rendering_a_world_with_a_camera() {
    let world = World::default();
    let mut camera = Camera::new(11, 11, std::f64::consts::FRAC_PI_2);
    let from = Point::new(0.0, 0.0, -5.0);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vec4::new(0.0, 1.0, 0.0);
    camera.transform = Matrix::view_transformation(from, to, up);
    let image = camera.render(&world);
    assert_eq!(
        *image.pixel_at((5, 5)),
        Color::new(0.38066, 0.47583, 0.2855)
    );
}
