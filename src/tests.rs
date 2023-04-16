use crate::{
    canvas::{Canvas, Color},
    matrix::Matrix,
    vector::{Point, Vec4},
};

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

#[test]
fn colors_red_green_blue() {
    let c = Color::from([-0.5, 0.4, 1.7]);
    assert_eq!(c.red(), -0.5);
    assert_eq!(c.green(), 0.4);
    assert_eq!(c.blue(), 1.7);
}

#[test]
fn test_color_at_a_pixel_after_writing() {
    let red = Color::from([1.0, 0.0, 0.0]);
    let mut canvas = Canvas::new(20, 20);
    assert_eq!(canvas.pixel_at((2, 3)), &Color::black());
    canvas.write_pixel((2, 3), &red);
    assert_eq!(canvas.pixel_at((2, 3)), &red);
}

#[test]
fn test_canvas_to_ppm() {
    let mut canvas = Canvas::new(5, 3);
    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);
    canvas.write_pixel((0, 0), &c1);
    canvas.write_pixel((2, 1), &c2);
    canvas.write_pixel((4, 2), &c3);

    assert_eq!(
        canvas.to_ppm(),
        r#"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 
"#
    );
}

#[test]
fn test_matrix_4x4() {
    let m4 = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [1.0, 5.5, 3.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [7.5, 2.0, 3.0, 16.5],
    ]);
    assert_eq!(m4.0[0][0], 1.0);
    assert_eq!(m4.0[1][1], 5.5);
    assert_eq!(m4.0[3][0], 7.5);
    assert_eq!(m4.0[3][3], 16.5);
}

#[test]
fn cmpare_matrix_4x4() {
    let m4 = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [1.0, 5.5, 3.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [7.5, 2.0, 3.0, 16.5],
    ]);
    let m41 = m4.clone();
    assert_eq!(m4, m41);
}

#[test]
fn multiply_matrix_4x4() {
    let a = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    let b = Matrix::from([
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    ]);

    assert_eq!(
        a * b,
        Matrix::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ])
    )
}

#[test]
fn multiply_matric_4x4_to_4x1() {
    // multiplying matrix to a vector
    let a = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    // convert the vector to matrix first
    let b = Matrix::from(Vec4(1.0, 2.0, 3.0, 1.0));
    let res = Matrix::from(Vec4(18.0, 24.0, 33.0, 1.0));
    assert_eq!(a * b, res);
}

#[test]
fn test_identity_transformation() {
    // multiplying matrix to a vector
    let a = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    // convert the vector to matrix first
    let i = Matrix::identiry_4x4();
    let res = a.clone();
    assert_eq!(a * i, res);
}

#[test]
fn test_transpose_4x4() {
    // multiplying matrix to a vector
    let a = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    let a_1 = Matrix::from([
        [1.0, 2.0, 8.0, 0.0],
        [2.0, 4.0, 6.0, 0.0],
        [3.0, 4.0, 4.0, 0.0],
        [4.0, 2.0, 1.0, 1.0],
    ]);
    assert_eq!(a.transpose(), a_1);
}

#[test]
fn transpose_of_identity_is_identity() {
    // multiplying matrix to a vector
    let a = Matrix::identiry_4x4();
    let ac = a.clone();
    assert_eq!(a.transpose(), ac);
}
