#![no_std]

extern crate alloc;

use core::f32::MAX;

use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::ToString;
use anyhow::Error;
use crankstart::display::Display;
use crankstart::geometry::ScreenRect;
use crankstart::{crankstart_game, system::*, Game, Playdate};
use crankstart::{graphics::*, log_to_console};
use crankstart_sys::PDButtons;
use draw::*;
use euclid::{Point2D, Size2D, Vector2D};
use rand::rngs::SmallRng;
use rand::SeedableRng;

mod draw;
mod utils;

use draw::*;
use utils::*;

const START_SCORE: f32 = 100.0;
const CIAMPINO: usize = 28000;
const SPEED: f32 = 5.0;
const DELTA_TO_METERS: f32 = 3.0;
const INERTIA: f32 = 0.003;
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

struct State {
    state: GameState,
    delta: f32,
    train: Train,
    score: f32,
    rng: SmallRng,
}

impl State {
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
        }))
    }
}

impl Game for State {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        Graphics::get().clear(LCDColor::Solid(LCDSolidColor::kColorWhite))?;
        let system = System::get();
        let (pressed, _, _) = system.get_button_state()?;
        let crank_change = system.get_crank_change()?;

        // Reset if B is pressed
        if pressed & PDButtons::kButtonB == PDButtons::kButtonB {
            *self = *State::new(_playdate)?;
        }

        // Movement
        if self.state == GameState::Start {
            self.delta += self.train.velocity * SPEED;
            if pressed & PDButtons::kButtonA == PDButtons::kButtonA {
                self.delta = 0.0;
                self.state = GameState::During;
            }
        } else if self.state == GameState::During {
            self.delta += self.train.velocity * SPEED;
            self.train.velocity += clamp(crank_change, 0.0, f32::MAX) / 2000.0;
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

        if self.train.velocity > 0.95 {
            screen_shake(4, &mut self.rng)?;
        } else if self.train.velocity > 0.9 {
            screen_shake(2, &mut self.rng)?;
        } else if self.train.velocity > 0.8 {
            screen_shake(1, &mut self.rng)?;
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
        draw_train(self.delta)?;
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
            draw_score(self.score as usize)?;
        }
        // UI
        if self.state == GameState::Exploded {
            Display::get().set_refresh_rate(20.0)?;
            draw_explosion(self.delta, &mut self.rng)?;
            screen_shake(16, &mut self.rng)?;
        }
        draw_pillars(self.delta)?;
        // Game over screen
        if self.state == GameState::Arrived {
            draw_game_over_screen(self.delta)?;
        }
        Ok(())
    }
}

crankstart_game!(State);
