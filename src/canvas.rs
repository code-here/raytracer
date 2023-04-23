use std::ops::{Add, Mul, Sub};

// forth value for alpha value
#[derive(Debug, Clone)]
pub struct Color(f64, f64, f64, f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(r, g, b, 1.0)
    }
    pub fn black() -> Self {
        Self(0.0, 0.0, 0.0, 1.0)
    }

    pub fn white() -> Self {
        Self(1.0, 1.0, 1.0, 1.0)
    }

    pub fn red(&self) -> f64 {
        self.0
    }
    pub fn green(&self) -> f64 {
        self.1
    }
    pub fn blue(&self) -> f64 {
        self.2
    }
    pub fn rgb(&self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }
    pub fn red_255(&self) -> u8 {
        let r = self.red() * 256.0;
        if r < 0.0 {
            0
        } else if r > 255.0 {
            255
        } else {
            r as u8
        }
    }
    pub fn green_255(&self) -> u8 {
        let g = self.green() * 256.0;
        if g < 0.0 {
            0
        } else if g > 255.0 {
            255
        } else {
            g as u8
        }
    }
    pub fn blue_255(&self) -> u8 {
        let b = self.blue() * 256.0;
        if b < 0.0 {
            0
        } else if b > 255.0 {
            255
        } else {
            b as u8
        }
    }
    pub fn rgb_255(&self) -> (u8, u8, u8) {
        (self.red_255(), self.green_255(), self.blue_255())
    }
}

impl From<[f64; 3]> for Color {
    fn from(value: [f64; 3]) -> Self {
        Self(value[0], value[1], value[2], 1.0)
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, 1.0)
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, 1.0)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0, self * rhs.1, self * rhs.2, 1.0)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2, 1.0)
    }
}

impl Add<&Color> for &Color {
    type Output = Color;
    fn add(self, rhs: &Color) -> Self::Output {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, 1.0)
    }
}

impl Sub<&Color> for &Color {
    type Output = Color;
    fn sub(self, rhs: &Color) -> Self::Output {
        Color(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, 1.0)
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        Color(self * rhs.0, self * rhs.1, self * rhs.2, 1.0)
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        Color(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2, 1.0)
    }
}

pub struct Canvas {
    pub width: u64,
    pub height: u64,
    data: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: u64, height: u64) -> Self {
        Self {
            width,
            height,
            data: (0..height)
                .map(|_| (0..width).map(|_| Color::black()).collect::<Vec<Color>>())
                .collect::<Vec<_>>(),
        }
    }
    // the canvas top left corner is (0,0) so from top to down is +ve y  and left to right is +ve x
    // so if you are drawing from a 4 quadrant coordiante system convert it to above before writing the point to canvas
    pub fn write_pixel(&mut self, position: (usize, usize), color: &Color) {
        let (x, y) = position;
        // check if the point to be drawn is inside canvas
        if !(y >= self.data.len() || x >= self.data[y].len()) {
            self.data[y][x] = color.clone();
        }
    }

    // this draws the image upside down since we are substracting canvas height with y coordinate
    // so bottom left is (0,0)
    // use this to draw from a normalized world cordinates
    pub fn write_pixel_with_aspect_ratio(&mut self, position: (f64, f64), color: &Color) {
        let (x, y) = position;
        let aspect_ratio = self.aspect_ratio();
        let x = (x * aspect_ratio) as usize;
        let y = (self.height as f64 - (y * aspect_ratio)) as usize;
        // check if the point to be drawn is inside canvas
        if !(y >= self.data.len() || x >= self.data[y].len()) {
            self.data[y][x] = color.clone();
        }
    }

    pub fn pixel_at(&self, position: (usize, usize)) -> &Color {
        let (x, y) = position;
        &self.data[y][x]
    }
    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
        for vc in &self.data {
            for c in vc {
                let (r, g, b) = c.rgb_255();
                ppm.push_str(&format!("{} {} {} ", r, g, b));
            }
            ppm.push('\n');
        }
        ppm
    }
    pub fn aspect_ratio(&self) -> f64 {
        self.height as f64 / self.width as f64
    }
}

impl AsRef<Color> for Color {
    fn as_ref(&self) -> &Color {
        self
    }
}

impl PartialEq for Color {
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
