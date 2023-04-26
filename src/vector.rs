use std::ops::{Add, Mul, Neg, Sub};

// (x, y, z, w)
#[derive(Debug, Clone)]
pub struct Point(pub f64, pub f64, pub f64, pub f64);
// we could have made a type alias like type Vec4 = Point but since we need 1.0 for points na 0.0 for vectors in last coordinate
#[derive(Debug, Clone)]
pub struct Vec4(pub f64, pub f64, pub f64, pub f64);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 1.0)
    }

    pub fn origin() -> Self {
        Self(0.0, 0.0, 0.0, 1.0)
    }
}

impl From<[f64; 4]> for Point {
    fn from(value: [f64; 4]) -> Self {
        let [x, y, z, w] = value;
        Point(x, y, z, w)
    }
}

impl From<[f64; 4]> for Vec4 {
    fn from(value: [f64; 4]) -> Self {
        let [x, y, z, w] = value;
        Vec4(x, y, z, w)
    }
}
impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 0.0)
    }

    // TODO: writes tests for these
    pub fn square_of_magnitude(&self) -> f64 {
        let Vec4(x, y, z, w) = self;
        x * x + y * y + z * z + w * w
    }

    pub fn magnitude(&self) -> f64 {
        self.square_of_magnitude().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = 1.0 / self.magnitude();
        Self::new(mag * self.0, mag * self.1, mag * self.2)
    }

    pub fn dot(&self, other: &Vec4) -> f64 {
        let Vec4(x1, y1, z1, _) = self;
        let Vec4(x2, y2, z2, _) = other;
        x1 * x2 + y1 * y2 + z1 * z2
    }

    pub fn cross(&self, other: &Vec4) -> Vec4 {
        let Vec4(x1, y1, z1, _) = self;
        let Vec4(x2, y2, z2, _) = other;
        Self::new(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
    }
    // provide the reflected vector for a input vector in and normal
    pub fn reflect(&self, normal: &Vec4) -> Self {
        let input = self;
        input.as_ref() - (2.0 * input.dot(&normal) * normal).as_ref()
    }
}

// p1+ v1-> p2; p1 travels over v1 to get to p2
impl Add<Vec4> for Point {
    type Output = Point;
    fn add(self, rhs: Vec4) -> Self::Output {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Add<&Vec4> for &Point {
    type Output = Point;
    fn add(self, rhs: &Vec4) -> Self::Output {
        Point(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

// v1+ v2= v3; here v3 is the resultant/effective vector of v1 and v2
impl Add<Vec4> for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Self::Output {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

// p1- v1= p2; here p2 travellers backward on v1 to get to p2
impl Sub<Vec4> for Point {
    type Output = Point;
    fn sub(self, rhs: Vec4) -> Self::Output {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

// p1 - p2 = v1; here v1 is a vector from p2 to p1
impl Sub<Point> for Point {
    type Output = Vec4;
    fn sub(self, rhs: Point) -> Self::Output {
        Vec4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Sub<&Point> for &Point {
    type Output = Vec4;
    fn sub(self, rhs: &Point) -> Self::Output {
        Vec4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

// v1 - v2 -> v3
impl Sub<Vec4> for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Vec4) -> Self::Output {
        Vec4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Sub<&Vec4> for &Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: &Vec4) -> Self::Output {
        Vec4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

// vector with it's opposite direction
impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2, self.3)
    }
}

// TODO: write tests for below

// multiply the magnitude of a vector
impl Mul<Vec4> for f64 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<&Vec4> for f64 {
    type Output = Vec4;
    fn mul(self, rhs: &Vec4) -> Self::Output {
        Vec4::new(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let small_value = 0.00001;
        if (self.0 - other.0).abs() < small_value
            && (self.1 - other.1).abs() < small_value
            && (self.2 - other.2).abs() < small_value
            && (self.3 - other.3).abs() < small_value
        {
            true
        } else {
            false
        }
    }
}

impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        let small_value = 0.00001;
        if (self.0 - other.0).abs() < small_value
            && (self.1 - other.1).abs() < small_value
            && (self.2 - other.2).abs() < small_value
            && (self.3 - other.3).abs() < small_value
        {
            true
        } else {
            false
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl AsRef<Vec4> for Vec4 {
    fn as_ref(&self) -> &Vec4 {
        self
    }
}

impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Point {
        self
    }
}
