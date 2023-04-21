use crate::{
    matters::{Intersection, Sphere},
    ray::Ray,
    vector::{Point, Vec4},
};

#[test]
fn test_ray_creation() {
    let p = Point::new(1.0, 2.0, 3.0);
    let d = Vec4::new(4.0, 5.0, 6.0);
    let r = Ray::new(p.clone(), d.clone());
    assert_eq!(r.origin, p);
    assert_eq!(r.direction, d);
}

#[test]
fn computing_a_point_from_a_distance() {
    let p = Point::new(2.0, 3.0, 4.0);
    let d = Vec4::new(1.0, 0.0, 0.0);
    let r = Ray::new(p.clone(), d.clone());
    assert_eq!(r.position(0.0), p);
    assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
    assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
    assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
}

#[test]
fn a_ray_intersects_a_sphere_at_2_points() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vec4::new(0.0, 0.0, 1.0));
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(4.0, sphere.clone());
    let intersection2 = Intersection::new(6.0, sphere.clone());
    assert_eq!(sphere.intersect(&ray), vec![intersection1, intersection2]);
}

#[test]
fn a_ray_intersects_a_sphere_at_1_points() {
    let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vec4::new(0.0, 0.0, 1.0));
    let sphere = Sphere::default();
    let intersect = Intersection::new(5.0, sphere.clone());
    assert_eq!(sphere.intersect(&ray), vec![intersect.clone(), intersect]);
}

#[test]
fn a_ray_intersects_a_sphere_at_0_points() {
    let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vec4::new(0.0, 0.0, 1.0));
    let sphere = Sphere::default();
    assert_eq!(sphere.intersect(&ray), vec![]);
}
#[test]
fn a_ray_originated_inside_sphere_intersects_at_two_points() {
    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vec4::new(0.0, 0.0, 1.0));
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(-1.0, sphere.clone());
    let intersection2 = Intersection::new(1.0, sphere.clone());
    assert_eq!(sphere.intersect(&ray), vec![intersection1, intersection2]);
}

#[test]
fn a_sphere_is_behide_a_ray() {
    let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vec4::new(0.0, 0.0, 1.0));
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(-6.0, sphere.clone());
    let intersection2 = Intersection::new(-4.0, sphere.clone());
    assert_eq!(sphere.intersect(&ray), vec![intersection1, intersection2]);
}

#[test]
fn hits_when_all_intersections_are_positive_distance() {
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(1.0, sphere.clone());
    let intersection2 = Intersection::new(2.0, sphere.clone());
    assert_eq!(
        Intersection::hits(&vec![intersection1.clone(), intersection2]),
        Some(intersection1)
    );
}

#[test]
fn hits_when_some_intersections_have_negative_distance() {
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(-1.0, sphere.clone());
    let intersection2 = Intersection::new(1.0, sphere.clone());
    assert_eq!(
        Intersection::hits(&vec![intersection1, intersection2.clone()]),
        Some(intersection2)
    );
}

#[test]
fn hits_when_all_intersections_are_negative_distance() {
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(-1.0, sphere.clone());
    let intersection2 = Intersection::new(-2.0, sphere.clone());
    assert_eq!(
        Intersection::hits(&vec![intersection1.clone(), intersection2]),
        None
    );
}

#[test]
fn hits_is_always_the_non_negative_number() {
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(5.0, sphere.clone());
    let intersection2 = Intersection::new(7.0, sphere.clone());
    let intersection3 = Intersection::new(-3.0, sphere.clone());
    let intersection4 = Intersection::new(2.0, sphere.clone());
    assert_eq!(
        Intersection::hits(&vec![
            intersection1,
            intersection2,
            intersection3,
            intersection4.clone()
        ]),
        Some(intersection4)
    );
}
