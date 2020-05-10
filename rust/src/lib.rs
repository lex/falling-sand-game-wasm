#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

extern crate web_sys;

mod particle;
mod renderer;
mod utils;

use wasm_bindgen::prelude::*;
use rand::{Rng, SeedableRng};

use particle::Direction;
use particle::Particle;
use particle::ParticleType;
use renderer::Renderer;

#[wasm_bindgen]
pub struct SandGame {
    particles: Vec<Particle>,
    clocks: Vec<u8>,
    width: usize,
    height: usize,
    renderer: Renderer,
    clock: u8,
    rng: rand_pcg::Lcg64Xsh32,
    time: f32,
}

#[wasm_bindgen]
impl SandGame {
    pub fn new(width: u32, height: u32) -> SandGame {
        utils::set_panic_hook();

        let width = width as usize;
        let height = height as usize;

        let mut particles: Vec<Particle> = vec![Particle { p_type: ParticleType::Empty}; width * height];
        let clocks: Vec<u8> = vec![0; width * height];

        let rng = rand_pcg::Pcg32::seed_from_u64(419);

        for y in 0..height {
            for x in 0..width {
                let p_type = match (x % width, y % height, x % (width - 1), y % (height - 1)) {
                    (0, _, _, _) => ParticleType::Wall,
                    (_, 0, _, _) => ParticleType::Wall,
                    (_, _, 0, _) => ParticleType::Wall,
                    (_, _, _, 0) => ParticleType::Wall,
                    _ => ParticleType::Empty,
                };

                let index = y * width + x;
                particles[index].p_type = p_type;
            }
        }

        let renderer = Renderer {
            context: None,
            program_info: None,
            buffers: None,
        };

        log!("Sand game operational.");

        SandGame {
            particles,
            clocks,
            width,
            height,
            renderer,
            clock: 0,
            rng: rng,
            time: 0.0,
        }
    }

    pub fn spawn(&mut self, x: u32, y: u32, p_type: u8) {
        // how????
        let p_type = match p_type {
            0 => ParticleType::Empty,
            1 => ParticleType::Wall,
            2 => ParticleType::Sand,
            3 => ParticleType::Water,
            4 => ParticleType::Plant,
            5 => ParticleType::Fire,
            _ => ParticleType::Empty,
        };

        let index = self.get_index(x as usize, y as usize);
        self.particles[index].p_type = p_type;
    }

    pub fn step(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = self.get_index(x, y);

                let particle = &self.particles[index];

                if self.clocks[index].wrapping_sub(self.clock) == 1 {
                    continue;
                }

                match particle.p_type {
                    ParticleType::Wall => self.update_wall(x, y),
                    ParticleType::Sand => self.update_sand(x, y),
                    ParticleType::Water => self.update_water(x, y),
                    ParticleType::Plant => self.update_plant(x, y),
                    ParticleType::Fire => self.update_fire(x, y),
                    _ => (),
                };
            }
        }

        self.clock = self.clock.wrapping_add(1);
        self.time += 0.16;
    }

    pub fn render(&mut self) {
        let f: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.particles.as_ptr() as *const u8,
                self.particles.len() * std::mem::size_of::<u8>(),
            )
        };

        self.renderer.render(f, self.width as u32, self.height as u32, self.time);
    }

    pub fn initialize_webgl(&mut self) {
        self.renderer.setup_webgl();
    }
}

