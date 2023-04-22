use crate::{
    canvas::{Canvas, Color},
    vector::{Point, Vec4},
};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Projectile {
    pub position: Point,
    velocity: Vec4,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vec4) -> Self {
        Self { position, velocity }
    }
    pub fn to_ppm_file(
        &mut self,
        env: &Environment,
        img_width: u64,
        img_height: u64,
        projectile_color: &Color,
    ) {
        let mut canvas = Canvas::new(img_width, img_height);
        let aspect_ratio = canvas.aspect_ratio();
        while self.position.1 > 0.0 {
            canvas.write_pixel(
                (
                    (self.position.0 * aspect_ratio) as usize,
                    (canvas.height as f64 - (self.position.1 * aspect_ratio)) as usize,
                ),
                &projectile_color,
            );
            tick(self, &env);
        }

        let path = std::path::Path::new(".\\projectile.ppm");
        if !path.exists() {
            std::fs::File::create(&path).unwrap();
        }
        let mut file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
        file.write_all(canvas.to_ppm().as_bytes()).unwrap();
    }
}

pub struct Environment {
    gravity: Vec4,
    wind: Vec4,
}

impl Environment {
    pub fn new(gravity: Vec4, wind: Vec4) -> Self {
        Self { gravity, wind }
    }
}

pub fn tick<'a>(proj: &'a mut Projectile, env: &Environment) -> &'a mut Projectile {
    proj.position = proj.position.clone() + proj.velocity.clone();
    proj.velocity = proj.velocity.clone() + env.gravity.clone() + env.wind.clone();
    proj
}

// call this function in main function to create ppm file with projectile
pub fn draw_projectile_on_ppm_file() {
    let mut projectile = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        11.25 * Vec4::new(1.0, 1.8, 0.0).normalize(),
    );
    let env = Environment::new(Vec4::new(0.0, -0.1, 0.0), Vec4::new(-0.01, 0.0, 0.0));
    projectile.to_ppm_file(&env, 900, 550, &Color::new(1.0, 0.0, 0.0));
}
