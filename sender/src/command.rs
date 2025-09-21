#![allow(dead_code, unused_macros)]
use lazy_static::lazy_static;
use std::collections::HashMap;

//* Directions to use with movement commands
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
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
pub trait Command: Sync {
    fn encode(&self) -> EncodedCmd;
}
type EncodedCmd = u16;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    distance: u16,
    direction: Direction,
}

impl Move {
    /// Creates a new movement command.
    ///
    /// Distance moved can be at most 2^16 steps.
    pub fn new(distance: u16, direction: Direction) -> Self {
        assert!(distance < 2_u16.pow(12));
        Self {
            distance,
            direction,
        }
    }

    pub fn left(distance: u16) -> Self {
        Self::new(distance, Direction::Left)
    }

    pub fn right(distance: u16) -> Self {
        Self::new(distance, Direction::Right)
    }

    pub fn up(distance: u16) -> Self {
        Self::new(distance, Direction::Up)
    }

    pub fn down(distance: u16) -> Self {
        Self::new(distance, Direction::Down)
    }

    pub fn newline() -> Self {
        Self::down(16)
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

#[derive(Debug, Clone, Copy)]
pub struct HomePosition {
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

#[derive(Debug, Clone, Copy)]
pub struct SetCharWidth {
    width: u8,
}

impl SetCharWidth {
    pub fn new(width: u8) -> Self {
        Self { width }
    }
}

impl Command for SetCharWidth {
    fn encode(&self) -> EncodedCmd {
        0x8000 | (self.width as u16)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PrintChar {
    pub letter: u8,
    pub thickness: u8,
    pub movement: Option<HorizontalDir>,
}

impl PrintChar {
    pub fn new(letter: char, thickness: u8, movement: Option<HorizontalDir>) -> Self {
        Self {
            letter: *LETTERS_MAP.get(&letter).unwrap_or(&1),
            thickness,
            movement,
        }
    }

    pub fn char(letter: char) -> Self {
        Self::new(letter, 42, Some(HorizontalDir::Right))
    }

    pub fn string(letters: &str) -> Vec<Box<dyn Command>> {
        letters
            .chars()
            .map(|c| Box::new(Self::char(c)) as Box<dyn Command>)
            .collect()
    }
}

lazy_static! {
    static ref LETTERS: [char; 100] = [
        '.', ',', '-', 'v', 'l', 'm', 'j', 'w', '²', 'µ', 'f', '^', '>', '´', '+', '1', '2', '3',
        '4', '5', '6', '7', '8', '9', '0', 'E', '£', 'B', 'F', 'P', 'S', 'Z', 'V', '&', 'Y', 'A',
        'T', 'L', '$', 'R', '*', 'C', '\'', 'D', '?', 'N', 'I', 'U', ')', 'W', '_', '=', ';', ':',
        'M', '\'', 'H', '(', 'K', '/', 'O', '!', 'X', '§', 'Q', 'J', '%', '³', 'G', '°', 'Ü', '`',
        'Ö', '<', 'Ä', '#', 't', 'x', 'q', 'ß', 'ü', 'ö', 'ä', 'y', 'k', 'p', 'h', 'c', 'g', 'n',
        'r', 's', 'e', 'a', 'i', 'd', 'u', 'b', 'o', 'z',
    ];
    static ref LETTERS_MAP: HashMap<char, u8> = LETTERS
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, (i + 1) as u8))
        .collect();
}

impl Command for PrintChar {
    fn encode(&self) -> EncodedCmd {
        (self.letter as u16) << 8
            | match self.movement {
                None => 0b00,
                Some(HorizontalDir::Right) => 0b10,
                Some(HorizontalDir::Left) => 0b11,
            } << 6
            | (self.thickness as u16)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Space {
    distance: u8,
}

impl Space {
    pub fn new(distance: u8) -> Self {
        Self { distance }
    }
}

impl Default for Space {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Command for Space {
    fn encode(&self) -> EncodedCmd {
        0x8300 | (self.distance as u16)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Backspace {
    distance: u8,
}

impl Backspace {
    pub fn new(distance: u8) -> Self {
        Self { distance }
    }
}

impl Default for Backspace {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Command for Backspace {
    fn encode(&self) -> EncodedCmd {
        0x8004 | (self.distance as u16)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Control {
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

lazy_static! {
    pub static ref ONLINE: Vec<Box<dyn Command>> = [Clear, Start, Enq, Stx]
        .into_iter()
        .map(|x| Box::new(x) as Box<dyn Command>)
        .collect();
    pub static ref OFFLINE: Vec<Box<dyn Command>> = [Etx, Clear]
        .into_iter()
        .map(|x| Box::new(x) as Box<dyn Command>)
        .collect();
}

pub fn command_sequence<'a>(commands: &[impl Command + Clone + 'a]) -> Vec<Box<dyn Command + 'a>> {
    commands.iter().cloned().map(|x| Box::new(x) as _).collect()
}

#[macro_export]
macro_rules! cmds {
    (eval $e:expr) => {::std::vec::Vec::<::std::boxed::Box<dyn $crate::Command>>::new()};
    ( $($x:expr),+ $(,)? ) => { ::std::vec![ $( ::std::boxed::Box::new($x) as ::std::boxed::Box<dyn $crate::command::Command> ),+ ] };
}

pub fn cmd<'a>(cmd: impl Command + 'a) -> Box<dyn Command + 'a> {
    Box::new(cmd)
}
