#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use anyhow::Error;
use crankstart::{crankstart_game, Game, Playdate};

struct GGJ2024 {}

impl GGJ2024 {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        Ok(Box::new(Self {}))
    }
}

impl Game for GGJ2024 {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        Ok(())
    }
}

crankstart_game!(GGJ2024);
