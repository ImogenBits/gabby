#![allow(dead_code)]
#![feature(io_error_other)]

#[macro_use]
mod command;
mod typewriter;

use image::{imageops::FilterType::Gaussian, DynamicImage, Luma, GrayImage};
use std::io;
use rand::{thread_rng, Rng};
use typewriter::Typewriter;

fn print_image(image: &DynamicImage, width: u16) {
    let height = (image.height() as f64 / image.width() as f64 * width as f64) as u32;
    let mut rng = thread_rng();
    println!(
        "{}",
        image
            .resize_exact(width as u32, height, Gaussian)
            .grayscale()
            .into_luma8()
            .rows()
            .map(|r| {
                r.map(|p| if p.0[0] < rng.gen_range(0..=255) { '.' } else { ' ' })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn test_image(image: &DynamicImage, width: u16) {
    let height = (image.height() as f64 / image.width() as f64 * width as f64) as u32;
    let mut rng = thread_rng();
    let mut image = image
        .resize_exact(width as u32, height, Gaussian)
        .grayscale()
        .into_luma8();
    for p in image.pixels_mut() {
        p.0[0] = if p.0[0] < rng.gen_range(0..=255) { 0 } else { 255 };
    }
    let _ = image.save("out.png");
}

fn main() -> io::Result<()> {
    //let mut gabby = Typewriter::new()?;

    let image = image::open("me.png").unwrap();
    test_image(&image, 500);
    //gabby.print_image(&image, 100);

    Ok(())
}
