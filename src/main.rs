#![allow(dead_code)]
#![feature(io_error_other)]

#[macro_use]
mod command;
mod typewriter;

use image::{imageops::FilterType::Gaussian, DynamicImage};
use std::{io, iter::once};
use typewriter::Typewriter;

fn print_image(image: &DynamicImage, width: u16) {
    let height = (image.height() as f64 / image.width() as f64 * width as f64) as u32;
    println!(
        "{}",
        image
            .resize_exact(width as u32, height, Gaussian)
            .grayscale()
            .into_luma8()
            .rows()
            .flat_map(|r| {
                r.map(|p| if p.0[0] < 128 { '.' } else { ' ' })
                    .chain(once('\n'))
            })
            .collect::<String>()
    );
}

fn main() -> io::Result<()> {
    //let mut gabby = Typewriter::new()?;

    let image = image::open("transgenderlogo.jpg").unwrap();
    print_image(&image, 100);
    //gabby.print_image(&image, 100);

    Ok(())
}
