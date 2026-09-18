

use std::io::Write;

use super::vec3;

#[derive(Copy, Clone)]
pub struct Colour(pub vec3::Vec3);


fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_colour<W: Write>(writer: &mut W, colour: Colour) {
    let r = linear_to_gamma(colour.r()).clamp(0.0, 0.999);
    let g = linear_to_gamma(colour.g()).clamp(0.0, 0.999);
    let b = linear_to_gamma(colour.b()).clamp(0.0, 0.999);

    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    writeln!(writer, "{} {} {}", ir, ig, ib).unwrap();
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour(vec3::Vec3::new(r, g, b))
    }

    pub fn r(&self) -> f64 {
        self.0.x
    }

    pub fn g(&self) -> f64 {
        self.0.y
    }

    pub fn b(&self) -> f64 {
        self.0.z
    }
}