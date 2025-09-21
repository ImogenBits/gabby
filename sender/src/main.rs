#![allow(dead_code)]
#![feature(io_error_other)]

#[macro_use]
mod command;
mod typewriter;

use image::{imageops::{FilterType::Triangle, colorops::dither, BiLevel}, DynamicImage, GrayImage};
use std::io;
use typewriter::Typewriter;

fn print_image(image: &GrayImage) {
    println!(
        "{}",
        image
            .rows()
            .map(|r| {
                r.map(|p| if p.0[0] < 128 { '.' } else { ' ' })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn preprocess_image(image: &DynamicImage, width: u16) -> GrayImage {
    let height = (image.height() as f64 / image.width() as f64 * width as f64) as u32;
    let mut image = image
        .resize_exact(width as u32, height, Triangle)
        .grayscale()
        .into_luma8();
    let color_map = BiLevel;
    dither(&mut image, &color_map);
    image
}

fn main() -> io::Result<()> {
    
    let image = image::open("me.png").unwrap();
    let image = preprocess_image(&image, 40);
    let _ = image.save("out.png");
    let num_dots = image
    .enumerate_pixels()
    .filter(|(_, _, p)| p.0[0] < 128)
    .count();
    println!("{num_dots}");
    let mut gabby = Typewriter::new()?;
    gabby.print_image(&image);

    Ok(())
}
