#![allow(dead_code)]
#![feature(io_error_other)]
mod command;

use command::{Command, OFFLINE, ONLINE};
use std::io::{self, Read, Write};
use std::net::TcpStream;

struct Typewriter {
    stream: TcpStream,
}

impl Typewriter {
    fn new() -> io::Result<Typewriter> {
        let stream = TcpStream::connect("192.168.178.25:80")?;
        stream.set_nonblocking(true)?;
        Ok(Self { stream })
    }

    fn send(&mut self, data: &[&dyn Command]) -> io::Result<Option<Vec<u8>>> {
        for cmd in data {
            let cmd = cmd.encode();
            self.stream.write(&cmd.to_be_bytes())?;
            if (cmd & 0xF000) == 0xA000 {
                let mut buf = [0];
                self.stream.read_exact(&mut buf)?;
                let reply = buf[0];
                if reply == 0xA4 {
                    let reply_data = (&mut self.stream)
                        .bytes()
                        .map_while(Result::ok)
                        .take_while(|x| *x != 0)
                        .collect::<Vec<_>>();
                    return Ok(Some(reply_data));
                }
            }
        }
        Ok(None)
    }

    fn receive(&mut self) -> io::Result<Option<u8>> {
        match (&mut self.stream).bytes().next() {
            Some(Ok(c)) => Ok(Some(c)),
            Some(Err(ref e)) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }
}

fn main() -> io::Result<()> {
    let mut gabby = Typewriter::new()?;
    gabby.send(&ONLINE)?;
    gabby.send(&OFFLINE)?;

    Ok(())
}
