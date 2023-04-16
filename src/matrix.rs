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
    pub fn transpose(&self) -> Self {
        let mut trans = Matrix::zero(self.0.len(), self.0[0].len());
        for (row, rv) in self.0.iter().enumerate() {
            for (col, v) in rv.iter().enumerate() {
                trans.0[col][row] = *v;
            }
        }
        trans
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
