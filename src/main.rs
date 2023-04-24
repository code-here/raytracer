mod canvas;
mod matrix;
mod matters;
mod projectile;
mod ray;
#[cfg(test)]
mod tests;
mod vector;
mod world;

fn main() {
    // projectile::draw_projectile_on_ppm_file();
    // matrix::clock_to_ppm_file();
    let mut sphere = crate::matters::sphere::Sphere::default();
    // sphere.simple_sphere_to_canvas();
    sphere.sphere_with_lighting_to_canvas();
}
