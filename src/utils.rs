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

pub struct Timer {
    current: i32,
    end: i32,
    reached: bool,
}

impl Timer {
    #[inline(always)]
    pub fn new(start: i32, end: i32) -> Self {
        Self {
            current: start,
            end,
            reached: false,
        }
    }

    #[inline(always)]
    pub const fn get_value(&self) -> i32 {
        self.current
    }

    #[inline(always)]
    pub fn step(&mut self) -> bool {
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
