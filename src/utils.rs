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
