
extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;

use piston_window::PistonWindow;
use piston::window::WindowSettings;
use piston::event::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::input::mouse::MouseButton;
use piston::input::Button;

mod game;
mod aim;

const WINDOW: [u32; 2] = [800, 800];
const RED: [f32; 4]    = [1.0, 0.0, 0.0, 1.0];
const AIM_NUM: u8      = 5;

fn main() {

    let opengl = OpenGL::V3_2;
    let window: PistonWindow = 
        WindowSettings::new("AimGame", WINDOW)
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    let mut gl = GlGraphics::new(opengl);
    let mut settings = game::Settings::new(&WINDOW);
    settings.set_min_diameter(80.0f64)
            .set_max_diameter(110.0f64);
    let mut game = game::Game::new(&mut settings);

    for _ in (0..AIM_NUM) {
        let loc = game.get_rand_loc();
        game.add_aim(RED, loc);
    }

    let mut mouse_pos: [f64; 2] = [0.0, 0.0];
    for e in window.events() {
        if let Some(args) = e.render_args() {
            game.draw(&args, &mut gl);
        }

        if let Some(args) = e.update_args() {
            game.update(args.dt);
        }

        if let Some(ref args) = e.mouse_cursor_args() {
            mouse_pos = *args;
        }

        if let Some(args) = e.press_args() {
            match args {
                Button::Mouse(MouseButton::Left) => 
                    game.mouse_press(&mouse_pos),
                _ => {}
            }
        }

    }
}
