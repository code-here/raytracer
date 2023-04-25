use crate::{
    canvas::Color,
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
