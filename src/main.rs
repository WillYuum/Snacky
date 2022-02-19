extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

// use std::iter::FromIterator;

pub mod main_game;
use main_game::GameRequiredArgs;

static mut dt: f64 = 0.1;

fn main() {
    // Change this to OpenGL::V2_1 if this fails.
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    const WIDTH: u32 = COLS * SQUARE_WIDTH;
    const HEIGHT: u32 = ROWS * SQUARE_WIDTH;

    let mut window: Window = WindowSettings::new("Snake Game", [WIDTH, HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| {
            panic!("Failed to build window: {}", e);
        });

    let game_required_args = GameRequiredArgs {
        cols: COLS,
        rows: ROWS,
        square_width: SQUARE_WIDTH,
        opengl: opengl,
    };
    let game = &mut main_game::init(&game_required_args);
    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
    println!("Congratulations, your score was: {}", game.score);
}
