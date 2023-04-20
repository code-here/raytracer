use crate::canvas::{Canvas, Color};

#[test]
fn colors_red_green_blue() {
    let c = Color::from([-0.5, 0.4, 1.7]);
    assert_eq!(c.red(), -0.5);
    assert_eq!(c.green(), 0.4);
    assert_eq!(c.blue(), 1.7);
}

#[test]
fn test_color_at_a_pixel_after_writing() {
    let red = Color::from([1.0, 0.0, 0.0]);
    let mut canvas = Canvas::new(20, 20);
    assert_eq!(canvas.pixel_at((2, 3)), &Color::black());
    canvas.write_pixel((2, 3), &red);
    assert_eq!(canvas.pixel_at((2, 3)), &red);
}

#[test]
fn test_canvas_to_ppm() {
    let mut canvas = Canvas::new(5, 3);
    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);
    canvas.write_pixel((0, 0), &c1);
    canvas.write_pixel((2, 1), &c2);
    canvas.write_pixel((4, 2), &c3);

    assert_eq!(
        canvas.to_ppm(),
        r#"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 
"#
    );
}
