mod util;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::util::{Color, Cell, GameState, Direction};
use piston::{PressEvent, Button, Key};
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    grid: Vec<Vec<Cell>>,
    delta_total: f64,
    state: GameState,
    direction: Direction,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 25.0);
        let grid = &self.grid;

        let state = self.state;

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

    fn update(&mut self, args: &UpdateArgs) {
        use piston::input::keyboard::*;
        match self.state {
            GameState::Playing => {
                self.delta_total += args.dt;
                if self.delta_total > 0.200 {
                    println!("Update");
                    self.delta_total = 0.0;

                    let mut tail = vec!();
                    let mut head = (-1, -1);
                    let mut food = (0, 0);

                    for (x, row) in self.grid.clone().iter().enumerate() {
                        for (y, cell) in row.iter().enumerate() {
                            match cell {
                                Cell::Head => head = (x as i32, y as i32),
                                Cell::Tail => tail.push((x as i32, y as i32)),
                                Cell::Food => food = (x as i32, y as i32),
                                _ => {}
                            }
                            self.grid[x][y] = Cell::Empty;
                        }
                    }

                    tail.push(head);

                    if food == head {
                        self.new_fruit();
                    } else {
                        tail.remove(0);
                    }

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

                    for (x, y) in tail.clone() {
                        self.grid[x as usize][y as usize] = Cell::Tail;
                    }

                    self.grid[food.0 as usize][food.1 as usize] = Cell::Food;

                    println!("Head {:?}, Tail {:#?}", head, tail);

                    if head.1 >= 0 && head.0 >= 0 {
                        if head.1 < 15 && head.0 < 15 {
                            self.grid[head.0 as usize][(head.1) as usize] = Cell::Head;
                        } else {
                            self.state = GameState::Over;
                        }
                    } else {
                        println!("Game Over!");
                        self.state = GameState::Over;
                    }
                }
            }
            GameState::Over => {}
        }
    }

    fn new(gl: GlGraphics) -> Self {
        let mut app = App {
            gl,
            grid: (0..15).map(|_| (0..15).map(|_| Cell::Empty).collect()).collect(),
            delta_total: 0.0,
            state: GameState::Playing,
            direction: Direction::None,
        };

        app.grid[0][0] = Cell::Head;

        app.new_fruit();

        app
    }

    fn set_direction(&mut self, direction: Direction) {
        if self.direction != direction {
            self.direction = direction;
        }
    }

    fn reset(&mut self) {
        self.grid = (0..15).map(|_| (0..15).map(|_| Cell::Empty).collect()).collect();
        self.grid[0][0] = Cell::Head;
        self.direction = Direction::None;
        self.state = GameState::Playing;
        self.new_fruit();
    }

    fn new_fruit(&mut self) {
        let mut rng: ThreadRng = rand::thread_rng();
        let pos1: usize = rng.gen_range(1usize, 15usize);
        let pos2: usize = rng.gen_range(1usize, 15usize);
        self.grid[pos1][pos2] = Cell::Food;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [425, 425])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(Key::W) | Button::Keyboard(Key::Up) => {
                    app.set_direction(Direction::Up)
                }
                Button::Keyboard(Key::A) | Button::Keyboard(Key::Left) => {
                    app.set_direction(Direction::Left)
                }
                Button::Keyboard(Key::S) | Button::Keyboard(Key::Down) => {
                    app.set_direction(Direction::Down)
                }
                Button::Keyboard(Key::D) | Button::Keyboard(Key::Right) => {
                    app.set_direction(Direction::Right)
                }
                Button::Keyboard(Key::R) => {
                    app.reset()
                }
                _ => {}
            }
        }
    }
}
