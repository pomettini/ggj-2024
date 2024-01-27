use crankstart::{graphics, log_to_console};
use euclid::Trig;
use rand::RngCore;

use super::*;

#[inline(always)]
const fn white() -> LCDColor {
    LCDColor::Solid(LCDSolidColor::kColorWhite)
}

#[inline(always)]
const fn black() -> LCDColor {
    LCDColor::Solid(LCDSolidColor::kColorBlack)
}

#[inline(always)]
const fn xor() -> LCDColor {
    LCDColor::Solid(LCDSolidColor::kColorXOR)
}

#[inline(always)]
pub fn draw_mountains(delta: f32) -> Result<(), Error> {
    let graphics = Graphics::get();
    let delta = delta % (89.0 * 2.0);
    for i in (0..8).step_by(2) {
        graphics.draw_line(
            Point2D::new((89 * i) - delta as i32, 116),
            Point2D::new((89 * (i + 1)) - delta as i32, 27),
            1,
            black(),
        )?;
        graphics.draw_line(
            Point2D::new(89 * (i + 1) - delta as i32, 27),
            Point2D::new(89 * (i + 2) - delta as i32, 116),
            1,
            black(),
        )?;
    }
    Ok(())
}

#[inline(always)]
pub fn draw_train(delta: f32) -> Result<(), Error> {
    let graphics = Graphics::get();
    // Body
    graphics.fill_rect(
        ScreenRect::new(Point2D::new(-1, 60), Size2D::new(201, 120)),
        white(),
    )?;
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(-1, 60), Size2D::new(201, 120)),
        black(),
    )?;
    // Leftmost window
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(-40, 82), Size2D::new(60, 45)),
        black(),
    )?;
    // Central window
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(40, 82), Size2D::new(60, 45)),
        black(),
    )?;
    // Rightmost window
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(120, 82), Size2D::new(30, 45)),
        black(),
    )?;
    Ok(())
}

#[inline(always)]
pub fn draw_wheels(delta: f32) -> Result<(), Error> {
    let graphics = Graphics::get();
    for i in 0..3 {
        let distance = 80;
        // Wheel
        graphics.fill_ellipse(
            None,
            None,
            Point2D::new(-40 + (distance * i), 150),
            Size2D::new(60, 60),
            1,
            0.0,
            0.0,
            white(),
            LCDRect {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            },
        )?;
        graphics.draw_ellipse(
            Point2D::new(-40 + (distance * i), 150),
            Size2D::new(60, 60),
            1,
            0.0,
            0.0,
            black(),
        )?;
        graphics.draw_ellipse(
            Point2D::new(-36 + (distance * i), 154),
            Size2D::new(52, 52),
            1,
            0.0,
            0.0,
            black(),
        )?;
        let first_wheel_x = ((delta as f32 / 10.0).sin() * 26.0) as i32;
        let first_wheel_y = ((delta as f32 / 10.0).cos() * 26.0) as i32;
        let second_wheel_x = ((((delta as f32) + 15.7) / 10.0).sin() * 26.0) as i32;
        let second_wheel_y = ((((delta as f32) + 15.7) / 10.0).cos() * 26.0) as i32;
        // First wheel
        graphics.draw_line(
            Point2D::new(
                (distance * i) + -36 + 25 - first_wheel_x,
                25 + 154 + first_wheel_y,
            ),
            Point2D::new(
                (distance * i) + -36 + 25 + first_wheel_x,
                25 + 154 - first_wheel_y,
            ),
            1,
            black(),
        )?;
        // Second wheel
        graphics.draw_line(
            Point2D::new(
                (distance * i) + -36 + 25 - second_wheel_x,
                25 + 154 + second_wheel_y,
            ),
            Point2D::new(
                (distance * i) + -36 + 25 + second_wheel_x,
                25 + 154 - second_wheel_y,
            ),
            1,
            black(),
        )?;
    }
    Ok(())
}

#[inline(always)]
pub fn draw_wheel_bars(delta: f32) -> Result<(), Error> {
    let graphics = Graphics::get();
    let sin = ((delta as f32 / 10.0).sin() * 28.0) as i32;
    let cos = ((delta as f32 / 10.0).cos() * 28.0) as i32;
    graphics.fill_rect(
        ScreenRect::new(Point2D::new(-29 - sin, 177 + cos), Size2D::new(181, 6)),
        white(),
    )?;
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(-29 - sin, 177 + cos), Size2D::new(181, 6)),
        black(),
    )?;
    Ok(())
}

#[inline(always)]
pub fn draw_pillars(delta: f32) -> Result<(), Error> {
    let graphics = Graphics::get();
    let delta = delta * 5.0 % 120.0;
    for i in 0..5 {
        graphics.fill_rect(
            ScreenRect::new(
                Point2D::new((120 * i) - delta as i32, -1),
                Size2D::new(10, 212),
            ),
            white(),
        )?;
        graphics.draw_rect(
            ScreenRect::new(
                Point2D::new((120 * i) - delta as i32, -1),
                Size2D::new(10, 212),
            ),
            black(),
        )?;
    }
    Ok(())
}

#[inline(always)]
pub fn draw_floor() -> Result<(), Error> {
    Graphics::get().draw_line(
        Point2D::new(0, 210),
        Point2D::new(400, 210),
        1,
        LCDColor::Solid(LCDSolidColor::kColorBlack),
    )?;
    Ok(())
}

#[inline(always)]
pub fn draw_velocity_bar(value: f32, delta: f32, should_blink: bool) -> Result<(), Error> {
    let graphics = Graphics::get();
    let value = clamp(value, 0.0, 1.0);
    graphics.fill_rect(
        ScreenRect::new(Point2D::new(241, 175), Size2D::new(138, 15)),
        white(),
    )?;
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(241, 175), Size2D::new(138, 15)),
        black(),
    )?;
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(244, 178), Size2D::new(132, 9)),
        black(),
    )?;
    // Bar will flash if above 95%
    if should_blink && value > 0.95 {
        if delta % 40.0 < 20.0 {
            return Ok(());
        }
    }
    graphics.fill_rect(
        ScreenRect::new(
            Point2D::new(244, 178),
            Size2D::new(((132 as f32) * value) as i32, 9),
        ),
        black(),
    )?;
    Ok(())
}

#[inline(always)]
pub fn draw_explosion(delta: f32, rng: &mut SmallRng) -> Result<(), Error> {
    let graphics = Graphics::get();
    for i in 0..10 {
        let x = rng.next_u32() % 200;
        let y = rng.next_u32() % 120;
        graphics.fill_ellipse(
            None,
            None,
            Point2D::new(-20 + x as i32, 30 + y as i32),
            Size2D::new(40, 40),
            1,
            0.0,
            0.0,
            if i >= 5 { white() } else { black() },
            LCDRect {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            },
        )?;
    }
    Ok(())
}

#[inline(always)]
pub fn screen_shake(amount: usize, rng: &mut SmallRng) -> Result<(), Error> {
    let x = rng.next_u32() % (amount * 2) as u32;
    let y = rng.next_u32() % (amount * 2) as u32;
    Display::get().set_offset(Point2D::new(
        (amount as i32) - x as i32,
        (amount as i32) - y as i32,
    ))?;
    Ok(())
}
