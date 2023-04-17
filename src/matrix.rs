use std::ops::Mul;

use crate::vector::Vec4;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl Matrix {
    pub fn identiry_4x4() -> Self {
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
        if self.rows() != 2 || self.cols() != 2 {
            Err("not a 2x2 matrix")
        } else {
            let Self(mat) = self;
            Ok(mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0])
        }
    }

    pub fn minor_3x3(&self, row: usize, col: usize) -> Result<f64, &str> {
        if self.rows() != 3 || self.cols() != 3 {
            Err("not a 3x3 matrix")
        } else {
            let sub = self.submatrix(row, col);
            Ok(sub.det_2x2().unwrap())
        }
    }

    pub fn cofactor_3x3(&self, row: usize, col: usize) -> Result<f64, &str> {
        if self.rows() != 3 || self.cols() != 3 {
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
        if self.rows() != 3 || self.cols() != 3 {
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
        if self.rows() != 4 || self.cols() != 4 {
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
