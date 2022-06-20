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

    gabby.feed_direction = None;
    for i in 1..=12 {
        gabby.print_char('.');
        gabby.move_head(i, 0);
        gabby.print_char('.');
        gabby.newline();
        gabby.carriage_return();
    }

    gabby.off();
    Ok(())
}
