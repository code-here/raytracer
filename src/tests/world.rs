use crate::{
    canvas::Color,
    matrix::Matrix,
    matters::{light::Light, sphere::Sphere, Intersectable, Intersection},
    ray::Ray,
    vector::{Point, Vec4},
    world::{Camera, World},
};

#[test]
fn new_world_has_no_light_source_and_no_objects() {
    let world = World::new();
    assert!(world.light.is_none());
    assert!(world.spheres.is_none());
}

#[test]
fn default_world() {
    let world = World::default();
    let (color, diffuse, specular) = (
        world.spheres.as_ref().unwrap()[0].material.color.clone(),
        world.spheres.as_ref().unwrap()[0].material.diffuse,
        world.spheres.as_ref().unwrap()[0].material.specular,
    );
    assert_eq!(
        (color, diffuse, specular),
        (Color::new(0.8, 1.0, 0.6), 0.7, 0.2)
    );
}

#[test]
fn default_world_intersect() {
    let world = World::default();
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vec4::new(0.0, 0.0, 1.0));
    let intersections = world.world_intersect(&ray);
    assert_eq!(
        (
            intersections[0].distance,
            intersections[1].distance,
            intersections[2].distance,
            intersections[3].distance
        ),
        (4.0, 4.5, 5.5, 6.0)
    );
}

#[test]
fn shading_an_intersection() {
    let world = World::default();
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vec4::new(0.0, 0.0, 1.0));
    let intersection = Intersection::new(4.0, world.spheres.as_ref().unwrap()[0].clone());
    let mut comp = Sphere::prepare_computation(&intersection, &ray);
    let c = world.shade_hits_sphere(&mut comp);
    assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
}

#[test]
fn shading_an_intersection_from_inside() {
    let mut world = World::default();
    world.light = Some(Light::new(
        Point::new(0.0, 0.25, 0.0),
        Color::new(1.0, 1.0, 1.0),
    ));
    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vec4::new(0.0, 0.0, 1.0));
    let intersection = Intersection::new(0.5, world.spheres.as_ref().unwrap()[1].clone());
    let mut comp = Sphere::prepare_computation(&intersection, &ray);
    let c = world.shade_hits_sphere(&mut comp);
    assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
}

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
