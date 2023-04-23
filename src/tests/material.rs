use crate::{
    canvas::Color,
    matters::{light::Light, material::Material},
    vector::{Point, Vec4},
};

#[test]
fn default_material() {
    let material = Material::default();
    assert_eq!(material.color, Color::white());
    assert_eq!(material.ambient, 0.1);
    assert_eq!(material.diffuse, 0.9);
    assert_eq!(material.specular, 0.9);
    assert_eq!(material.shininess, 200.0);
}

#[test]
fn lighting_with_eye_between_the_light_and_surface() {
    let eye_vector = Vec4::new(0.0, 0.0, -1.0);
    let normal_vector = Vec4::new(0.0, 0.0, -1.0);
    let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let mut material = Material::default();
    assert_eq!(
        material.lighting(
            &light,
            &Point::new(0.0, 0.0, 0.0),
            &eye_vector,
            &normal_vector
        ),
        Color::new(1.9, 1.9, 1.9)
    );
}

#[test]
fn lighting_with_eye_between_the_light_and_surface_with_eye_offset_45_deg() {
    let eye_vector = Vec4::new(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
    let normal_vector = Vec4::new(0.0, 0.0, -1.0);
    let light = Light::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let mut material = Material::default();
    assert_eq!(
        material.lighting(
            &light,
            &Point::new(0.0, 0.0, 0.0),
            &eye_vector,
            &normal_vector
        ),
        Color::new(1.0, 1.0, 1.0)
    );
}

#[test]
fn lighting_with_eye_opposite_surface_with_light_offset_45_deg() {
    let eye_vector = Vec4::new(0.0, 0.0, -1.0);
    let normal_vector = Vec4::new(0.0, 0.0, -1.0);
    let light = Light::new(Point::new(0.0, 10.0, -10.0), Color::white());
    let mut material = Material::default();
    assert_eq!(
        material.lighting(
            &light,
            &Point::new(0.0, 0.0, 0.0),
            &eye_vector,
            &normal_vector
        ),
        Color::new(0.7364, 0.7364, 0.7364)
    );
}

#[test]
fn lighting_with_eye_in_path_of_the_reflection_vector() {
    let eye_vector = Vec4::new(0.0, -2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
    let normal_vector = Vec4::new(0.0, 0.0, -1.0);
    let light = Light::new(Point::new(0.0, 10.0, -10.0), Color::white());
    let mut material = Material::default();
    assert_eq!(
        material.lighting(
            &light,
            &Point::new(0.0, 0.0, 0.0),
            &eye_vector,
            &normal_vector
        ),
        Color::new(1.6364, 1.6364, 1.6364)
    );
}

#[test]
fn lighting_with_light_behind_the_surface() {
    let eye_vector = Vec4::new(0.0, 0.0, -1.0);
    let normal_vector = Vec4::new(0.0, 0.0, -1.0);
    let light = Light::new(Point::new(0.0, 0.0, 10.0), Color::white());
    let mut material = Material::default();
    assert_eq!(
        material.lighting(
            &light,
            &Point::new(0.0, 0.0, 0.0),
            &eye_vector,
            &normal_vector
        ),
        Color::new(0.1, 0.1, 0.1)
    );
}
