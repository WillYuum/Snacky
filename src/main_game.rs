use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Button, Key, RenderArgs, UpdateArgs};

use std::collections::linked_list::LinkedList;

pub struct GameRequiredArgs {
    pub cols: u32,
    pub rows: u32,
    pub square_width: u32,
    pub opengl: OpenGL,
}

pub fn init(args: &GameRequiredArgs) -> Game {
    Game {
        score: 0,
        just_eaten: false,
        rows: args.rows,
        cols: args.cols,
        food: Food { x: 1, y: 1 },
        square_width: args.square_width,
        gl: GlGraphics::new(args.opengl),
        snake: Snake {
            gl: GlGraphics::new(args.opengl),
            move_dir: Direction::DOWN,
            width: args.square_width,
            snake_parts: LinkedList::from_iter(
                (vec![SnakePiece(args.cols / 2, args.rows / 2)]).into_iter(),
            ),
        },
    }
}

pub struct Game {
    pub rows: u32,
    pub cols: u32,
    pub gl: GlGraphics,
    pub snake: Snake,
    pub square_width: u32,
    pub score: u32,
    pub just_eaten: bool,
    pub food: Food,
}

impl Game {
    pub fn render(&mut self, args: &RenderArgs) {
        let green_color: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        //The |_c, gl| are arguements that are used for the closure function
        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            graphics::clear(green_color, gl);
        });
        self.snake.render(&mut self.gl, args);
        self.food.render(&mut self.gl, args, self.square_width);
    }

    pub fn update(&mut self, args: &UpdateArgs) -> bool {
        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            return false;
        }

        if self.just_eaten {
            self.score += 1;
            self.just_eaten = false;
        }

        self.just_eaten = self.food.update(&self.snake);
        if self.just_eaten {
            use rand::thread_rng;
            use rand::Rng;
            // try my luck
            let mut r = thread_rng();
            loop {
                let new_x = r.gen_range(0, self.cols);
                let new_y = r.gen_range(0, self.rows);
                if !self.snake.is_collide(new_x, new_y) {
                    self.food = Food { x: new_x, y: new_y };
                    break;
                }
            }
        }

        true
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.move_dir.clone();
        self.snake.move_dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction,
        };
    }
}

#[derive(Clone)]
pub struct SnakePiece(u32, u32);

#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Snake {
    gl: GlGraphics,
    snake_parts: std::collections::LinkedList<SnakePiece>,
    width: u32,
    move_dir: Direction,
}

impl Snake {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .snake_parts
            .iter()
            .map(|p| SnakePiece(p.0 * self.width, p.1 * self.width))
            .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
            .collect();

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        })
    }

    pub fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
        let mut new_front: SnakePiece =
            (*self.snake_parts.front().expect("No front of snake found.")).clone();

        if (self.move_dir == Direction::UP && new_front.1 == 0)
            || (self.move_dir == Direction::LEFT && new_front.0 == 0)
            || (self.move_dir == Direction::DOWN && new_front.1 == rows - 1)
            || (self.move_dir == Direction::RIGHT && new_front.0 == cols - 1)
        {
            return false;
        }

        match self.move_dir {
            Direction::UP => new_front.1 -= 1,
            Direction::DOWN => new_front.1 += 1,
            Direction::LEFT => new_front.0 -= 1,
            Direction::RIGHT => new_front.0 += 1,
        }

        if !just_eaten {
            self.remove_last_square();
        }

        // Checks self collision.
        if self.is_collide(new_front.0, new_front.1) {
            return false;
        }

        self.snake_parts.push_front(new_front);
        true
    }

    fn remove_last_square(&mut self) {
        self.snake_parts.pop_back();
    }

    fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|p| x == p.0 && y == p.1)
    }
}

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {
    // Return true if snake ate food this update
    fn update(&mut self, s: &Snake) -> bool {
        let front = s.snake_parts.front().unwrap();
        if front.0 == self.x && front.1 == self.y {
            true
        } else {
            false
        }
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl)
        });
    }
}
