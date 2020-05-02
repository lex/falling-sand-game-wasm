extern crate web_sys;

mod particle;
mod utils;

use std::fmt;
use std::mem;
use wasm_bindgen::prelude::*;

use particle::Direction;
use particle::Particle;
use particle::ParticleType;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct SandGame {
    width: u32,
    height: u32,
    input_buffer: Vec<particle::Particle>,
    output_buffer: Vec<particle::Particle>,
}

#[wasm_bindgen]
impl SandGame {
    pub fn new() -> SandGame {
        let width = 32;
        let height = 32;

        let mut input_buffer = Vec::new();
        let mut output_buffer = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let p_type = match (x % width, y % height, x % (width - 1), y % (height - 1)) {
                    (16, 1, _, _) => ParticleType::Sand,
                    (0, _, _, _) => ParticleType::Wall,
                    (_, 0, _, _) => ParticleType::Wall,
                    (_, _, 0, _) => ParticleType::Wall,
                    (_, _, _, 0) => ParticleType::Wall,
                    _ => ParticleType::Empty,
                };

                input_buffer.push(Particle { p_type });
                output_buffer.push(Particle {
                    p_type: ParticleType::Empty,
                });
            }
        }

        SandGame {
            width,
            height,
            input_buffer,
            output_buffer,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn spawn(&mut self) {
        let index = self.get_index(16, 1);
        self.input_buffer[index].p_type = ParticleType::Sand;
    }

    pub fn step(&mut self) {
        self.clear_output_buffer();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);

                let particle = self.input_buffer[index];

                match particle.p_type {
                    ParticleType::Wall => self.update_wall(x, y),
                    ParticleType::Sand => self.update_sand(x, y),
                    _ => (),
                };
            }
        }

        self.swap_buffers();
    }

    fn clear_output_buffer(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);

                let p_type = match (
                    x % self.width,
                    y % self.height,
                    x % (self.width - 1),
                    y % (self.height - 1),
                ) {
                    (0, _, _, _) => ParticleType::Wall,
                    (_, 0, _, _) => ParticleType::Wall,
                    (_, _, 0, _) => ParticleType::Wall,
                    (_, _, _, 0) => ParticleType::Wall,
                    _ => ParticleType::Empty,
                };

                self.output_buffer[index].p_type = p_type;
            }
        }
    }

    fn swap_buffers(&mut self) {
        // how do references work?????
        mem::swap(&mut self.input_buffer, &mut self.output_buffer);
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn update_wall(&mut self, x: u32, y: u32) {
        let index_current = self.get_index(x, y);
        self.output_buffer[index_current].p_type = ParticleType::Wall;
    }

    fn update_sand(&mut self, x: u32, y: u32) {
        let index_current = self.get_index(x, y);
        let index_down = self.get_index(x, y + 1);
        let index_down_left = self.get_index(x - 1, y + 1);
        let index_down_right = self.get_index(x + 1, y + 1);

        let particle_down = self.input_buffer[index_down];
        let particle_down_left = self.input_buffer[index_down_left];
        let particle_down_right = self.input_buffer[index_down_right];

        let direction = match (
            particle_down_left.p_type,
            particle_down.p_type,
            particle_down_right.p_type,
        ) {
            (_, ParticleType::Empty, _) => Direction::Down,
            (ParticleType::Empty, _, _) => Direction::DownLeft,
            (_, _, ParticleType::Empty) => Direction::DownRight,
            _ => Direction::None,
        };

        let new_index = match direction {
            Direction::Down => index_down,
            Direction::DownLeft => index_down_left,
            Direction::DownRight => index_down_right,
            Direction::None => index_current,
            _ => index_current,
        };

        self.output_buffer[new_index].p_type = ParticleType::Sand;
    }
}

impl fmt::Display for SandGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.output_buffer.as_slice().chunks(self.width as usize) {
            for &particle in line {
                let symbol = if particle.p_type == ParticleType::Empty {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
