

use std::io::Write;

use super::vec3;

#[derive(Copy, Clone)]
pub struct Colour(pub vec3::Vec3);


pub fn write_colour<W: Write>(writer: &mut W, colour: Colour) {
    let ir = (255.999 * colour.r()) as i32;
    let ig = (255.999 * colour.g()) as i32;
    let ib = (255.999 * colour.b()) as i32;

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