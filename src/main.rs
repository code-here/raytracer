mod canvas;
mod matrix;
mod matters;
mod projectile;
mod ray;
#[cfg(test)]
mod tests;
mod vector;

fn main() {
    // projectile::draw_projectile_on_ppm_file();
    // matrix::clock_to_ppm_file();
    let mut sphere = crate::matters::Sphere::default();
    // sphere.transformation = sphere
    //     .transformation
    //     .translation_mat_4x4_chain(125.0, 125.0, 0.0);

    sphere.simple_sphere_to_canvas();
}
