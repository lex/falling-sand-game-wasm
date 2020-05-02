use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Particle {
    pub p_type: ParticleType,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticleType {
    Empty = 0,
    Wall = 1,
    Sand = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left = 0,
    Down = 1,
    Right = 2,
    Up = 3,
    DownLeft = 4,
    DownRight = 5,
    UpRight = 6,
    UpLeft = 7,
    None = 8,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Direction::Left => "Direction::Left",
            Direction::Down => "Direction::Down",
            Direction::Right => "Direction::Right",
            Direction::Up => "Direction::Up",
            Direction::DownLeft => "Direction::DownLeft",
            Direction::DownRight => "Direction::DownRight",
            Direction::UpRight => "Direction::UpRight",
            Direction::UpLeft => "Direction::UpLeft",
            Direction::None => "Direction::None",
        };
        write!(f, "{}", printable)
    }
}
