use crate::{
    matrix::Matrix,
    vector::{Point, Vec4},
};

#[test]
fn translation_of_point_test_one() {
    let tr_mat = Matrix::translation_mat_4x4(5.0, -3.0, 2.0);
    let p = Point::new(-3.0, 4.0, 5.0);
    assert_eq!(tr_mat * p, Point::new(2.0, 1.0, 7.0));
}

#[test]
fn inverse_translation_of_point_test_one() {
    let tr_mat = Matrix::translation_mat_4x4(5.0, -3.0, 2.0);
    let p = Point::new(-3.0, 4.0, 5.0);
    assert_eq!(
        tr_mat.inverse_4x4().unwrap() * p,
        Point::new(-8.0, 7.0, 3.0)
    );
}

#[test]
fn translation_matrix_should_not_effect_vectors() {
    let tr_mat = Matrix::translation_mat_4x4(5.0, -3.0, 2.0);
    let p = Vec4::new(-3.0, 4.0, 5.0);
    assert_eq!(tr_mat * p, Vec4::new(-3.0, 4.0, 5.0));
}

#[test]
fn scaling_of_point_test_one() {
    let sc_mat = Matrix::scaling_mat_4x4(2.0, 3.0, 4.0);
    let p = Point::new(-4.0, 6.0, 8.0);
    assert_eq!(sc_mat * p, Point::new(-8.0, 18.0, 32.0));
}

#[test]
fn scaling_of_vectors_test_one() {
    let sc_mat = Matrix::scaling_mat_4x4(2.0, 3.0, 4.0);
    let p = Vec4::new(-4.0, 6.0, 8.0);
    assert_eq!(sc_mat * p, Vec4::new(-8.0, 18.0, 32.0));
}

#[test]
fn reflaction_is_scaling_by_negative_value_neg_x_here() {
    let sc_mat = Matrix::scaling_mat_4x4(-1.0, 1.0, 1.0);
    let p = Point::new(2.0, 3.0, 4.0);
    assert_eq!(sc_mat * p, Point::new(-2.0, 3.0, 4.0));
}

#[test]
fn rotate_around_x_axis() {
    let half_quarter = Matrix::rotation_x_mat_4x4(std::f64::consts::PI / 4.0);
    let inverse_half_quater = half_quarter.inverse_4x4().unwrap();
    let p = Point::new(0.0, 1.0, 0.0);
    assert_eq!(
        inverse_half_quater * p,
        Point::new(0.0, 2.0_f64.sqrt() * 0.5, -2.0_f64.sqrt() * 0.5)
    );
}

#[test]
fn rotate_around_y_axis() {
    let half_quarter = Matrix::rotation_y_mat_4x4(std::f64::consts::PI / 4.0);
    let quarter = Matrix::rotation_y_mat_4x4(std::f64::consts::PI / 2.0);
    let p = Point::new(0.0, 0.0, 1.0);
    assert_eq!(
        half_quarter * p.clone(),
        Point::new(2.0_f64.sqrt() * 0.5, 0.0, 2.0_f64.sqrt() * 0.5)
    );
    assert_eq!(quarter * p, Point::new(1.0, 0.0, 0.0));
}

#[test]
fn rotate_around_z_axis() {
    let half_quarter = Matrix::rotation_z_mat_4x4(std::f64::consts::PI / 4.0);
    let quarter = Matrix::rotation_z_mat_4x4(std::f64::consts::PI / 2.0);
    let p = Point::new(0.0, 1.0, 0.0);
    assert_eq!(
        half_quarter * p.clone(),
        Point::new(-2.0_f64.sqrt() * 0.5, 2.0_f64.sqrt() * 0.5, 0.0)
    );
    assert_eq!(quarter * p, Point::new(-1.0, 0.0, 0.0));
}

#[test]
fn shearing_transformation_move_x_propotion_to_y() {
    let shearing = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    assert_eq!(shearing * p, Point::new(5.0, 3.0, 4.0))
}
#[test]
fn shearing_transformation_move_x_propotion_to_z() {
    let shearing = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    assert_eq!(shearing * p, Point::new(6.0, 3.0, 4.0))
}
#[test]
fn shearing_transformation_move_y_propotion_to_x() {
    let shearing = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    assert_eq!(shearing * p, Point::new(2.0, 5.0, 4.0))
}
#[test]
fn shearing_transformation_move_y_propotion_to_z() {
    let shearing = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    assert_eq!(shearing * p, Point::new(2.0, 7.0, 4.0))
}
#[test]
fn shearing_transformation_move_z_propotion_to_x() {
    let shearing = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let p = Point::new(2.0, 3.0, 4.0);
    assert_eq!(shearing * p, Point::new(2.0, 3.0, 6.0))
}
#[test]
fn shearing_transformation_move_z_propotion_to_y() {
    let shearing = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let p = Point::new(2.0, 3.0, 4.0);
    assert_eq!(shearing * p, Point::new(2.0, 3.0, 7.0))
}

#[test]
fn individual_transformations_are_applied_in_sequence() {
    let p = Point::new(1.0, 0.0, 1.0);
    let a = Matrix::rotation_x_mat_4x4(std::f64::consts::PI / 2.0);
    let b = Matrix::scaling_mat_4x4(5.0, 5.0, 5.0);
    let c = Matrix::translation_mat_4x4(10.0, 5.0, 7.0);
    let rotation = a * p.clone();
    let scaling = b * rotation.clone();
    let traslation = c * scaling.clone();
    assert_eq!(rotation, Point::new(1.0, -1.0, 0.0));
    assert_eq!(scaling, Point::new(5.0, -5.0, 0.0));
    assert_eq!(traslation, Point::new(15.0, 0.0, 7.0));
}

#[test]
fn chained_transformation_must_be_applied_in_reverse() {
    let p = Point::new(1.0, 0.0, 1.0);
    let a = Matrix::rotation_x_mat_4x4(std::f64::consts::PI / 2.0);
    let b = Matrix::scaling_mat_4x4(5.0, 5.0, 5.0);
    let c = Matrix::translation_mat_4x4(10.0, 5.0, 7.0);
    assert_eq!(c * b * a * p, Point::new(15.0, 0.0, 7.0));
}

#[test]
fn chained_transformation_must_be_applied_in_reverse_with_builder_pattern() {
    let p = Point::new(1.0, 0.0, 1.0);
    let transformations = Matrix::identity_4x4()
        .rotation_x_mat_4x4_chain(std::f64::consts::PI / 2.0)
        .scaling_mat_4x4_chain(5.0, 5.0, 5.0)
        .translation_mat_4x4_chain(10.0, 5.0, 7.0);
    assert_eq!(transformations * p, Point::new(15.0, 0.0, 7.0));
}
