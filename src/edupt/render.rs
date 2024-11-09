use rayon::prelude::*;

use std::io::Write;

use super::vec::*;
use super::ray::Ray;
use super::random::Random;
use super::sphere::*;
use super::radiance::radiance;
use super::ppm::save_ppm_file;

pub fn render(width: u32, height: u32, samples: u32, supersamples: u32) {
    let camera_position = Vector::new(50.0, 52.0, 220.0);
    let camera_dir = normalize(&mut Vector::new(0.0, -0.04, -1.0));
    let camera_up = Vector::new(0.0, 1.0, 0.0);

    let screen_width = 30.0 * (width as f64) / (height as f64);
    let screen_height = 30.0;
    let screen_dist = 40.0;
    let screen_x = normalize(&mut cross(&camera_dir, &camera_up)) * screen_width;
    let screen_y = normalize(&mut cross(&screen_x, &camera_dir)) * screen_height;
    let screen_center = camera_position + camera_dir * screen_dist;

    let mut image = vec![Color::zero(); (width * height) as usize];

    println!("{}x{} {} spp", width, height, samples * (supersamples * supersamples));

    let bands: Vec<(usize, &mut [Color])> = image.chunks_mut(width as usize).enumerate().collect();
    bands.into_par_iter().for_each(|(y, band)| {
        let y = height - (y as u32) - 1;
        if (y %30)==0{writeln!(std::io::stderr(), "Rendering (y = {}) {}%", y, 100.0 * (y as f64) / ((height - 1) as f64)).unwrap();}
        let mut rnd = Random::new((y + 1) as u64);
        for x in 0..width {
            for sy in 0..supersamples {
                for sx in 0..supersamples {
                    let mut accumulated_radiance = Color::zero();
                    for _ in 0..samples {
                        let rate = 1.0 / (supersamples as f64);
                        let r1 = (sx as f64) * rate + rate / 2.0;
                        let r2 = (sy as f64) * rate + rate / 2.0;
                        let screen_position = screen_center +
                            screen_x * ((r1 + (x as f64)) / (width as f64) - 0.5) +
                            screen_y * ((r2 + (y as f64)) / (height as f64) - 0.5);
                        let dir = normalize(&mut (screen_position - camera_position));
                        accumulated_radiance = accumulated_radiance +
                            radiance(&Ray::new(camera_position, dir), &mut rnd, 0) / (samples as f64) / ((supersamples * supersamples) as f64);
                    }
                    band[x as usize] = band[x as usize] + accumulated_radiance;
                }
            }
        }
    });

    // single thread loop
    // for y in 0..height {
    //     writeln!(std::io::stderr(), "Rendering (y = {}) {}%", y, 100.0 * (y as f64) / ((height - 1) as f64)).unwrap();
    //     let mut rnd = Random::new((y + 1) as u64);
    //     for x in 0..width {
    //         let image_index = (height - y - 1) * width + x;
    //         for sy in 0..supersamples {
    //             for sx in 0..supersamples {
    //                 let mut accumulated_radiance = Color::zero();
    //                 for _ in 0..samples {
    //                     let rate = 1.0 / (supersamples as f64);
    //                     let r1 = (sx as f64) * rate + rate / 2.0;
    //                     let r2 = (sy as f64) * rate + rate / 2.0;
    //                     let screen_position = screen_center +
    //                         screen_x * ((r1 + (x as f64)) / (width as f64) - 0.5) +
    //                         screen_y * ((r2 + (y as f64)) / (height as f64) - 0.5);
    //                     let dir = normalize(&mut (screen_position - camera_position));
    //                     accumulated_radiance = accumulated_radiance +
    //                         radiance(&Ray::new(camera_position, dir), &mut rnd, 0) / (samples as f64) / ((supersamples * supersamples) as f64);
    //                 }
    //                 image[image_index as usize] = image[image_index as usize] + accumulated_radiance;
    //             }
    //         }
    //     }
    // }

    save_ppm_file("image.ppm", image, width, height);
}
