use std::fs::File;
use std::io::Write;

use rs_tracing::{write_colour, Colour};

fn main() {
    let img_width = 256;
    let img_height = 256;

    let mut file = File::create("output.ppm").unwrap();
    writeln!(file, "P3\n{img_width} {img_height}\n255").unwrap();

    for j in 0..img_height {
        for i in 0..img_width {
            let colour = Colour::new(i as f64 / img_width as f64, j as f64 / img_height as f64, 0.2);
            write_colour(&mut file, colour);
        }
    }
}
