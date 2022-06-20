#![allow(dead_code)]
#![feature(io_error_other)]

#[macro_use]
mod command;
mod typewriter;

use image::{imageops::FilterType::Gaussian, DynamicImage};
use std::io;
use typewriter::Typewriter;

fn to_bitmap(image: DynamicImage, width: u16) -> Vec<Vec<bool>> {
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
    let image = image::open("transgenderlogo.jpg").unwrap();
    let bitmap = to_bitmap(image, 50);
    print_bitmap(bitmap);

    /*  let mut gabby = Typewriter::new()?;
    gabby.on();

    gabby.feed_direction = None;
    for i in [1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1] {
        if i == 1 {
            gabby.print_char('.');
        }
        gabby.move_head(3, 0);
    }




    gabby.off();*/
    Ok(())
}
