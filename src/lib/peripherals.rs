extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::{Rectangle, rectangle, DrawState};
use opengl_graphics::GlGraphics;
use piston::{Key, Button};
use piston_window::{RenderArgs, clear, Context};

pub struct Peripherals {
    pub grid: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
    pub key_states: Vec<bool>,
}

impl Peripherals {

    pub fn new() -> Peripherals {
        let width = 64 as usize;
        let height = 32 as usize;
        let grid = vec![vec![false; height]; width];

        let peripherals = Peripherals{grid: grid, width, height, key_states: vec![false; 16]};
        peripherals
    }

    fn draw_grid(&mut self, gl: &mut GlGraphics, c: &Context) {
        let pixel_width = c.get_view_size()[0] as usize / self.width;
        let pixel_height = c.get_view_size()[1] as usize / self.height;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.grid[x][y] {
                    Rectangle::new([1.0, 1.0, 1.0, 1.0])
                        .draw(rectangle::rectangle_by_corners((x*pixel_width) as f64,
                                                                    (y*pixel_height) as f64,
                                                                    (x*pixel_width + pixel_width) as f64,
                                                                    (y*pixel_height + pixel_height) as f64),
                            &DrawState::default(),
                            c.transform,
                            gl
                        )
                }
            }
        }
    }

    pub fn key_to_num(&self, &args: &Button) -> i8 {
        match args {
            Button::Keyboard(Key::D1) => {
                return 1;
            }
            Button::Keyboard(Key::D2) => {
                return 2;
            }
            Button::Keyboard(Key::D3) => {
                return 3;
            }
            Button::Keyboard(Key::D4) => {
                return 0xC;
            }
            Button::Keyboard(Key::Q) => {
                return 4;
            }
            Button::Keyboard(Key::W) => {
                return 5;
            }
            Button::Keyboard(Key::E) => {
                return 6;
            }
            Button::Keyboard(Key::R) => {
                return 0xD;
            }
            Button::Keyboard(Key::A) => {
                return 7;
            }
            Button::Keyboard(Key::S) => {
                return 8;
            }
            Button::Keyboard(Key::D) => {
                return 9;
            }
            Button::Keyboard(Key::F) => {
                return 0xE;
            }
            Button::Keyboard(Key::Z) => {
                return 0xA;
            }
            Button::Keyboard(Key::X) => {
                return 0;
            }
            Button::Keyboard(Key::C) => {
                return 0xB;
            }
            Button::Keyboard(Key::V) => {
                return 0xF;
            }
            _ => {
                return -1;
            }
        }

    }

    pub fn set_key(&mut self, &args: &Button) {
        let key = self.key_to_num(&args);
        if key >= 0 && key <= 0xF{
            self.key_states[key as usize] = true;
            println!("Press {key}")

        }
    }

    pub fn release_key(&mut self, &args: &Button) {
        let key = self.key_to_num(&args);
        if key >= 0 && key <= 0xF{
            self.key_states[key as usize] = false;
            println!("Release {key}")
        }
    }

    pub fn draw(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let ref c = Context::new_abs(args.window_size[0] as f64, args.window_size[1] as f64);
        gl.draw(args.viewport(), |_, gl,| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
            self.draw_grid(gl, c);
        });
    }

    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.grid[x][y] = false;
            }
        }
    }

    pub fn flip(&mut self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            let val = self.grid[x][y] == true;
            self.grid[x][y] ^= true;
            return val;
        }
        return false;
    }
}
