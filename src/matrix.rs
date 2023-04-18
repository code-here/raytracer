use std::{cmp::PartialEq, f64::EPSILON, ops::Mul};

use crate::vector::{Point, Vec4};

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl Matrix {
    pub fn identity_4x4() -> Self {
        Self(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn zero(row: usize, col: usize) -> Self {
        let mut res = Vec::with_capacity(row);
        for _ in 0..row {
            let mut temp = Vec::with_capacity(col);
            for _ in 0..col {
                temp.push(0.0)
            }
            res.push(temp);
        }
        Matrix(res)
    }
    pub fn rows(&self) -> usize {
        self.0.len()
    }
    pub fn cols(&self) -> usize {
        self.0[0].len()
    }
    pub fn transpose(&self) -> Self {
        let mut trans = Matrix::zero(self.rows(), self.cols());
        for (row, rv) in self.0.iter().enumerate() {
            for (col, v) in rv.iter().enumerate() {
                trans.0[col][row] = *v;
            }
        }
        trans
    }
    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut smatrix = self.0.clone();
        smatrix.remove(row);
        let smatrix = smatrix
            .into_iter()
            .map(|mut r| {
                let _ = r.remove(col);
                r
            })
            .collect();
        Self(smatrix)
    }

    pub fn det_2x2(&self) -> Result<f64, &str> {
        if !self.check_size(2) {
            Err("not a 2x2 matrix")
        } else {
            let Self(mat) = self;
            Ok(mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0])
        }
    }

    pub fn minor_3x3(&self, row: usize, col: usize) -> Result<f64, &str> {
        if !self.check_size(3) {
            Err("not a 3x3 matrix")
        } else {
            let sub = self.submatrix(row, col);
            Ok(sub.det_2x2().unwrap())
        }
    }

    pub fn cofactor_3x3(&self, row: usize, col: usize) -> Result<f64, &str> {
        if !self.check_size(3) {
            Err("not a 3x3 matrix")
        } else {
            let minor = self.minor_3x3(row, col)?;
            if (row + col) % 2 == 0 {
                Ok(minor)
            } else {
                Ok(-minor)
            }
        }
    }

    pub fn det_3x3(&self) -> Result<f64, &str> {
        if !self.check_size(3) {
            Err("not a 3x3 matrix")
        } else {
            let mut det = 0.0;
            // does not mat cols or rows; since it's a square matrix
            for col in 0..3 {
                det = det + self.0[0][col] * self.cofactor_3x3(0, col)?;
            }
            Ok(det)
        }
    }
    pub fn det_4x4(&self) -> Result<f64, &str> {
        if !self.check_size(4) {
            Err("not a square matrix")
        } else {
            let mut det = 0.0;
            let mut sign = 1.0;
            // does not mat cols or rows; since it's a square matrix
            for col in 0..self.cols() {
                det = det + sign * self.0[0][col] * self.submatrix(0, col).det_3x3().unwrap();
                // toggle sign
                sign *= -1.0;
            }
            Ok(det)
        }
    }

    pub fn check_size(&self, size: usize) -> bool {
        if self.rows() != size || self.cols() != size {
            false
        } else {
            true
        }
    }

    pub fn cofactor_4x4(&self, row: usize, col: usize) -> Result<f64, &str> {
        if !self.check_size(4) {
            Err("not a 4x4 Matrix")
        } else {
            let sub = self.submatrix(row, col);
            let det = sub.det_3x3().unwrap();
            if (row + col) % 2 == 0 {
                Ok(det)
            } else {
                Ok(-det)
            }
        }
    }

    /// steps to find inverse
    /// 1. take the determinant of matrix.
    /// 2. create a matrix of cofactors
    /// 3. take the transpose of the cofactor matrix
    /// 4. divide every elements of transposed matrix with the determinant taken in the first step
    pub fn inverse_4x4(&self) -> Result<Self, &str> {
        if !self.check_size(4) {
            Err("not a 4x4 matrix")
        } else {
            let one_by_det = 1.0 / self.det_4x4()?;
            let mut inverse = Matrix::zero(4, 4);
            for ridx in 0..4 {
                for cidx in 0..4 {
                    inverse.0[ridx][cidx] = self.cofactor_4x4(ridx, cidx)?;
                }
            }
            inverse = inverse.transpose();
            for ridx in 0..4 {
                for cidx in 0..4 {
                    inverse.0[ridx][cidx] = inverse.0[ridx][cidx] * one_by_det;
                }
            }
            Ok(inverse)
        }
    }

    pub fn translation_mat_4x4(x: f64, y: f64, z: f64) -> Self {
        let Self(mut identity) = Self::identity_4x4();
        (identity[0][3], identity[1][3], identity[2][3]) = (x, y, z);
        Matrix(identity)
    }
}

impl From<[[f64; 4]; 4]> for Matrix {
    fn from(value: [[f64; 4]; 4]) -> Self {
        Self(value.map(|inner| inner.to_vec()).to_vec())
    }
}
impl From<[[f64; 3]; 3]> for Matrix {
    fn from(value: [[f64; 3]; 3]) -> Self {
        Self(value.map(|inner| inner.to_vec()).to_vec())
    }
}
impl From<[[f64; 2]; 2]> for Matrix {
    fn from(value: [[f64; 2]; 2]) -> Self {
        Self(value.map(|inner| inner.to_vec()).to_vec())
    }
}

// create 4x1 matrix from vector
impl From<Vec4> for Matrix {
    fn from(value: Vec4) -> Self {
        let Vec4(x, y, z, w) = value;
        Matrix(vec![vec![x], vec![y], vec![z], vec![w]])
    }
}

// create 4x1 matrix from point
impl From<Point> for Matrix {
    fn from(value: Point) -> Self {
        let Point(x, y, z, w) = value;
        Matrix(vec![vec![x], vec![y], vec![z], vec![w]])
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let m1 = self.0;
        let m2 = rhs.0;
        if m1[0].len() != m2.len() {
            return Matrix(vec![vec![]]);
        }
        let mut m12 = Vec::new();
        for mc in m1 {
            let mut temp = Vec::with_capacity(mc.len());
            for m2col in 0..m2[0].len() {
                let mut temp_sum = 0.0;
                for (m2row, c) in mc.iter().enumerate() {
                    temp_sum += c * m2[m2row][m2col];
                }
                temp.push(temp_sum);
            }
            m12.push(temp);
        }
        Matrix(m12)
    }
}

// custom partialeq to compare floting numbers
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for ridx in 0..self.rows() {
            for cidx in 0..self.cols() {
                // can use less or more zero like  0.000001 or 0.00000000000001 as we want the equation to be accurate
                if !(f64::abs(self.0[ridx][cidx] - other.0[ridx][cidx]) < 0.00000000001) {
                    return false;
                }
            }
        }
        true
    }
}

// for multiplying translation matrix to point
impl Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        let m1 = Matrix::from(rhs);
        let res = self * m1;
        let mut point = [0.0; 4];
        (0..4).for_each(|i| point[i] = res.0[i][0]);
        Point::from(point)
    }
}

// for multiplying translation matrix to vector; should not effect the vector; just writting for tests
impl Mul<Vec4> for Matrix {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        let m1 = Matrix::from(rhs);
        let res = self * m1;
        let mut vec4 = [0.0; 4];
        (0..4).for_each(|i| vec4[i] = res.0[i][0]);
        Vec4::from(vec4)
    }
}
