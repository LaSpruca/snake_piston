pub use opengl_graphics::GlGraphics;
use crate::util::*;
use piston_window::Glyphs;
use piston::{RenderArgs, UpdateArgs};
use graphics::*;
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    grid: Vec<Vec<Cell>>,
    delta_total: f64,
    state: GameState,
    direction: Direction,
    tail: Vec<(i32, i32)>,
    speed: f64,
    score: u32,
    glyphs: Glyphs,
    skip: bool
}

impl App {
    // Make in a new instance of game with default values
    pub fn new(gl: GlGraphics, glyphs: Glyphs) -> Self {
        let mut app = App {
            gl,
            glyphs,
            grid: (0..15).map(|_| (0..15).map(|_| Cell::Empty).collect()).collect(),
            delta_total: 0.0,
            state: GameState::Playing,
            direction: Direction::None,
            tail: vec!(),
            speed: 0.200,
            score: 0,
            skip: false
        };

        app.grid[0][0] = Cell::Head;

        app.new_fruit();

        app
    }

    // Function used to render one frame of the game
    pub fn render(&mut self, args: &RenderArgs) {
        let square = rectangle::square(0.0, 0.0, 25.0);
        let grid = self.grid.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(Color::BLACK.rgb(), gl);

            let transform = c
                .transform
                .trans(25.0, 25.0);


            for (x, row) in grid.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    let transformed = transform.trans(25.0 * x as f64, 25.0 * y as f64);
                    match cell {
                        Cell::Empty => rectangle(Color::BLUE.rgb(), square, transformed, gl),
                        Cell::Food => rectangle(Color::RED.rgb(), square, transformed, gl),
                        Cell::Head => rectangle(Color::GREEN.rgb(), square, transformed, gl),
                        Cell::Tail => rectangle(Color::YELLOW.rgb(), square, transformed, gl),
                    }
                }
            }
        });
    }

    // Function called once per update cycle thing
    pub fn update(&mut self, args: &UpdateArgs) {
        self.delta_total += args.dt;

        let mut ye = false;

        while self.delta_total > self.speed {
            self.update_internal();
            self.delta_total -= self.speed;
            ye = true;
        }

        if ye {
            self.delta_total = 0.0;
        }
    }

    // Actual update logic
    fn update_internal(&mut self) {
        if !self.skip {
            match self.state {
                GameState::Playing => {
                    let mut head = (-1, -1);
                    let mut food = (0, 0);

                    for (x, row) in self.grid.clone().iter().enumerate() {
                        for (y, cell) in row.iter().enumerate() {
                            match cell {
                                Cell::Head => head = (x as i32, y as i32),
                                Cell::Food => food = (x as i32, y as i32),
                                _ => {}
                            }
                            self.grid[x][y] = Cell::Empty;
                        }
                    }

                    self.tail.push(head);

                    match self.direction {
                        Direction::Up => {
                            head.1 -= 1;
                        }
                        Direction::Down => {
                            head.1 += 1;
                        }
                        Direction::Left => {
                            head.0 -= 1;
                        }
                        Direction::Right => {
                            head.0 += 1;
                        }
                        _ => {}
                    }

                    if food == head {
                        self.new_fruit();
                        self.speed *= 0.99;
                        self.score += 1;
                        println!("Score: {}", self.score);
                    } else {
                        self.tail.remove(0);
                    }

                    for (x, y) in self.tail.clone() {
                        self.grid[x as usize][y as usize] = Cell::Tail;
                    }

                    self.grid[food.0 as usize][food.1 as usize] = Cell::Food;

                    if !(head.1 >= 0 && head.0 >= 0) || self.tail.contains(&head) {
                        self.state = GameState::Over
                    } else {
                        if head.1 < 15 && head.0 < 15 {
                            self.grid[head.0 as usize][(head.1) as usize] = Cell::Head;
                        } else {
                            self.state = GameState::Over;
                        }
                    }
                }
                GameState::Over => {}
            }
        } else {
            self.skip = false;
        }
    }

    // Used tp set the direction
    pub fn set_direction(&mut self, direction: Direction) {
        use Direction::*;
        if self.direction != direction && self.state != GameState::Over
            && !(self.direction == Up && direction == Down)
            && !(self.direction == Down && direction == Up)
            && !(self.direction == Left && direction == Right)
            && !(self.direction == Right && direction == Left) {
            self.direction = direction;
            self.update_internal();
            self.skip = true;
        }
    }

    // Reset the game
    pub fn reset(&mut self) {
        self.grid = (0..15).map(|_| (0..15).map(|_| Cell::Empty).collect()).collect();
        self.grid[0][0] = Cell::Head;
        self.direction = Direction::None;
        self.state = GameState::Playing;
        self.tail = vec!();
        self.speed = 0.2;
        self.score = 0;
        self.new_fruit();
    }

    // Used to generate a new thread
    fn new_fruit(&mut self) {
        let mut rng: ThreadRng = rand::thread_rng();
        let pos1: usize = rng.gen_range(1usize, 15usize);
        let pos2: usize = rng.gen_range(1usize, 15usize);
        self.grid[pos1][pos2] = Cell::Food;
    }
}
