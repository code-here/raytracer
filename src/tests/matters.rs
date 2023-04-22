use crate::{
    matrix::Matrix,
    matters::{Intersection, Sphere},
    ray::Ray,
    vector::{Point, Vec4},
};

// sphere tests

#[test]
fn default_sphere_has_identity_transformation() {
    let sphere = Sphere::new();
    assert_eq!(sphere.transformation, Matrix::identity_4x4());
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
fn a_sphere_is_behide_a_ray_again() {
    let ray = Ray::new(Point::new(0.0, 0.0, -2.5), Vec4::new(0.0, 0.0, 0.5));
    let sphere = Sphere::default();
    let intersection1 = Intersection::new(3.0, sphere.clone());
    let intersection2 = Intersection::new(7.0, sphere.clone());
    assert_eq!(sphere.intersect(&ray), vec![intersection1, intersection2]);
}

#[test]
fn changing_sphere_transformation() {
    let mut sphere = Sphere::new();
    sphere.transformation = sphere
        .transformation
        .translation_mat_4x4_chain(2.0, 3.0, 4.0);
    assert_eq!(
        sphere.transformation,
        Matrix::identity_4x4().translation_mat_4x4_chain(2.0, 3.0, 4.0)
    );
}

#[test]
fn intersecting_a_scaled_sphere_with_a_ray() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vec4::new(0.0, 0.0, 1.0));
    let mut sphere = Sphere::default();
    sphere.transformation = sphere.transformation.scaling_mat_4x4_chain(2.0, 2.0, 2.0);
    dbg!(&sphere.transformation);
    let intersection1 = Intersection::new(3.0, sphere.clone());
    let intersection2 = Intersection::new(7.0, sphere.clone());
    assert_eq!(sphere.intersect(&ray), vec![intersection1, intersection2]);
}

#[test]
fn intersecting_a_translated_sphere_with_a_ray() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vec4::new(0.0, 0.0, 1.0));
    let mut sphere = Sphere::default();
    sphere.transformation = sphere
        .transformation
        .translation_mat_4x4_chain(5.0, 0.0, 0.0);
    assert_eq!(sphere.intersect(&ray), vec![]);
}
