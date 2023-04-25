use crate::{
    matrix::Matrix,
    matters::{material::Material, sphere::Sphere, Intersection},
    ray::Ray,
    vector::{Point, Vec4},
};

// sphere tests

#[test]
fn default_sphere_has_identity_transformation() {
    let sphere = Sphere::default();
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
    let sphere = Sphere::new(Matrix::translation_mat_4x4(2.0, 3.0, 4.0));
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

#[test]
fn normal_on_sphere_at_point_on_x_axis() {
    let s = Sphere::default();
    let p = Point::new(1.0, 0.0, 0.0);
    assert_eq!(s.normal_at(&p), Vec4::new(1.0, 0.0, 0.0));
}
#[test]
fn normal_on_sphere_at_point_on_y_axis() {
    let s = Sphere::default();
    let p = Point::new(0.0, 1.0, 0.0);
    assert_eq!(s.normal_at(&p), Vec4::new(0.0, 1.0, 0.0));
}
#[test]
fn normal_on_sphere_at_point_on_z_axis() {
    let s = Sphere::default();
    let p = Point::new(0.0, 0.0, 1.0);
    assert_eq!(s.normal_at(&p), Vec4::new(0.0, 0.0, 1.0));
}
#[test]
fn normal_on_sphere_at_nonaxial_point() {
    let s = Sphere::default();
    let p = Point::new(
        3.0f64.sqrt() / 3.0,
        3.0f64.sqrt() / 3.0,
        3.0f64.sqrt() / 3.0,
    );
    assert_eq!(
        s.normal_at(&p),
        Vec4::new(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0
        )
    );
}
#[test]
fn normal_is_normalized_vector() {
    let s = Sphere::default();
    let p = Point::new(
        3.0f64.sqrt() / 3.0,
        3.0f64.sqrt() / 3.0,
        3.0f64.sqrt() / 3.0,
    );
    assert_eq!(
        s.normal_at(&p),
        Vec4::new(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
        )
        .normalize()
    );
}

#[test]
fn normal_on_a_translated_sphere() {
    let s = Sphere::new(Matrix::translation_mat_4x4(0.0, 1.0, 0.0));
    let p = Point::new(0.0, 1.70711, -0.70711);
    assert_eq!(s.normal_at(&p), Vec4::new(0.0, 0.70711, -0.70711,));
}
#[test]
fn normal_on_a_transformed_sphere() {
    let s = Sphere::new(
        Matrix::identity_4x4()
            .rotation_z_mat_4x4_chain(std::f64::consts::PI / 5.0)
            .scaling_mat_4x4_chain(1.0, 0.5, 1.0),
    );
    let p = Point::new(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
    assert_eq!(s.normal_at(&p), Vec4::new(0.0, 0.97014, -0.24253));
}

#[test]
fn a_sphere_has_a_default_material() {
    let sphere = Sphere::default();
    assert_eq!(sphere.material, Material::default());
}

#[test]
fn a_sphere_maybe_assigned_a_material() {
    let mut sphere = Sphere::default();
    let mut material = Material::default();
    material.ambient = 1.0;
    sphere.material = material.clone();
    assert_eq!(sphere.material, material);
}
