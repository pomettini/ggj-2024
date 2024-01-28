use super::*;

#[inline(always)]
pub const fn white() -> LCDColor {
    LCDColor::Solid(LCDSolidColor::kColorWhite)
}

#[inline(always)]
pub const fn black() -> LCDColor {
    LCDColor::Solid(LCDSolidColor::kColorBlack)
}

#[inline(always)]
pub const fn xor() -> LCDColor {
    LCDColor::Solid(LCDSolidColor::kColorXOR)
}

#[inline(always)]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

#[inline(always)]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub struct Timer {
    current: i32,
    end: i32,
    started: bool,
    reached: bool,
}

impl Timer {
    #[inline(always)]
    pub fn new(start: i32, end: i32, auto_start: bool) -> Self {
        Self {
            current: start,
            end,
            started: auto_start,
            reached: false,
        }
    }

    #[inline(always)]
    pub fn start(&mut self) {
        self.started = true;
    }

    #[inline(always)]
    pub const fn get_value(&self) -> i32 {
        self.current
    }

    #[inline(always)]
    pub fn get_percentage(&self) -> f32 {
        self.current as f32 / self.end as f32
    }

    #[inline(always)]
    pub fn step(&mut self) -> bool {
        if !self.started {
            return false;
        }
        if self.reached {
            return true;
        }
        self.current += 1;
        if self.current >= self.end {
            self.reached = true;
            return true;
        }
        false
    }
}
