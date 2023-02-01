extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;

pub struct Peripherals {
    pub gl: GlGraphics,
    pub window: Window,
    pub grid: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,
}

impl Peripherals {

    pub fn new() -> Peripherals {
        let opengl = OpenGL::V3_1;
        let width = 64 as usize;
        let height = 32 as usize;
        let pixel_size = 10;

        let window: Window = WindowSettings::new(
            "rust-chip8",
            [(width * pixel_size) as f64, (height * pixel_size) as f64]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

        let grid = vec![vec![false; height]; width];

        let peripherals = Peripherals{gl: GlGraphics::new(opengl), window: window, grid: grid,
                                                   width: width, height: height, pixel_size: pixel_size};
        peripherals
    }

    pub fn draw(&mut self) {

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
