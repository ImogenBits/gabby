#![allow(dead_code)]
#![feature(io_error_other)]

#[macro_use]
mod command;
mod typewriter;

use command::{OFFLINE, ONLINE};
use std::io;
use typewriter::Typewriter;

use crate::command::{Move, SetCharWidth};

fn main() -> io::Result<()> {
    let mut gabby = Typewriter::new()?;
    gabby.send_raw(&ONLINE);

    gabby.send_raw(&command::PrintChar::string("HHHH"));
    gabby.send_raw(&cmds![
        SetCharWidth::new(12),
        Move::down(12),
        Move::left(48)
    ]);
    gabby.send_raw(&command::PrintChar::string("HHHH"));
    gabby.send_raw(&cmds![
        SetCharWidth::new(12),
        Move::down(16),
        Move::left(48)
    ]);
    gabby.send_raw(&command::PrintChar::string("HHHH"));
    gabby.send_raw(&cmds![
        SetCharWidth::new(12),
        Move::down(20),
        Move::left(48)
    ]);
    gabby.send_raw(&command::PrintChar::string("HHHH"));

    gabby.send_raw(&OFFLINE);
    Ok(())
}
