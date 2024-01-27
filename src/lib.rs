#![no_std]

extern crate alloc;

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

const SPEED: f32 = 5.0;
const INERTIA: f32 = 0.003;

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
}

struct State {
    state: GameState,
    delta: f32,
    train: Train,
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
            state: GameState::During,
            delta: 0.0,
            train,
            rng,
        }))
    }
}

impl Game for State {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        Graphics::get().clear(LCDColor::Solid(LCDSolidColor::kColorWhite))?;
        let system = System::get();
        let crank_change = system.get_crank_change()?;
        // Movement
        if self.state == GameState::During {
            self.delta += self.train.velocity * SPEED;
            self.train.velocity += clamp(crank_change, 0.0, f32::MAX) / 2000.0;
            self.train.velocity -= INERTIA;
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

        // Draw stuff
        draw_mountains(self.delta)?;
        draw_train(self.delta)?;
        draw_wheels(self.delta)?;
        draw_wheel_bars(self.delta)?;
        draw_pillars(self.delta)?;
        draw_floor()?;
        // UI
        draw_velocity_bar(
            self.train.velocity,
            self.delta,
            self.state == GameState::During,
        )?;
        if self.state == GameState::Exploded {
            Display::get().set_refresh_rate(15.0)?;
            draw_explosion(self.delta, &mut self.rng)?;
            screen_shake(20, &mut self.rng)?;
        }
        Ok(())
    }
}

crankstart_game!(State);
