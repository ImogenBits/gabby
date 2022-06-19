#![allow(dead_code)]
mod command;
mod typewriter;

use command::{OFFLINE, ONLINE};
use typewriter::Typewriter;
use std::io;
use hex::encode_upper;



fn main() -> io::Result<()> {
    let mut gabby = Typewriter::new()?;
    let r = encode_upper(&gabby.send(&ONLINE)?);
    println!("online: {r}");

    let r = gabby.send(&command::Write::string("waaah!"))?;
    println!("write S: {}", encode_upper(r));

    let r = gabby.send(&OFFLINE)?;
    println!("offline: {}", encode_upper(r));

    Ok(())
}
