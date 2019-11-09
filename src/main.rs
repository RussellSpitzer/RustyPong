extern crate piston_window;

use piston_window::*;
use std::fmt;
use std::time::{Duration, Instant};


const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WINDOW_WIDTH: f64 = 640.0;
const WINDOW_HEIGHT: f64 = 480.0;
const BALL_RADIUS: f64 = 20.0;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Rust Pong!", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true).build().unwrap();

    let mut events = Events::new(EventSettings::new());

    let mut game = init_game();
    let mut last_frame = Instant::now();
    println!("Init {}", game);
    while let Some(e) = events.next(&mut window) {
        //Draw current game state
        window.draw_2d(&e, |c, g, device| {
            clear(BLACK, g);
            rectangle(WHITE,
                      game.player_1_paddle.dim,
                      c.transform,
                      g);
            rectangle(WHITE,
                      game.player_2_paddle.dim,
                      c.transform,
                      g);
            ellipse(WHITE,
                    [game.ball.x, game.ball.y, 20.0, 20.0],
                    c.transform,
                    g);
        });
        //Game logic
        e.update(|args| {
            let dt = args.dt * 1000.0;
            //println!("Update dt = {}, {}",dt,  game);
            // Move Ball based on Velocity
            game.ball.x += game.ball.x_vel * dt;
            game.ball.y += game.ball.y_vel * dt;
            // Check Contact with Top and Bottom
            if game.ball.y < BALL_RADIUS && game.ball.y_vel < 0.0 ||
                game.ball.y > (WINDOW_HEIGHT - BALL_RADIUS) && game.ball.y_vel > 0.0 {
                game.ball.y_vel *= -1.0;
            }
            // Out of bounds left and right
            if (game.ball.x > WINDOW_WIDTH || game.ball.x < 0.0) {
                game.ball.x = WINDOW_WIDTH / 2.0
            }
        });
        e.mouse_cursor(|pos| {
            game.player_1_paddle.dim[1] = pos[1];
            if (game.player_1_paddle.dim[1] > WINDOW_HEIGHT - 120.0) {
                game.player_1_paddle.dim[1] = WINDOW_HEIGHT - 120.0;
            }
            if (game.player_1_paddle.dim[1] < 0.0) {
                game.player_1_paddle.dim[1] = 0.0;
            }
        });
    };
}

fn init_game() -> GameState {
    GameState {
        player_1_paddle: Paddle::new([20.0, 10.0, 20.0, 120.0]),
        player_2_paddle: Paddle::new([600.0, 10.0, 20.0, 120.0]),
        ball: Ball::new([320.0, 240.0], [0.08, 0.08]),
    }
}

struct Paddle {
    dim: [f64; 4]
}

impl Paddle {
    fn new(dim: [f64; 4]) -> Paddle {
        Paddle {
            dim: dim,
        }
    }
}

struct Ball {
    x: f64,
    y: f64,
    x_vel: f64,
    y_vel: f64,
}

impl Ball {
    fn new(pos: [f64; 2], vel: [f64; 2]) -> Ball {
        Ball {
            x: pos[0],
            y: pos[1],
            x_vel: vel[0],
            y_vel: vel[1],
        }
    }
}

struct GameState {
    player_1_paddle: Paddle,
    player_2_paddle: Paddle,
    ball: Ball,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Player 1 {}) (Player 2 {}), (Ball {})",
               self.player_1_paddle, self.player_2_paddle, self.ball)
    }
}

impl fmt::Display for Paddle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pos ({}, {}) ", self.dim[0], self.dim[1])
    }
}

impl fmt::Display for Ball {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pos ({}, {}) Vel ({}, {})", self.x, self.y, self.x_vel, self.y_vel)
    }
}

