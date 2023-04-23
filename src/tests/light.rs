use crate::{canvas::Color, matters::light::Light, vector::Point};

#[test]
fn a_point_light_has_a_position_and_intensity() {
    let position = Point::origin();
    let color = Color::new(1.0, 1.0, 1.0);
    let light = Light::new(position.clone(), color.clone());
    assert_eq!(light.position, position);
    assert_eq!(light.intensity, color);
}
