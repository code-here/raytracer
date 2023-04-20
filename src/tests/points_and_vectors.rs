use crate::vector::{Point, Vec4};

#[test]
fn point_have_one_as_last_coordinate() {
    let point = Point::new(1.0, 2.0, 3.0);
    assert_eq!(point, Point(1.0, 2.0, 3.0, 1.0));
}

#[test]
fn vector_have_zero_as_last_coordinate() {
    let vec = Vec4::new(-1.0, 2.0, 3.0);
    assert_eq!(vec, Vec4(-1.0, 2.0, 3.0, 0.0));
}

#[test]
fn cmp_points() {
    let point = Point::new(1.0, 2.0, 3.0);
    let spoint = Point::new(1.0, 2.0, 3.0);
    assert!(point.eq(&spoint));
}

#[test]
fn add_vector_to_point() {
    let point = Point::origin();
    let vec = Vec4::new(1.0, 2.0, 3.0);
    assert_eq!(Point::new(1.0, 2.0, 3.0), point + vec);
}

#[test]
fn subtract_points_gives_vector() {
    let p1 = Point::new(3.0, 2.0, 1.0);
    let p2 = Point::new(5.0, 6.0, 7.0);
    assert_eq!(Vec4::new(-2.0, -4.0, -6.0), p1 - p2);
}

#[test]
fn substract_vector_from_point_gives_vector() {
    let p1 = Point::new(3.0, 2.0, 1.0);
    let v1 = Vec4::new(5.0, 6.0, 7.0);
    assert_eq!(Point::new(-2.0, -4.0, -6.0), p1 - v1);
}

#[test]
fn substracting_vectors_gives_vector() {
    let v1 = Vec4::new(3.0, 2.0, 1.0);
    let v2 = Vec4::new(5.0, 6.0, 7.0);
    assert_eq!(Vec4::new(-2.0, -4.0, -6.0), v1 - v2);
}

#[test]
fn negeting_a_vector() {
    let v1 = Vec4::new(3.0, 2.0, 1.0);
    assert_eq!(Vec4::new(-3.0, -2.0, -1.0), -v1);
}
