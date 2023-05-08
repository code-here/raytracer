use crate::{
    canvas::Color,
    matrix::Matrix,
    matters::{light::Light, sphere::Sphere, Intersectable, Intersection, Shape},
    ray::Ray,
    vector::{Point, Vec4},
    world::World,
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
fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
    let world = World::default();
    let point = Point::new(0.0, 10.0, 0.0);
    assert_eq!(world.is_shadowed(&point), false);
}

#[test]
fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
    let world = World::default();
    let point = Point::new(10.0, -10.0, 10.0);
    assert_eq!(world.is_shadowed(&point), true);
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_light() {
    let world = World::default();
    let point = Point::new(-20.0, 20.0, -20.0);
    assert_eq!(world.is_shadowed(&point), false);
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_point() {
    let world = World::default();
    let point = Point::new(-2.0, 2.0, -2.0);
    assert_eq!(world.is_shadowed(&point), false);
}

#[test]
fn shade_hit_is_given_an_intersection_in_shadow() {
    let mut world = World::new();
    world.light = Some(Light::new(
        Point::new(0.0, 0.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));
    let s1 = Sphere::default();
    let s2 = Sphere::new(Matrix::translation_mat_4x4(0.0, 0.0, 10.0));
    world.spheres = Some(vec![s1, s2.clone()]);
    let ray = Ray::new(Point::new(0.0, 0.0, 0.5), Vec4::new(0.0, 0.0, 1.0));
    let i = Intersection::new(4.0, s2);
    let mut comps = Sphere::prepare_computation(&i, &ray);
    let c = world.shade_hits_sphere(&mut comps);
    assert_eq!(c, Color::new(0.1, 0.1, 0.1));
}
