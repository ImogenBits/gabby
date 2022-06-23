#![allow(dead_code)]

use image::GrayImage;

use crate::command::{
    cmd, Command, Direction, HorizontalDir, Move, PrintChar, SetCharWidth, Space, OFFLINE, ONLINE,
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
    pos: (i16, i16),
    line_start: u8,
    char_width: u8,
    pub print_weight: u8,
    pub line_height: u16,
    pub feed_direction: Option<HorizontalDir>,
}

impl Typewriter {
    pub fn new() -> io::Result<Typewriter> {
        let stream = StreamInterface::new()?;
        let mut s = Self {
            stream,
            pos: (0, 0),
            line_start: 0,
            char_width: 12,
            line_height: 16,
            print_weight: 20,
            feed_direction: Some(HorizontalDir::Right),
        };
        s.on();
        Ok(s)
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
        self.pos.0 += match self.feed_direction {
            Some(HorizontalDir::Left) => -(self.char_width as i16),
            Some(HorizontalDir::Right) => self.char_width as i16,
            None => 0,
        }
    }

    pub fn newline(&mut self) {
        let c = Move::new(self.line_height, Direction::Down);
        self.stream.send(&cmd(c));
        self.pos.1 += self.line_height as i16;
    }

    pub fn carriage_return(&mut self) {
        let c = Move::new(self.pos.0 as u16, Direction::Left);
        self.stream.send(&cmd(c));
        self.pos.0 = 0;
    }

    pub fn space(&mut self) {
        let c = Space::default();
        self.stream.send(&cmd(c));
        self.pos.0 += self.char_width as i16;
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

    pub fn move_head(&mut self, x: i16, y: i16) {
        if x != 0 {
            let c = Move::new(
                x.abs() as u16,
                if x > 0 {
                    Direction::Right
                } else {
                    Direction::Left
                },
            );
            self.stream.send(&cmd(c));
            self.pos.0 += x;
        }
        if y != 0 {
            let c = Move::new(
                y.abs() as u16,
                if y > 0 {
                    Direction::Down
                } else {
                    Direction::Up
                },
            );
            self.stream.send(&cmd(c));
            self.pos.1 += y;
        }
    }

    pub fn move_to(&mut self, x: i16, y: i16) {
        self.move_head(x - self.pos.0, y - self.pos.1)
    }

    pub fn print_image(&mut self, image: &GrayImage) {
        if image.height() > 300 {
            return;
        }
        let start_pos = self.pos;
        let old_feed_dir = self.feed_direction;
        self.feed_direction = None;
        let olf_weight = self.print_weight;
        self.print_weight = 15;
        image
            .enumerate_pixels()
            .filter(|(_, _, p)| p.0[0] < 128)
            .map(|(x, y, _)| (start_pos.0 + 3 * x as i16, start_pos.1 + 2 * y as i16))
            .for_each(|(x, y)| {
                self.move_to(x, y);
                self.print_char('.')
            });
        self.feed_direction = old_feed_dir;
        self.print_weight = olf_weight;
    }
}

impl Drop for Typewriter {
    fn drop(&mut self) {
        self.off();
    }
}
