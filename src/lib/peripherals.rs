extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use graphics::{Rectangle, rectangle, DrawState};
use opengl_graphics::GlGraphics;
use piston_window::{RenderArgs, clear, Context};

pub struct Peripherals {
    pub grid: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl Peripherals {

    pub fn new() -> Peripherals {
        let width = 64 as usize;
        let height = 32 as usize;
        let grid = vec![vec![false; height]; width];

        let peripherals = Peripherals{grid: grid, width, height};
        peripherals
    }

    fn draw_grid(&mut self, gl: &mut GlGraphics, c: &Context) {
        let pixel_width = c.get_view_size()[0] as usize / self.width;
        let pixel_height = c.get_view_size()[1] as usize / self.height;
        for x in 0..self.width {
            for y in 0..self.height {
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

    pub fn draw(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let ref c = Context::new_abs(args.window_size[0] as f64, args.window_size[1] as f64);
        gl.draw(args.viewport(), |_, gl,| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
            self.draw_grid(gl, c);
        });
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
