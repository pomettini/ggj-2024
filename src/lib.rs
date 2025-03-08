#![no_std]

extern crate alloc;

extern crate playdate as pd;
// extern crate playdate_controls as controls;

use pd::controls::buttons::PDButtonsExt;
use pd::controls::peripherals::Buttons;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use draw::*;
use num_traits::real::Real;
use pd::controls::peripherals::Crank;

use crankit_game_loop::{game_loop, Game, Playdate};
use pd::display::Display;
use pd::graphics::bitmap::*;
use pd::graphics::*;
use pd::system::prelude::*;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

mod draw;
mod utils;

use utils::*;

const START_SCORE: f32 = 100.0;
const CIAMPINO: usize = 30000;
const SPEED: f32 = 5.0;
const DELTA_TO_METERS: f32 = 5.0;
const INERTIA: f32 = 0.005;
const LAST_STATION: usize = TRAIN_STOPS.len() - 1;
const SPEED_SCORE_MULTIPLIER: f32 = 15.0;

const POINT2D_ZERO: Point<i32> = Point::new(0, 0);
const SIZE2D_SCREEN_SIZE: Point<i32> = Point::new(400, 240);

/// 2D point
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    const fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

const TRAIN_STOPS: [(usize, &str); 12] = [
    (0, "P. Nuova"),
    (800, "P. Susa"),
    (5500, "P. Garibaldi"),
    (7300, "Rogoredo"),
    (10900, "R. Emilia"),
    (13700, "Bologna C."),
    (17500, "Fir. SMN"),
    (26300, "Tiburtina"),
    (CIAMPINO, "Ciampino"),
    (33500, "Napoli C."),
    (38200, "Salerno"),
    (666666, "Game Over"),
];

pub const fn prime_minister_rating(distance: i32) -> &'static str {
    match distance {
        0..=5 => "Minister plays your skin flute",
        6..=50 => "Minister gives you a promotion",
        51..=200 => "Minister gives you a handshake",
        201..=500 => "Minister is pleased",
        501..=1000 => "Minister is a bit upset",
        1001..=5000 => "Minister is pissed off",
        5001..=10000 => "Minister slaps your face",
        _ => "Minister fires you and your family",
    }
}

#[derive(PartialEq)]
enum GameState {
    Start,
    During,
    Arrived,
    Exploded,
}

#[derive(Default)]
struct Train {
    velocity: f32,
    current_stop: usize,
}

impl Train {
    fn get_next_stop_distance(&mut self, position: i32) -> i32 {
        let distance = (TRAIN_STOPS[self.current_stop].0 as i32) - position;
        if distance <= 0 {
            self.current_stop += 1;
        }
        distance
    }
}

pub struct MyGame {
    state: GameState,
    delta: f32,
    train: Train,
    score: f32,
    rng: SmallRng,
    init_timer: Timer,
    explosion_timer: Timer,
    game_over_timer: Timer,
    was_crank_moved: bool,
}

impl Game for MyGame {
    fn new(_: &Playdate) -> Self {
        Display::Cached().set_refresh_rate(50.0);

        let time = System::Cached().seconds_since_epoch();
        let rng = SmallRng::seed_from_u64(time as u64);
        Self {
            state: GameState::Start,
            delta: 0.0,
            train: Train {
                velocity: 0.5,
                ..Default::default()
            },
            score: START_SCORE,
            rng,
            init_timer: Timer::new(0, 20, false),
            explosion_timer: Timer::new(0, 40, true),
            game_over_timer: Timer::new(0, 20, true),
            was_crank_moved: false,
        }
    }

