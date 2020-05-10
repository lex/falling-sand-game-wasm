#[repr(C)]
#[derive(Clone)]
pub struct Particle {
    pub p_type: ParticleType,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub enum ParticleType {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Plant = 4,
    Fire = 5,
    Oil = 6,
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
