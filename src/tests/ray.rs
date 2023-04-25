use crate::{
    matrix::Matrix,
    matters::{sphere::Sphere, Intersectable, Intersection},
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
fn hits_when_all_intersections_are_positive_distance() {
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(1.0, sphere.clone());
    let intersection2 = Intersection::new(2.0, sphere.clone());
    assert_eq!(
        Sphere::hits(&vec![intersection1.clone(), intersection2]),
        Some(intersection1)
    );
}

#[test]
fn hits_when_some_intersections_have_negative_distance() {
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(-1.0, sphere.clone());
    let intersection2 = Intersection::new(1.0, sphere.clone());
    assert_eq!(
        Sphere::hits(&vec![intersection1, intersection2.clone()]),
        Some(intersection2)
    );
}

#[test]
fn hits_when_all_intersections_are_negative_distance() {
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(-1.0, sphere.clone());
    let intersection2 = Intersection::new(-2.0, sphere.clone());
    assert_eq!(
        Sphere::hits(&vec![intersection1.clone(), intersection2]),
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
        Sphere::hits(&vec![
            intersection1,
            intersection2,
            intersection3,
            intersection4.clone()
        ]),
        Some(intersection4)
    );
}

#[test]
fn translating_a_ray() {
    let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vec4::new(0.0, 1.0, 0.0));
    let translation_m = Matrix::identity_4x4().translation_mat_4x4_chain(3.0, 4.0, 5.0);
    let transformed = ray.transform(translation_m);
    assert_eq!(transformed.origin, Point::new(4.0, 6.0, 8.0));
    assert_eq!(transformed.direction, Vec4::new(0.0, 1.0, 0.0));
}

#[test]
fn scaling_a_ray() {
    let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vec4::new(0.0, 1.0, 0.0));
    let scaling_m = Matrix::identity_4x4().scaling_mat_4x4_chain(2.0, 3.0, 4.0);
    let transformed = ray.transform(scaling_m);
    assert_eq!(transformed.origin, Point::new(2.0, 6.0, 12.0));
    assert_eq!(transformed.direction, Vec4::new(0.0, 3.0, 0.0));
}

#[test]
fn precomputing_the_state_of_an_intersection() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vec4::new(0.0, 0.0, 1.0));
    let sphere = Sphere::default();
    let intersection = Intersection::new(4.0, sphere);
    let comp = Sphere::prepare_computation(&intersection, &ray);
    assert_eq!(intersection.distance, comp.distance);
    assert_eq!(intersection.object, comp.object);
    assert_eq!(Point::new(0.0, 0.0, -1.0), comp.point);
    assert_eq!(Vec4::new(0.0, 0.0, -1.0), comp.eyev);
    assert_eq!(Vec4::new(0.0, 0.0, -1.0), comp.normalv);
}
