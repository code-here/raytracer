use crate::{
    canvas::Color,
    vector::{Point, Vec4},
};

use super::light::Light;

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(
        &mut self,
        light: &Light,
        point: &Point,
        eye_vector: &Vec4,
        normal_vector: &Vec4,
    ) -> Color {
        let (diffuse, specular, ambient);
        // combine the surface color with the light's color/intensity
        let effective_color = self.color.as_ref() * light.intensity.as_ref();
        // find the direction to the light source
        let light_vector = (light.position.as_ref() - point).normalize();
        // compute the ambient contribution
        ambient = self.ambient * effective_color.as_ref();
        //  light_dot_normal represents the cosine of the angle between the
        //  light vector and the normal vector. A negative number means the
        //  light is on the other side of the surface.
        let light_dot_normal = light_vector.dot(normal_vector.as_ref());
        if light_dot_normal < 0.0 {
            // black
            diffuse = Color::black();
            // black
            specular = Color::black();
        } else {
            // compute the diffuse contribution
            diffuse = light_dot_normal * self.diffuse * &effective_color;
            //  reflect_dot_eye represents the cosine of the angle between the
            //  reflection vector and the eye vector. A negative number means the
            //  light reflects away from the eye.
            let reflect_vector = (-light_vector).reflect(normal_vector.as_ref());
            let reflect_dot_eye = reflect_vector.dot(eye_vector.as_ref());
            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                //  compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = self.specular * factor * light.intensity.as_ref();
            }
        }
        ambient + diffuse + specular
    }
}
