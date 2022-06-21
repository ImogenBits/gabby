#![allow(dead_code)]
#![feature(io_error_other)]

#[macro_use]
mod command;
mod typewriter;

use image::{imageops::FilterType::Gaussian, DynamicImage};
use std::io;
use typewriter::Typewriter;

fn to_bitmap(image: &DynamicImage, width: u16) -> Vec<Vec<bool>> {
    let height = (image.height() as f64 / image.width() as f64 * width as f64) as u32;
    image
        .resize_exact(width as u32, height, Gaussian)
        .grayscale()
        .into_luma8()
        .rows()
        .map(|r| r.map(|p| p.0[0] < 128).collect())
        .collect()
}

fn print_bitmap(bitmap: Vec<Vec<bool>>) {
    let out = bitmap
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|b| if b { '.' } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{out}");
}

fn main() -> io::Result<()> {
    let mut gabby = Typewriter::new()?;
    gabby.on();

    let image = image::open("transgenderlogo.jpg").unwrap();
    let bitmap = to_bitmap(&image, 100);
    print_bitmap(bitmap);
    //gabby.print_image(&image, 100);

    gabby.off();
    Ok(())
}
