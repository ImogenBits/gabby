#![allow(dead_code)]
#![feature(io_error_other)]

#[macro_use]
mod command;
mod typewriter;

use std::io;
use typewriter::Typewriter;

fn main() -> io::Result<()> {
    let mut gabby = Typewriter::new()?;
    gabby.on();

    gabby.print_string("I am a Wah!\nand I need many headpats :3");

    gabby.off();
    Ok(())
}
