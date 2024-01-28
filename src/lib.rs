#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use anyhow::Error;
use crankstart::display::Display;
use crankstart::geometry::ScreenRect;
use crankstart::{crankstart_game, graphics::*, log_to_console, system::*, Game, Playdate};
use crankstart_sys::PDButtons;
use draw::*;
use euclid::Trig;
use euclid::{Point2D, Size2D};
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
const LAST_STATION: usize = 11;

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
    #[inline(always)]
    fn get_next_stop_distance(&mut self, position: i32) -> i32 {
        let distance = (TRAIN_STOPS[self.current_stop].0 as i32) - position;
        if distance <= 0 {
            self.current_stop += 1;
        }
        distance
    }
}

struct State {
    state: GameState,
    delta: f32,
    train: Train,
    score: f32,
    rng: SmallRng,
    init_timer: Timer,
    explosion_timer: Timer,
    game_over_timer: Timer,
}

impl State {
    #[inline(always)]
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        Display::get().set_refresh_rate(50.0)?;
        let mut train = Train::default();
        train.velocity = 0.5;
        let (_, time) = System::get().get_seconds_since_epoch().unwrap();
        let rng = SmallRng::seed_from_u64(time as u64);
        Ok(Box::new(Self {
            state: GameState::Start,
            delta: 0.0,
            train,
            score: START_SCORE,
            rng,
            init_timer: Timer::new(0, 20, false),
            explosion_timer: Timer::new(0, 40, true),
            game_over_timer: Timer::new(0, 20, true),
        }))
    }
}

impl Game for State {
    #[inline(always)]
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        Graphics::get().clear(LCDColor::Solid(LCDSolidColor::kColorWhite))?;
        let system = System::get();
        let (pressed, _, _) = system.get_button_state()?;
        let crank_change = system.get_crank_change()?;

        // Reset if B is pressed
        if (self.state == GameState::Arrived || self.state == GameState::Exploded)
            && pressed & PDButtons::kButtonB == PDButtons::kButtonB
        {
            *self = *State::new(_playdate)?;
        }

        // Movement
        if self.state == GameState::Start {
            self.delta += self.train.velocity * SPEED;
            if pressed & PDButtons::kButtonA == PDButtons::kButtonA {
                // Once timer reaches end, switches game state to during
                self.init_timer.start();
            }
        } else if self.state == GameState::During {
            self.delta += self.train.velocity * SPEED;
            self.train.velocity += clamp(crank_change, 0.0, f32::MAX) / 1500.0;
            self.train.velocity -= INERTIA;
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
                screen_shake(4, &mut self.rng)?;
            } else if self.train.velocity > 0.9 {
                screen_shake(2, &mut self.rng)?;
            } else if self.train.velocity > 0.8 {
                screen_shake(1, &mut self.rng)?;
            } else {
                Display::get().set_offset(Point2D::new(0, 0))?;
            }
        } else {
            Display::get().set_offset(Point2D::new(0, 0))?;
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
        draw_mountains(self.delta)?;
        draw_train()?;
        draw_wheels(self.delta)?;
        draw_wheel_bars(self.delta)?;
        draw_floor()?;

        if self.state != GameState::Start {
            draw_stops(
                current_stop_name,
                next_stop_distance,
                next_stop_name,
                self.train.current_stop == LAST_STATION,
            )?;
            draw_velocity_bar(
                self.train.velocity,
                self.delta,
                self.state == GameState::During,
            )?;
            // draw_score(self.score as usize)?;
        }

        draw_pillars(self.delta)?;

        // UI
        if self.state == GameState::Exploded {
            if !self.explosion_timer.step() {
                Display::get().set_refresh_rate(20.0)?;
                draw_explosion(self.explosion_timer.get_value(), &mut self.rng)?;
                screen_shake(20, &mut self.rng)?;
            } else {
                Display::get().set_refresh_rate(50.0)?;
                draw_post_explosion_screen(self.game_over_timer.get_percentage())?;
                self.game_over_timer.step();
            }
        }

        // Game over screen
        if self.state == GameState::Arrived {
            let abs_distance_score =
                ((self.delta * DELTA_TO_METERS - CIAMPINO as f32) as i32).abs();
            let speed_score = (self.score * 15.0) as i32;
            let unclamped_score = speed_score + (1000 - abs_distance_score);
            let final_score = clamp(unclamped_score, 0, i32::MAX);
            /*
            log_to_console!(
                "{}, {}, {}",
                abs_distance_score,
                speed_score,
                unclamped_score
            );
            */
            draw_game_ended_screen(
                self.game_over_timer.get_percentage(),
                self.delta,
                final_score,
            )?;
            self.game_over_timer.step();
        }

        // Intro screen
        if self.state == GameState::Start {
            draw_intro_screen(self.init_timer.get_percentage(), self.delta)?;
            if self.init_timer.step() {
                // Once timer reaches end, switches game state to during
                self.delta = 0.0;
                self.state = GameState::During;
            }
        }

        // system.draw_fps(0, 0)?;
        Ok(())
    }
}

crankstart_game!(State);
