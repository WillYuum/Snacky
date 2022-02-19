use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;

use std::collections::linked_list::LinkedList;

pub struct GameRequiredArgs {
    pub cols: u32,
    pub rows: u32,
    pub square_width: u32,
}

pub fn init(args: &GameRequiredArgs) -> Game {
    Game {
        gl: GlGraphics::new(OpenGL::V3_2),
        snake: Snake {
            pos_x: 0,
            pos_y: 0,
            snake_parts: LinkedList::from_iter(
                (vec![SnakePiece(args.cols / 2, args.rows / 2)]).into_iter(),
            ),
            width: args.square_width,
        },
    }
}

pub struct Game {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub snake: Snake,
}

impl Game {
    pub fn render(&mut self, arg: &RenderArgs) {
        let green_color: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        //The |_c, gl| are arguements that are used for the closure function
        self.gl.draw(arg.viewport(), |_c, gl| {
            // Clear the screen.
            graphics::clear(green_color, gl);
        });
        self.snake.render(&mut self.gl, arg)
    }
}

#[derive(Clone)]
pub struct SnakePiece(u32, u32);

pub struct Snake {
    pub pos_x: i32,
    pub pos_y: i32,
    snake_parts: std::collections::LinkedList<SnakePiece>,
    width: u32,
}

impl Snake {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red_color: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .snake_parts
            .iter()
            .map(|p| SnakePiece(p.0 * self.width, p.1 * self.width))
            .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(red_color, square, transform, gl));
        });
    }
}