    fn update(&mut self, pd: &Playdate) {
        clear(Color::WHITE);

        #[cfg(feature = "draw-fps")]
        System::Default().draw_fps(0, 0);

        let buttons = Buttons::Cached().get();
        let crank_change = Crank::Cached().change();

        // Reset if B is pressed
        if (self.state == GameState::Arrived || self.state == GameState::Exploded)
            && buttons.current.b()
        {
            *self = MyGame::new(pd);
        }

        // Movement
        if self.state == GameState::Start {
            self.delta += self.train.velocity * SPEED;
            if buttons.current.a() {
                // Once timer reaches end, switches game state to during
                self.init_timer.start();
            }
        } else if self.state == GameState::During {
            self.delta += self.train.velocity * SPEED;
            self.train.velocity += clamp(crank_change, 0.0, f32::MAX) / 1500.0;
            // If user has not moved crank, velocity will not be decreased
            if self.was_crank_moved {
                self.train.velocity -= INERTIA;
            }
            self.score -= 0.02;
        }

        // Train can never move below 0.0 of velocity
        self.train.velocity = clamp(self.train.velocity, 0.0, f32::MAX);
        if self.train.velocity > (1.0 - f32::EPSILON) {
            self.state = GameState::Exploded;
        }
        if self.train.velocity < (0.0 + f32::EPSILON) {
            self.state = GameState::Arrived;
        };

        // Screen shake
        if self.state == GameState::During {
            if self.train.velocity > 0.95 {
                screen_shake(4, &mut self.rng);
            } else if self.train.velocity > 0.9 {
                screen_shake(2, &mut self.rng);
            } else if self.train.velocity > 0.8 {
                screen_shake(1, &mut self.rng);
            } else {
                Display::Cached().set_offset(POINT2D_ZERO.x, POINT2D_ZERO.y);
            }
        } else {
            Display::Cached().set_offset(POINT2D_ZERO.x, POINT2D_ZERO.y);
        }

        // Get current and next stop
        let current_stop_name = if self.train.current_stop != 0 {
            TRAIN_STOPS[self.train.current_stop - 1].1
        } else {
            ""
        };
        let next_stop_distance = self
            .train
            .get_next_stop_distance((self.delta * DELTA_TO_METERS) as i32);
        let next_stop_name = TRAIN_STOPS[self.train.current_stop].1;

        // Draw stuff
        draw_mountains(self.delta);
        draw_train();
        draw_wheels(self.delta);
        draw_wheel_bars(self.delta);
        draw_floor();

        if self.state != GameState::Start {
            draw_stops(
                current_stop_name,
                next_stop_distance,
                next_stop_name,
                self.train.current_stop == LAST_STATION,
            );
            draw_velocity_bar(
                self.train.velocity,
                self.delta,
                self.state == GameState::During,
            );
        }

        draw_pillars(self.delta);

        // UI
        if self.state == GameState::Exploded {
            if !self.explosion_timer.step() {
                Display::Cached().set_refresh_rate(20.0);
                draw_explosion(self.explosion_timer.get_value(), &mut self.rng);
                screen_shake(20, &mut self.rng);
            } else {
                Display::Cached().set_refresh_rate(50.0);
                draw_post_explosion_screen(self.game_over_timer.get_percentage());
                self.game_over_timer.step();
            }
        }

        // Game over screen
        if self.state == GameState::Arrived {
            let abs_distance_score =
                ((self.delta * DELTA_TO_METERS - CIAMPINO as f32) as i32).abs();
            let speed_score = (self.score * SPEED_SCORE_MULTIPLIER) as i32;
            let unclamped_score = speed_score + (1000 - abs_distance_score);
            let final_score = clamp(unclamped_score, 0, i32::MAX);

            draw_game_ended_screen(
                self.game_over_timer.get_percentage(),
                self.delta,
                final_score,
            );

            self.game_over_timer.step();
        }

        // Intro screen
        if self.state == GameState::Start {
            draw_intro_screen(self.init_timer.get_percentage(), self.delta);

            if self.init_timer.step() {
                // Once timer reaches end, switches game state to during
                self.delta = 0.0;
                self.train.current_stop = 0;
                self.state = GameState::During;
            }
        }

        if !self.was_crank_moved && self.state == GameState::During && crank_change > f32::EPSILON {
            self.was_crank_moved = true;
        }
    }
}

game_loop!(MyGame);
