use alloc::string::String;
use anyhow::Ok;
use crankstart::{graphics, log_to_console};
use euclid::Trig;
use rand::{
    distributions::{self, DistMap},
    RngCore,
};

use super::*;

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
        let first_wheel_x = ((delta / 10.0).sin() * 26.0) as i32;
        let first_wheel_y = ((delta / 10.0).cos() * 26.0) as i32;
        let second_wheel_x = (((delta + 15.7) / 10.0).sin() * 26.0) as i32;
        let second_wheel_y = (((delta + 15.7) / 10.0).cos() * 26.0) as i32;
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
    let sin = ((delta / 10.0).sin() * 28.0) as i32;
    let cos = ((delta / 10.0).cos() * 28.0) as i32;
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
pub fn draw_stops(
    current_stop_name: &str,
    distance: i32,
    next_stop_name: &str,
    out_of_bounds: bool,
) -> Result<(), Error> {
    let graphics = Graphics::get();
    graphics.fill_rect(
        ScreenRect::new(Point2D::new(232, 24), Size2D::new(144, 67)),
        white(),
    )?;
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(232, 24), Size2D::new(144, 67)),
        black(),
    )?;
    // Arrow arm
    graphics.draw_line(Point2D::new(232, 31), Point2D::new(369, 31), 1, black())?;
    graphics.draw_line(Point2D::new(369, 31), Point2D::new(369, 52), 1, black())?;
    // Arrow pointing down
    graphics.draw_line(Point2D::new(369, 54), Point2D::new(363, 48), 1, black())?;
    graphics.draw_line(Point2D::new(369, 54), Point2D::new(375, 48), 1, black())?;
    // Text
    Graphics::get().draw_text(current_stop_name, Point2D::new(243, 40))?;
    // If out of bounds
    if out_of_bounds {
        Graphics::get().draw_text("Ndo cazzo vai", Point2D::new(243, 64))?;
        return Ok(());
    }
    // If in bounds
    let distance_str: String = if distance >= 1000 {
        let mut d = distance.to_string();
        // Remove two last characters
        d.pop();
        d.pop();
        d + "km"
    } else {
        distance.to_string() + "m"
    };
    Graphics::get().draw_text(
        &(distance_str + " " + next_stop_name),
        Point2D::new(243, 64),
    )?;
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
    // Text
    Graphics::get().draw_text("Velocity:", Point2D::new(241, 151))?;
    // Bar
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
    if should_blink && value > 0.95 && delta % 40.0 < 20.0 {
        return Ok(());
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
pub fn draw_score(score: usize) -> Result<(), Error> {
    Graphics::get().draw_text(
        &("Score: ".to_owned() + &score.to_string()),
        Point2D::new(16, 16),
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
pub fn draw_game_over_screen(delta: f32) -> Result<(), Error> {
    let graphics = Graphics::get();
    // Boxes
    graphics.fill_rect(
        ScreenRect::new(Point2D::new(29, 71), Size2D::new(342, 98)),
        white(),
    )?;
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(29, 71), Size2D::new(342, 98)),
        black(),
    )?;
    graphics.draw_rect(
        ScreenRect::new(Point2D::new(32, 74), Size2D::new(336, 92)),
        black(),
    )?;
    // Text
    let distance = (delta * DELTA_TO_METERS - CIAMPINO as f32) as i32;
    let mut text = String::new();
    text.push_str("You are ");
    let measurement_unit = if distance.abs() >= 1000 {
        let mut d = distance.abs().to_string();
        d.pop();
        d.pop();
        d + " km"
    } else {
        let d = distance.abs().to_string();
        d + " meters"
    };
    text.push_str(&measurement_unit);
    text.push_str(if distance.is_negative() {
        " before Ciampino"
    } else {
        "after Ciampino"
    });
    Graphics::get().draw_text(&text, Point2D::new(44, 84))?;
    Graphics::get().draw_text("Final score: 2000", Point2D::new(44, 108))?;
    Graphics::get().draw_text("Press B to try again", Point2D::new(44, 141))?;
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
