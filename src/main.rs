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
    matrix::clock_to_ppm_file();
}
