use std::io::Write;
use std::fs;

use super::sphere::Color;

fn clamp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

fn to_int(x: f64) -> u8 {
    (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as u8
}

pub fn save_ppm_file(filename: &str, image: Vec<Color>, width: u32, height: u32) {
    let mut f = fs::File::create(filename).unwrap();
    writeln!(f, "P3\n{} {}\n{}", width, height, 255).unwrap();
    for i in 0..(width * height) {
        write!(f, "{} {} {} ", to_int(image[i as usize].x), to_int(image[i as usize].y), to_int(image[i as usize].z)).unwrap();
    }
}