impl SandGame {
    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width + x) as usize
    }

    fn update_wall(&mut self, x: usize, y: usize) {
        let index_current = self.get_index(x, y);
        self.particles[index_current].p_type = ParticleType::Wall;
        self.clocks[index_current] = self.clock.wrapping_add(1);
    }

    fn update_fire(&mut self, x: usize, y: usize) {
        let index_current = self.get_index(x, y);

        let index_down = self.get_index(x, y + 1);
        let index_left = self.get_index(x - 1, y);
        let index_right = self.get_index(x + 1, y);
        let index_up = self.get_index(x, y - 1);

        for index in [index_down, index_left, index_right, index_up].iter() {
            let particle = &self.particles[*index];
            if particle.p_type == ParticleType::Plant {
                self.particles[*index].p_type = ParticleType::Fire;
                self.clocks[*index] = self.clock.wrapping_add(1);
            }

        }

        if self.clocks[index_current].wrapping_sub(self.clock) > 10 {
            self.particles[index_current].p_type = ParticleType::Empty;
            self.clocks[index_current] = self.clock.wrapping_add(1);
        }
    }

    fn update_plant(&mut self, x: usize, y: usize) {
        let index_current = self.get_index(x, y);

        let index_down = self.get_index(x, y + 1);
        let index_down_left = self.get_index(x - 1, y + 1);
        let index_down_right = self.get_index(x + 1, y + 1);
        let index_left = self.get_index(x - 1, y);
        let index_right = self.get_index(x + 1, y);

        let index_up = self.get_index(x, y - 1);
        let index_up_left = self.get_index(x - 1, y - 1);
        let index_up_right = self.get_index(x + 1, y - 1);

        let particle_down = &self.particles[index_down];
        let particle_down_left = &self.particles[index_down_left];
        let particle_down_right = &self.particles[index_down_right];
        let particle_left = &self.particles[index_left];
        let particle_right = &self.particles[index_right];

        let particle_up = &self.particles[index_up];
        let particle_up_left = &self.particles[index_up_left];
        let particle_up_right = &self.particles[index_up_right];

        let direction = match (
            particle_down_left.p_type,
            particle_down.p_type,
            particle_down_right.p_type,
            particle_left.p_type,
            particle_right.p_type,
            particle_up.p_type,
            particle_up_left.p_type,
            particle_up_right.p_type,
        ) {
            (ParticleType::Water, _, _, _, _, _, _, _) => Direction::DownLeft,
            (_, ParticleType::Water, _, _, _, _, _, _) => Direction::Down,
            (_, _, ParticleType::Water, _, _, _, _, _) => Direction::DownRight,
            (_, _, _, ParticleType::Water, _, _, _, _) => Direction::Left,
            (_, _, _, _, ParticleType::Water, _, _, _) => Direction::Right,
            (_, _, _, _, _, ParticleType::Water, _, _) => Direction::Up,
            (_, _, _, _, _, _, ParticleType::Water, _) => Direction::UpLeft,
            (_, _, _, _, _, _, _, ParticleType::Water) => Direction::UpRight,
            _ => Direction::None,
        };

        if direction == Direction::None {
            return;
        }

        let index_new = match direction {
            Direction::Down => index_down,
            Direction::DownLeft => index_down_left,
            Direction::DownRight => index_down_right,
            Direction::Left => index_left,
            Direction::Right => index_right,
            Direction::Up => index_up,
            Direction::UpLeft => index_up_left,
            Direction::UpRight => index_up_right,
            Direction::None => index_current,
            _ => index_current,
        };

        self.particles[index_current].p_type = ParticleType::Plant;
        self.particles[index_new].p_type = ParticleType::Plant;
        self.clocks[index_current] = self.clock.wrapping_add(1);
        self.clocks[index_new] = self.clock.wrapping_add(1);
    }

    fn update_water(&mut self, x: usize, y: usize) {
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

        let r = self.rng.gen_range(0, 2);

        let direction = match (
            particle_down_left.p_type,
            particle_down.p_type,
            particle_down_right.p_type,
            particle_left.p_type,
            particle_right.p_type,
        ) {
            (_, ParticleType::Empty, _, _, _) => Direction::Down,
            (_, _, _, ParticleType::Empty, ParticleType::Empty) => {
                if r == 0 {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            (_, _, _, ParticleType::Empty, _) => Direction::Left,
            (_, _, _, _, ParticleType::Empty) => Direction::Right,
            (ParticleType::Empty, _, ParticleType::Empty, _, _) => {
                if r == 0 {
                    Direction::DownLeft
                } else {
                    Direction::DownRight
                }
            }
            (ParticleType::Empty, _, _, _, _) => Direction::DownLeft,
            (_, _, ParticleType::Empty, _, _) => Direction::DownRight,
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
        self.clocks[index_current] = self.clock.wrapping_add(1);
        self.clocks[index_new] = self.clock.wrapping_add(1);
    }

    fn update_sand(&mut self, x: usize, y: usize) {
        let index_current = self.get_index(x, y);
        let index_down = self.get_index(x, y + 1);
        let index_down_left = self.get_index(x - 1, y + 1);
        let index_down_right = self.get_index(x + 1, y + 1);

        let particle_down = &self.particles[index_down];
        let particle_down_left = &self.particles[index_down_left];
        let particle_down_right = &self.particles[index_down_right];

        let r = self.rng.gen_range(0, 2);

        let direction = match (
            particle_down_left.p_type,
            particle_down.p_type,
            particle_down_right.p_type,
        ) {
            (_, ParticleType::Empty, _) => Direction::Down,
            (_, ParticleType::Water, _) => Direction::Down,
            (ParticleType::Empty, _, ParticleType::Empty) => {
                if r == 0 {
                    Direction::DownLeft
                } else {
                    Direction::DownRight
                }
            }
            (ParticleType::Water, _, ParticleType::Water) => {
                if r == 0 {
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
        self.clocks[index_current] = self.clock.wrapping_add(1);
        self.clocks[index_new] = self.clock.wrapping_add(1);
    }
}
