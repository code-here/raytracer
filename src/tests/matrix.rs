use crate::{
    matrix::Matrix,
    vector::{Point, Vec4},
};

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
fn test_multiplying_identity_with_any_matrix_gives_the_same_matrix_as_result() {
    // multiplying matrix to a vector
    let a = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    // convert the vector to matrix first
    let i = Matrix::identity_4x4();
    assert_eq!(a.clone() * i, a);
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
    let a = Matrix::identity_4x4();
    let ac = a.clone();
    assert_eq!(a.transpose(), ac);
}

#[test]
fn determinant_of_2x2() {
    // multiplying matrix to a vector
    let a = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);
    assert_eq!(a.det_2x2(), Ok(17.0));
}

#[test]
fn test_submatrix_of_4x4() {
    // multiplying matrix to a vector
    let a_4x4 = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    assert_eq!(
        a_4x4.submatrix(2, 1),
        Matrix::from([[1.0, 3.0, 4.0], [2.0, 4.0, 2.0], [0.0, 0.0, 1.0],])
    );
}

#[test]
fn test_submatrix_of_3x3() {
    let a_3x3 = Matrix::from([[1.0, 2.0, 8.0], [2.0, 4.0, 6.0], [3.0, 4.0, 4.0]]);
    assert_eq!(
        a_3x3.submatrix(0, 2),
        Matrix::from([[2.0, 4.0], [3.0, 4.0],])
    );
}

#[test]
fn test_minor_of_3x3() {
    let a_3x3 = Matrix::from([[1.0, 2.0, 8.0], [2.0, 4.0, 6.0], [3.0, 4.0, 4.0]]);
    assert_eq!(a_3x3.minor_3x3(1, 0), Ok(-24.0));
}

#[test]
fn test_cofactor_of_3x3() {
    let a_3x3 = Matrix::from([[1.0, 2.0, 8.0], [2.0, 4.0, 6.0], [3.0, 4.0, 4.0]]);
    assert_eq!(a_3x3.cofactor_3x3(1, 0), Ok(24.0));
}

#[test]
fn test_determinant_of_4x4() {
    // multiplying matrix to a vector
    let a_4x4 = Matrix::from([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    assert_eq!(a_4x4.det_4x4(), Ok(-20.0));
}

#[test]
fn test_inverse_of_4x4_test_one() {
    // multiplying matrix to a vector
    let a_4x4 = Matrix::from([
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0],
    ]);
    let inverse_4x4 = Matrix::from([
        [
            0.21804511278195488,
            0.45112781954887216,
            0.24060150375939848,
            -0.045112781954887216,
        ],
        [
            -0.8082706766917293,
            -1.456766917293233,
            -0.44360902255639095,
            0.5206766917293233,
        ],
        [
            -0.07894736842105263,
            -0.22368421052631576,
            -0.05263157894736842,
            0.19736842105263158,
        ],
        [
            -0.5225563909774436,
            -0.8139097744360901,
            -0.3007518796992481,
            0.306390977443609,
        ],
    ]);
    assert_eq!(a_4x4.inverse_4x4(), Ok(inverse_4x4));
}

#[test]
fn test_inverse_of_4x4_test_two() {
    // multiplying matrix to a vector
    let a_4x4 = Matrix::from([
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    ]);
    let inverse_4x4 = Matrix::from([
        [
            -0.15384615384615385,
            -0.15384615384615385,
            -0.28205128205128205,
            -0.5384615384615384,
        ],
        [
            -0.07692307692307693,
            0.12307692307692308,
            0.02564102564102564,
            0.03076923076923077,
        ],
        [
            0.358974358974359,
            0.358974358974359,
            0.4358974358974359,
            0.9230769230769231,
        ],
        [
            -0.6923076923076923,
            -0.6923076923076923,
            -0.7692307692307693,
            -1.9230769230769231,
        ],
    ]);
    assert_eq!(a_4x4.inverse_4x4(), Ok(inverse_4x4));
}
#[test]
fn test_inverse_of_4x4_test_three() {
    // multiplying matrix to a vector
    let a_4x4 = Matrix::from([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ]);
    let inverse_4x4 = Matrix::from([
        [
            -0.04074074074074074,
            -0.07777777777777778,
            0.14444444444444443,
            -0.2222222222222222,
        ],
        [
            -0.07777777777777778,
            0.03333333333333333,
            0.36666666666666664,
            -0.3333333333333333,
        ],
        [
            -0.029012345679012345,
            -0.14629629629629629,
            -0.10925925925925925,
            0.12962962962962962,
        ],
        [
            0.17777777777777778,
            0.06666666666666667,
            -0.26666666666666666,
            0.3333333333333333,
        ],
    ]);
    assert_eq!(a_4x4.inverse_4x4(), Ok(inverse_4x4));
}

#[test]
fn test_axb_equals_c_where_c_multiply_by_inverse_of_b_gives_a() {
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

    let mul = Matrix::from([
        [20.0, 22.0, 50.0, 48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0, 46.0, 42.0],
    ]);

    assert_eq!(mul * b.inverse_4x4().unwrap(), a)
}

#[test]
fn inverse_of_identity_is_identity() {
    let i = Matrix::identity_4x4();
    assert_eq!(i.clone().inverse_4x4(), Ok(i));
}

#[test]
fn multiplying_a_matrix_with_its_inverse_gives_itentity() {
    let a_4x4 = Matrix::from([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ]);
    let i = Matrix::identity_4x4();
    assert_eq!(a_4x4.clone() * a_4x4.inverse_4x4().unwrap(), i);
}

#[test]
fn the_transformation_matrix_for_the_default_orientation() {
    let from = Point::new(0.0, 0.0, 0.0);
    let to = Point::new(0.0, 0.0, -1.0);
    let up = Vec4::new(0.0, 1.0, 0.0);

    let view_transformation = Matrix::view_transformation(from, to, up);
    assert_eq!(Matrix::identity_4x4(), view_transformation);
}

#[test]
fn a_view_transformation_matrix_looking_in_positive_z_direction() {
    let from = Point::new(0.0, 0.0, 0.0);
    let to = Point::new(0.0, 0.0, 1.0);
    let up = Vec4::new(0.0, 1.0, 0.0);

    let view_transformation = Matrix::view_transformation(from, to, up);
    assert_eq!(
        Matrix::scaling_mat_4x4(-1.0, 1.0, -1.0),
        view_transformation
    );
}

#[test]
fn the_view_transformation_moves_the_world() {
    let from = Point::new(0.0, 0.0, 8.0);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vec4::new(0.0, 1.0, 0.0);

    let view_transformation = Matrix::view_transformation(from, to, up);
    assert_eq!(
        Matrix::translation_mat_4x4(0.0, 0.0, -8.0),
        view_transformation
    );
}

#[test]
fn an_arbitrary_view_transformation() {
    let from = Point::new(1.0, 3.0, 2.0);
    let to = Point::new(4.0, -2.0, 8.0);
    let up = Vec4::new(1.0, 1.0, 0.0);

    let view_transformation = Matrix::view_transformation(from, to, up);
    assert_eq!(
        Matrix::from([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000]
        ]),
        view_transformation
    );
}
