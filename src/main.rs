#![allow(dead_code)]
#![feature(io_error_other)]
mod command;

use std::net::TcpStream;
use std::io::{self, Write, Read};
use hex::encode_upper;
use command::{Command, ONLINE, OFFLINE};


struct Typewriter {
    stream: TcpStream,
}

impl Typewriter {
    fn new() -> io::Result<Typewriter> {
        let stream = TcpStream::connect("192.168.178.25:80")?;
        Ok(Self {stream})
    }

    fn send(&mut self, data: &[&dyn Command]) -> io::Result<()>{
        assert!(data.len() % 2 == 0);
        for cmd in data {
            let cmd = cmd.encode();
            self.stream.write(&cmd.to_be_bytes())?;
            if (cmd & 0xF000) == 0xA000 {
                let mut buf = [0];
                self.stream.read_exact(&mut buf)?;
                let reply = buf[0];
                if reply == 0xA4 {
                    let mut reply_data = [0; 128];
                    let mut i = 0;
                    self.stream.read_exact(&mut buf)?;
                    while buf[0] != 0 {
                        reply_data[i] = buf[0];
                        i += 1;
                        self.stream.read_exact(&mut buf)?;
                    }
                    let reply_str = encode_upper(&reply_data[..i]);
                    println!("{reply_str}");
                }
            }
            
        }
        

        Ok(())
    }
}


fn main() -> io::Result<()> {
    let mut gabby = Typewriter::new()?;
    gabby.send(&ONLINE)?;
    gabby.send(&OFFLINE)?;

    Ok(())
}
