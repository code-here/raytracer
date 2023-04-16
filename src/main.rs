use canvas::Color;

use crate::{
    projectile::{Environment, Projectile},
    vector::{Point, Vec4},
};

mod canvas;
mod matrix;
mod projectile;
#[cfg(test)]
mod tests;
mod vector;

fn main() {
    let mut projectile = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        11.25 * Vec4::new(1.0, 1.8, 0.0).normalize(),
    );
    let env = Environment::new(Vec4::new(0.0, -0.1, 0.0), Vec4::new(-0.01, 0.0, 0.0));
    projectile.to_ppm_file(&env, 900, 550, &Color::new(1.0, 0.0, 0.0));
}
