#![allow(dead_code)]
#![feature(io_error_other)]

#[macro_use]
mod command;
mod typewriter;

use command::{OFFLINE, ONLINE};
use typewriter::Typewriter;
use std::io;

use crate::command::{SetCharWidth, Move};



fn main() -> io::Result<()> {
    let mut gabby = Typewriter::new()?;
    gabby.send(&ONLINE)?;

    gabby.send(&command::Write::string("HHHH"))?;
    gabby.send(&cmds![SetCharWidth::new(12), Move::down(12), Move::left(48)])?;
    gabby.send(&command::Write::string("HHHH"))?;
    gabby.send(&cmds![SetCharWidth::new(12), Move::down(16), Move::left(48)])?;
    gabby.send(&command::Write::string("HHHH"))?;
    gabby.send(&cmds![SetCharWidth::new(12), Move::down(20), Move::left(48)])?;
    gabby.send(&command::Write::string("HHHH"))?;

    gabby.send(&OFFLINE)?;
    Ok(())
}
