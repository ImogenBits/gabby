#![allow(dead_code)]
#![feature(io_error_other, result_flattening)]
mod command;

use command::{Command, OFFLINE, ONLINE, HorizontalDir};
use hex::encode_upper;
use std::io::{self, Read, Write};
use std::net::TcpStream;

struct Typewriter {
    stream: TcpStream,
}

impl Typewriter {
    fn new() -> io::Result<Typewriter> {
        let stream = TcpStream::connect("192.168.178.25:80")?;
        Ok(Self { stream })
    }

    fn send(&mut self, data: &[&dyn Command]) -> io::Result<Vec<u8>> {
        let mut ret = vec![];
        for cmd in data {
            let cmd = cmd.encode();
            self.stream.write(&cmd.to_be_bytes())?;
            let mut buf = [0];
            self.stream.read_exact(&mut buf)?;
            let len = buf[0];
            let mut reply = Vec::with_capacity(len as usize);
            (&mut self.stream)
                .take(len as u64)
                .read_to_end(&mut reply)?;
            ret.extend(reply);
        }
        Ok(ret)
    }

    fn receive(&mut self) -> io::Result<u8> {
        self.stream.set_nonblocking(true)?;
        let r = (&mut self.stream)
            .bytes()
            .next()
            .ok_or(io::Error::other(""))
            .flatten();
        self.stream.set_nonblocking(false)?;
        r
    }

    fn data_available(&self) -> bool {
        let _ = self.stream.set_nonblocking(true);
        let r = self.stream.peek(&mut [0]).is_ok();
        let _ = self.stream.set_nonblocking(false);
        r
    }
}

fn main() -> io::Result<()> {
    let mut gabby = Typewriter::new()?;
    let r = encode_upper(&gabby.send(&ONLINE)?);
    println!("online: {r}");

    let c = command::Write {letter: 31, thickness: 42, movement: Some(HorizontalDir::Right)};
    let r = gabby.send(&[&c, &c, &c, &c, &c, &c, &c])?;
    println!("write S: {}", encode_upper(r));

    let r = gabby.send(&OFFLINE)?;
    println!("offline: {}", encode_upper(r));

    
    while gabby.data_available() {
        println!("{}", encode_upper(&[gabby.receive()?]));
    }

    Ok(())
}
