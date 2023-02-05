use piston::{EventLoop, UpdateEvent, RenderEvent};
use rustchip8::{CPU, Opcode};

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston_window::{WindowSettings, PistonWindow, EventSettings, Events};
use opengl_graphics::{OpenGL, GlGraphics};

fn main() {
    let mut cpu: CPU = CPU::new();
    let mut gl: GlGraphics = GlGraphics::new(OpenGL::V3_2);

    let pixel_size = 10;
    let mut window: PistonWindow = WindowSettings::new(
            "rust-chip8",
            [(cpu.peripherals.width * pixel_size) as f64, (cpu.peripherals.height * pixel_size) as f64]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

        let mut events = Events::new(EventSettings::new()).ups(700);

        while let Some(e) = events.next(&mut window) {
            if let Some(ref _args) = e.update_args() {
                let opcode: Opcode = cpu.fetch();
                cpu.execute(opcode);
            }

            if let Some(ref args) = e.render_args() {
               cpu.peripherals.draw(args, &mut gl);
            }
        }
}
