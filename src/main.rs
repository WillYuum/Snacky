extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub mod main_game;
use main_game::GameRequiredArgs;

fn main() {
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    let window_width = COLS * SQUARE_WIDTH;
    let window_height = ROWS * SQUARE_WIDTH;

    let mut window: Window = WindowSettings::new("Snake Game", [window_width, window_height])
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
    };
    let game = &mut main_game::init(&game_required_args);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        //Rendering
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        //Updates
        if let Some(u) = e.update_args() {
            println!("Update!");

            //print amout of fps
            // let fps = 1.0 / u.dt;
            // println!("fps: {}", fps);
        }

        // key inputs
        if let Some(k) = e.press_args() {
            println!("Press!");
        }
    }
}
