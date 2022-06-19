#![allow(dead_code)]
use std::{num::NonZeroU8, collections::HashMap};
use lazy_static::lazy_static;

//* Directions to use with movement commands
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl PartialEq<HorizontalDir> for Direction {
    fn eq(&self, other: &HorizontalDir) -> bool {
        match self {
            Self::Left => *other == HorizontalDir::Left,
            Self::Right => *other == HorizontalDir::Right,
            _ => false,
        }
    }
}

impl From<HorizontalDir> for Direction {
    fn from(direction: HorizontalDir) -> Self {
        match direction {
            HorizontalDir::Left => Self::Left,
            HorizontalDir::Right => Self::Right,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HorizontalDir {
    Left,
    Right,
}

impl PartialEq<Direction> for HorizontalDir {
    fn eq(&self, other: &Direction) -> bool {
        match self {
            Self::Left => *other == Direction::Left,
            Self::Right => *other == Direction::Right,
        }
    }
}

impl TryFrom<Direction> for HorizontalDir {
    type Error = ();
    fn try_from(direction: Direction) -> Result<Self, Self::Error> {
        match direction {
            Direction::Left => Ok(HorizontalDir::Left),
            Direction::Right => Ok(HorizontalDir::Right),
            _ => Err(()),
        }
    }
}

//* Commands themselves
pub trait Command {
    fn encode(&self) -> EncodedCmd;
}
type EncodedCmd = u16;

struct Move {
    distance: u16,
    direction: Direction,
}

impl Move {
    /// Creates a new movement command.
    ///
    /// Distance moved can be at most 2^16 steps.
    fn new(distance: u16, direction: Direction) -> Self {
        if distance >= 2_u16.pow(12) {
            panic!()
        }

        Self {
            distance,
            direction,
        }
    }

    fn left(distance: u16) -> Self {
        Self::new(distance, Direction::Left)
    }

    fn right(distance: u16) -> Self {
        Self::new(distance, Direction::Right)
    }

    fn up(distance: u16) -> Self {
        Self::new(distance, Direction::Up)
    }

    fn down(distance: u16) -> Self {
        Self::new(distance, Direction::Down)
    }
}

impl Command for Move {
    fn encode(&self) -> EncodedCmd {
        0xC000
            | match self.direction {
                Direction::Left => 0b10 << 12,
                Direction::Right => 0b00 << 12,
                Direction::Up => 0b11 << 12,
                Direction::Down => 0b01 << 12,
            }
            | self.distance
    }
}

struct HomePosition {
    carriage: bool,
    color_tape: bool,
    type_wheel: bool,
}

impl Command for HomePosition {
    fn encode(&self) -> EncodedCmd {
        0x8200
            | if self.carriage { 1 << 8 } else { 0 }
            | if self.color_tape { 1 << 9 } else { 0 }
            | if self.type_wheel { 1 << 10 } else { 0 }
    }
}

struct SetCharWidth {
    width: u8,
}

pub struct Write {
    pub letter: u8,
    pub thickness: u8,
    pub movement: Option<HorizontalDir>,
}

impl Write {
    fn new(letter: char, thickness: u8, movement: Option<HorizontalDir>) -> Self {
        Self {
            letter: *LETTERS_MAP.get(&letter).unwrap_or(&1),
            thickness,
            movement,
        }
    }

    fn char(letter: char) -> Self {
        Self::new(letter, 42, Some(HorizontalDir::Right))
    }
}

lazy_static!{
    static ref LETTERS: [char; 100] = [
        '.', ',', '-', 'v', 'l', 'm', 'j', 'w',
        '²', 'µ', 'f', '^', '>', '´', '+', '1',

        '2', '3', '4', '5', '6', '7', '8', '9',
        '0', 'E', '£', 'B', 'F', 'P', 'S', 'Z',

        'V', '&', 'Y', 'A', 'T', 'L', '$', 'R',
        '*', 'C', '\'', 'D', '?', 'N', 'I', 'U',

        ')', 'W', '_', '=', ';', ':', 'M', '\'',
        'H', '(', 'K', '/', 'O', '!', 'X', '§',

        'Q', 'J', '%', '³', 'G', '°', 'Ü', '`',
        'Ö', '<', 'Ä', '#', 't', 'x', 'q', 'ß',

        'ü', 'ö', 'ä', 'y', 'k', 'p', 'h', 'c',
        'g', 'n', 'r', 's', 'e', 'a', 'i', 'd',

        'u', 'b', 'o', 'z',
    ];
    static ref LETTERS_MAP: HashMap<char, u8> = LETTERS.iter().enumerate().map(|(i, c)| (*c, (i + 1) as u8)).collect();
}

impl Command for Write {
    fn encode(&self) -> EncodedCmd {
        (self.letter as u16) << 8
            | (self.thickness as u16) << 2
            | match self.movement {
                None => 0,
                Some(HorizontalDir::Right) => 1,
                Some(HorizontalDir::Left) => 3,
            }
    }
}

struct Space {
    distance: Option<NonZeroU8>,
}

struct Backspace {
    distance: Option<NonZeroU8>,
}

enum Control {
    Clear,
    Start,
    Stx,
    Etx,
    Enq,
}
use Control::*;

impl Command for Control {
    fn encode(&self) -> EncodedCmd {
        match self {
            Clear => 0xA000,
            Start => 0xA100,
            Stx => 0xA200,
            Etx => 0xA300,
            Enq => 0xA400,
        }
    }
}

pub const ONLINE: [&dyn Command; 4] = [&Clear, &Start, &Enq, &Stx];
pub const OFFLINE: [&dyn Command; 2] = [&Etx, &Clear];
