#![allow(dead_code)]

use std::io::{self, Read, Write};
use std::net::TcpStream;
use crate::command::Command;

pub struct Typewriter {
    stream: TcpStream,
}

impl Typewriter {
    pub fn new() -> io::Result<Typewriter> {
        let stream = TcpStream::connect("192.168.178.25:80")?;
        Ok(Self { stream })
    }

    pub fn send(&mut self, data: &[&dyn Command]) -> io::Result<Vec<u8>> {
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

    pub fn receive(&mut self) -> Option<u8> {
        let _ = self.stream.set_nonblocking(true);
        let r = (&mut self.stream).bytes().next().and_then(Result::ok);
        let _ = self.stream.set_nonblocking(false);
        r
    }

    pub fn data_available(&self) -> bool {
        let _ = self.stream.set_nonblocking(true);
        let r = self.stream.peek(&mut [0]).is_ok();
        let _ = self.stream.set_nonblocking(false);
        r
    }
}
