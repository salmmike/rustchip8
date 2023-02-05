use piston::{EventLoop, UpdateEvent, RenderEvent};
use rustchip8::{CPU, Opcode};
use std::env;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston_window::{WindowSettings, PistonWindow, EventSettings, Events};
use opengl_graphics::{OpenGL, GlGraphics};

fn main() {
    let mut cpu: CPU = CPU::new();
    let opengl = OpenGL::V3_2;

    let pixel_size = 10;
    let mut window: PistonWindow = WindowSettings::new(
            "rust-chip8",
            [(cpu.peripherals.width * pixel_size) as f64, (cpu.peripherals.height * pixel_size) as f64]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

        if env::args().len() >= 2 {
            let args = env::args();
            let arg = args.last().unwrap();
            cpu.read_program(arg).expect("Error");
        }

        let mut gl: GlGraphics = GlGraphics::new(OpenGL::V3_2);

        let fps = 700;

        let mut events = Events::new(EventSettings::new()).ups(fps);
        let mut i: u64 = 0;

        while let Some(e) = events.next(&mut window) {
            i += 1;

            if i >= fps/60 {
                i = 0;
                cpu.deincrement_timers();

            }
            if let Some(ref _args) = e.update_args() {
                let opcode: Opcode = cpu.fetch();
                cpu.execute(opcode);
            }

            if let Some(ref args) = e.render_args() {
                cpu.peripherals.draw(args, &mut gl);
            }
        }
}
