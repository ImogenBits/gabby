#![allow(dead_code)]

use std::io::{self, Read, Write};
use std::net::TcpStream;
use crate::command::Command;

struct StreamInterface {
    stream: TcpStream,
}

struct Packet {
    is_reply: bool,
    data: Vec<u8>,
}

impl StreamInterface {
    fn new() -> io::Result<Self> {
        let stream = TcpStream::connect("192.168.178.25:80")?;
        Ok(Self {stream})
    }

    fn has_next(&mut self) -> bool {
        let _ = self.stream.set_nonblocking(true);
        let r = self.stream.peek(&mut [0]).is_ok();
        let _ = self.stream.set_nonblocking(false);
        r
    }

    pub fn send(&mut self, command: &Box<dyn Command>) -> io::Result<Vec<u8>> {
        let cmd = command.encode();
        self.stream.write(&cmd.to_be_bytes())?;
        match self.find(|p| p.is_reply) {
            Some(p) => Ok(p.data),
            None => Err(io::Error::other("")),
        }

    }
}

impl Iterator for StreamInterface {
    type Item = Packet;

    fn next(&mut self) -> Option<Packet> {
        let mut buf = [0];
        if self.stream.read_exact(&mut buf).is_err() {
            return None;
        }
        let len = buf[0] & 0x7F;
        let is_reply = (buf[0] & 0x80) != 0;
        let mut reply = Vec::with_capacity(len as usize);
        let _ = (&mut self.stream)
            .take(len as u64)
            .read_to_end(&mut reply);
        Some(Packet {is_reply, data: reply})
    }

}


pub struct Typewriter {
    stream: StreamInterface,
}

impl Typewriter {
    pub fn new() -> io::Result<Typewriter> {
        let stream = StreamInterface::new()?;
        Ok(Self { stream })
    }

    pub fn send(&mut self, data: &[Box<dyn Command>]) -> io::Result<Vec<u8>> {
        let mut ret = vec![];
        for cmd in data {
            let reply = self.stream.send(cmd)?;
            ret.extend(reply);
        }
        Ok(ret)
    }

    
}
