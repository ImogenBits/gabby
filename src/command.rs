#![allow(dead_code)]
use std::num::NonZeroU8;

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
enum HorizontalDir {
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

struct Write {
    letter: u8,
    thickness: u8,
    movement: Option<HorizontalDir>,
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
