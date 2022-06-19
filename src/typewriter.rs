#![allow(dead_code)]

use crate::command::{
    cmd, Command, Direction, HorizontalDir, Move, PrintChar, SetCharWidth, OFFLINE, ONLINE, Space,
};
use std::io::{self, Read, Write};
use std::net::TcpStream;

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
        Ok(Self { stream })
    }

    fn has_next(&mut self) -> bool {
        let _ = self.stream.set_nonblocking(true);
        let r = self.stream.peek(&mut [0]).is_ok();
        let _ = self.stream.set_nonblocking(false);
        r
    }

    pub fn send(&mut self, command: &Box<dyn Command>) -> Vec<u8> {
        let cmd = command.encode();
        let _ = self.stream.write(&cmd.to_be_bytes());
        match self.find(|p| p.is_reply) {
            Some(p) => p.data,
            None => panic!(),
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
        let _ = (&mut self.stream).take(len as u64).read_to_end(&mut reply);
        Some(Packet {
            is_reply,
            data: reply,
        })
    }
}

pub struct Typewriter {
    stream: StreamInterface,
    pos: (u16, u16),
    line_start: u8,
    char_width: u8,
    pub line_height: u16,
    pub print_weight: u8,
    pub feed_direction: Option<HorizontalDir>,
    pub print_strength: u8,
}

impl Typewriter {
    pub fn new() -> io::Result<Typewriter> {
        let stream = StreamInterface::new()?;
        Ok(Self {
            stream,
            pos: (0, 0),
            line_start: 0,
            char_width: 12,
            line_height: 16,
            print_weight: 42,
            feed_direction: Some(HorizontalDir::Right),
            print_strength: 42,
        })
    }

    pub fn on(&mut self) {
        self.send_raw(&ONLINE);
    }

    pub fn off(&mut self) {
        self.send_raw(&OFFLINE);
    }

    pub fn send_raw(&mut self, data: &[Box<dyn Command>]) -> Vec<u8> {
        data.into_iter()
            .flat_map(|cmd| self.stream.send(cmd))
            .collect()
    }

    pub fn set_char_width(&mut self, char_width: u8) {
        self.char_width = char_width;
        let _ = self.stream.send(&cmd(SetCharWidth::new(char_width)));
    }

    pub fn get_char_width(&self) -> u8 {
        self.char_width
    }

    pub fn print_char(&mut self, character: char) {
        let c = PrintChar::new(character, self.print_weight, self.feed_direction);
        self.stream.send(&cmd(c));
        self.pos.0 = (self.pos.0 as i16
            + match self.feed_direction {
                Some(HorizontalDir::Left) => -(self.char_width as i16),
                Some(HorizontalDir::Right) => self.char_width as i16,
                None => 0,
            }) as u16
    }

    pub fn newline(&mut self) {
        let c = Move::new(self.line_height, Direction::Down);
        self.stream.send(&cmd(c));
        self.pos.1 += self.line_height;
    }

    pub fn carriage_return(&mut self) {
        let c = Move::new(self.pos.0, Direction::Left);
        self.stream.send(&cmd(c));
        self.pos.0 = 0;
    }

    pub fn space(&mut self) {
        let c = Space::default();
        self.stream.send(&cmd(c));
        self.pos.0 += self.char_width as u16;
    }

    pub fn print_string(&mut self, content: &str) {
        for c in content.chars() {
            match c {
                '\n' => {
                    self.newline();
                    self.carriage_return();
                }
                ' ' => self.space(),
                _ => self.print_char(c),
            }
        }
    }
}
