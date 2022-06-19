#![allow(dead_code)]
#![feature(io_error_other, result_flattening)]
mod command;
mod typewriter;

use command::{HorizontalDir, OFFLINE, ONLINE};
use typewriter::Typewriter;
use std::io;
use hex::encode_upper;



fn main() -> io::Result<()> {
    let mut gabby = Typewriter::new()?;
    let r = encode_upper(&gabby.send(&ONLINE)?);
    println!("online: {r}");

    let c = command::Write {
        letter: 31,
        thickness: 42,
        movement: Some(HorizontalDir::Right),
    };
    let r = gabby.send(&[&c, &c, &c, &c, &c, &c, &c])?;
    println!("write S: {}", encode_upper(r));

    let r = gabby.send(&OFFLINE)?;
    println!("offline: {}", encode_upper(r));

    while let Some(c) = gabby.receive() {
        println!("{}", encode_upper(&[c]));
    }

    Ok(())
}
