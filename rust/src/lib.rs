extern crate web_sys;

mod particle;
mod renderer;
mod utils;

use wasm_bindgen::prelude::*;

use particle::Direction;
use particle::Particle;
use particle::ParticleType;
use renderer::Renderer;

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
    particles: Vec<Particle>,
    framebuffer: Vec<u8>,
    renderer: Renderer,
    clock: u8,
}

#[wasm_bindgen]
impl SandGame {
    pub fn new(width: u32, height: u32) -> SandGame {
        utils::set_panic_hook();
        let mut particles: Vec<Particle> = Vec::new();

        let framebuffer: Vec<u8> = (0..(width * height * 3)).map(|_| 0).collect();

        for y in 0..height {
            for x in 0..width {
                let p_type = match (x % width, y % height, x % (width - 1), y % (height - 1)) {
                    (0, _, _, _) => ParticleType::Wall,
                    (_, 0, _, _) => ParticleType::Wall,
                    (_, _, 0, _) => ParticleType::Wall,
                    (_, _, _, 0) => ParticleType::Wall,
                    _ => ParticleType::Empty,
                };

                particles.push(Particle { p_type, clock: 0 });
            }
        }

        let renderer = Renderer {
            context: None,
            program_info: None,
            buffers: None,
        };

        SandGame {
            width,
            height,
            particles,
            framebuffer,
            renderer,
            clock: 0,
        }
    }

    pub fn spawn(&mut self, x: u32, y: u32, p_type: u8) {
        // how????
        let p_type = match p_type {
            0 => ParticleType::Empty,
            1 => ParticleType::Wall,
            2 => ParticleType::Sand,
            3 => ParticleType::Water,
            _ => ParticleType::Empty,
        };

        let index = self.get_index(x, y);
        self.particles[index].p_type = p_type;
    }

    pub fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);

                let particle = &self.particles[index];

                if particle.clock.wrapping_sub(self.clock) == 1 {
                    continue;
                }

                match particle.p_type {
                    ParticleType::Wall => self.update_wall(x, y),
                    ParticleType::Sand => self.update_sand(x, y),
                    ParticleType::Water => self.update_water(x, y),
                    _ => (),
                };
            }
        }

        self.clock = self.clock.wrapping_add(1);

        self.update_framebuffer();
        let f: &[u8] = &self.framebuffer;
        self.renderer.render(f, self.width, self.height);
    }

    pub fn initialize_webgl(&mut self) {
        self.renderer.setup_webgl();
    }

    fn update_framebuffer(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);
                let particle = &self.particles[index];

                let position = (y * (self.width * 3) + x * 3) as usize;

                let (r, g, b) = match particle.p_type {
                    ParticleType::Empty => (0, 0, 0),
                    ParticleType::Wall => (220, 220, 220),
                    ParticleType::Sand => (194, 178, 128),
                    ParticleType::Water => (128, 197, 222),
                    _ => (255, 128, 128),
                };

                self.framebuffer[position + 0] = r;
                self.framebuffer[position + 1] = g;
                self.framebuffer[position + 2] = b;
            }
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn update_wall(&mut self, x: u32, y: u32) {
        let index_current = self.get_index(x, y);
        self.particles[index_current].p_type = ParticleType::Wall;
        self.particles[index_current].clock = self.clock.wrapping_add(1);
    }

    fn update_water(&mut self, x: u32, y: u32) {
        let index_current = self.get_index(x, y);
        let index_down = self.get_index(x, y + 1);
        let index_down_left = self.get_index(x - 1, y + 1);
        let index_down_right = self.get_index(x + 1, y + 1);
        let index_left = self.get_index(x - 1, y);
        let index_right = self.get_index(x + 1, y);

        let particle_down = &self.particles[index_down];
        let particle_down_left = &self.particles[index_down_left];
        let particle_down_right = &self.particles[index_down_right];
        let particle_left = &self.particles[index_left];
        let particle_right = &self.particles[index_right];

        let direction = match (
            particle_down_left.p_type,
            particle_down.p_type,
            particle_down_right.p_type,
            particle_left.p_type,
            particle_right.p_type,
        ) {
            (_, ParticleType::Empty, _, _, _) => Direction::Down,
            (ParticleType::Empty, _, ParticleType::Empty, _, _) => {
                if self.clock % 2 == 0 {
                    Direction::DownLeft
                } else {
                    Direction::DownRight
                }
            }
            (ParticleType::Empty, _, _, _, _) => Direction::DownLeft,
            (_, _, ParticleType::Empty, _, _) => Direction::DownRight,
            (_, _, _, ParticleType::Empty, ParticleType::Empty) => {
                if self.clock % 2 == 0 {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            (_, _, _, ParticleType::Empty, _) => Direction::Left,
            (_, _, _, _, ParticleType::Empty) => Direction::Right,
            _ => Direction::None,
        };

        let index_new = match direction {
            Direction::Down => index_down,
            Direction::DownLeft => index_down_left,
            Direction::DownRight => index_down_right,
            Direction::Left => index_left,
            Direction::Right => index_right,
            Direction::None => index_current,
            _ => index_current,
        };

        self.particles[index_current].p_type = ParticleType::Empty;
        self.particles[index_new].p_type = ParticleType::Water;
        self.particles[index_current].clock = self.clock.wrapping_add(1);
        self.particles[index_new].clock = self.clock.wrapping_add(1);
    }

    fn update_sand(&mut self, x: u32, y: u32) {
        let index_current = self.get_index(x, y);
        let index_down = self.get_index(x, y + 1);
        let index_down_left = self.get_index(x - 1, y + 1);
        let index_down_right = self.get_index(x + 1, y + 1);

        let particle_down = &self.particles[index_down];
        let particle_down_left = &self.particles[index_down_left];
        let particle_down_right = &self.particles[index_down_right];

        let direction = match (
            particle_down_left.p_type,
            particle_down.p_type,
            particle_down_right.p_type,
        ) {
            (_, ParticleType::Empty, _) => Direction::Down,
            (_, ParticleType::Water, _) => Direction::Down,
            (ParticleType::Empty, _, ParticleType::Empty) => {
                if self.clock % 2 == 0 {
                    Direction::DownLeft
                } else {
                    Direction::DownRight
                }
            }
            (ParticleType::Water, _, ParticleType::Water) => {
                if self.clock % 2 == 0 {
                    Direction::DownLeft
                } else {
                    Direction::DownRight
                }
            }
            (ParticleType::Empty, _, _) => Direction::DownLeft,
            (ParticleType::Water, _, _) => Direction::DownLeft,
            (_, _, ParticleType::Empty) => Direction::DownRight,
            (_, _, ParticleType::Water) => Direction::DownRight,
            _ => Direction::None,
        };

        let index_new = match direction {
            Direction::Down => index_down,
            Direction::DownLeft => index_down_left,
            Direction::DownRight => index_down_right,
            Direction::None => index_current,
            _ => index_current,
        };

        let type_current = self.particles[index_current].p_type;
        let type_new = self.particles[index_new].p_type;

        self.particles[index_current].p_type = type_new;
        self.particles[index_new].p_type = type_current;
        self.particles[index_current].clock = self.clock.wrapping_add(1);
        self.particles[index_new].clock = self.clock.wrapping_add(1);
    }
}
