extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct Peripherals {
    pub gl: GlGraphics,
    pub window: Window,
}

impl Peripherals {

    pub fn new() -> Peripherals {
        let opengl = OpenGL::V3_1;
        let window: Window = WindowSettings::new("spinning-square", [200, 200])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let peripherals = Peripherals{gl: GlGraphics::new(opengl), window: window};
        peripherals
    }
}