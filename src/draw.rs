use super::*;

pub fn draw_mountains(delta: f32) {
    let graphics = Graphics::Default();

    let delta = delta % (89.0 * 2.0);

    for i in (0..8).step_by(2) {
        // Bottom left to top middle
        graphics.draw_line(
            (89 * i) - delta as i32,
            116,
            (89 * (i + 1)) - delta as i32,
            27,
            1,
            LCDColorConst::BLACK,
        );
        // Top middle to bottom right
        graphics.draw_line(
            89 * (i + 1) - delta as i32,
            27,
            89 * (i + 2) - delta as i32,
            116,
            1,
            LCDColorConst::BLACK,
        );
    }
}

pub fn draw_train() {
    let graphics = Graphics::Default();

    // Body
    graphics.fill_rect(-1, 60, 201, 120, LCDColorConst::WHITE);
    graphics.draw_rect(-1, 60, 201, 120, LCDColorConst::BLACK);
    // Leftmost window
    graphics.draw_rect(-40, 82, 60, 45, LCDColorConst::BLACK);
    // Central window
    graphics.draw_rect(40, 82, 60, 45, LCDColorConst::BLACK);
    // Rightmost window
    graphics.draw_rect(120, 82, 30, 45, LCDColorConst::BLACK);
}

pub fn draw_wheels(delta: f32) {
    let graphics = Graphics::Default();

    for i in 0..3 {
        let distance = 80;
        // Wheel
        graphics.fill_ellipse(
            -40 + (distance * i),
            150,
            60,
            60,
            0.0,
            0.0,
            LCDColorConst::WHITE,
        );
        graphics.draw_ellipse(
            -40 + (distance * i),
            150,
            60,
            60,
            1,
            0.0,
            0.0,
            LCDColorConst::BLACK,
        );
        graphics.draw_ellipse(
            -36 + (distance * i),
            154,
            52,
            52,
            1,
            0.0,
            0.0,
            LCDColorConst::BLACK,
        );
        let first_wheel_x = ((delta / 10.0).sin() * 26.0) as i32;
        let first_wheel_y = ((delta / 10.0).cos() * 26.0) as i32;
        let second_wheel_x = (((delta + 15.7) / 10.0).sin() * 26.0) as i32;
        let second_wheel_y = (((delta + 15.7) / 10.0).cos() * 26.0) as i32;
        // First wheel
        graphics.draw_line(
            (distance * i) + -36 + 25 - first_wheel_x,
            25 + 154 + first_wheel_y,
            (distance * i) + -36 + 25 + first_wheel_x,
            25 + 154 - first_wheel_y,
            1,
            LCDColorConst::BLACK,
        );
        // Second wheel
        graphics.draw_line(
            (distance * i) + -36 + 25 - second_wheel_x,
            25 + 154 + second_wheel_y,
            (distance * i) + -36 + 25 + second_wheel_x,
            25 + 154 - second_wheel_y,
            1,
            LCDColorConst::BLACK,
        );
    }
}

pub fn draw_wheel_bars(delta: f32) {
    let graphics = Graphics::Default();

    let sin = ((delta / 10.0).sin() * 28.0) as i32;
    let cos = ((delta / 10.0).cos() * 28.0) as i32;

    graphics.fill_rect(-29 - sin, 177 + cos, 181, 6, LCDColorConst::WHITE);
    graphics.draw_rect(-29 - sin, 177 + cos, 181, 6, LCDColorConst::BLACK);
}

pub fn draw_pillars(delta: f32) {
    let graphics = Graphics::Default();
    let delta = delta * 5.0 % 120.0;

    for i in 0..5 {
        graphics.fill_rect((120 * i) - delta as i32, -1, 10, 212, LCDColorConst::WHITE);
        graphics.draw_rect((120 * i) - delta as i32, -1, 10, 212, LCDColorConst::BLACK);
    }
}

pub fn draw_stops(
    current_stop_name: &str,
    distance: i32,
    next_stop_name: &str,
    out_of_bounds: bool,
) {
    let graphics = Graphics::Default();
    graphics.fill_rect(232, 24, 144, 67, LCDColorConst::WHITE);
    graphics.draw_rect(232, 24, 144, 67, LCDColorConst::BLACK);
    // Arrow arm
    graphics.draw_line(232, 31, 369, 31, 1, LCDColorConst::BLACK);
    graphics.draw_line(369, 31, 369, 52, 1, LCDColorConst::BLACK);
    // Arrow pointing down
    graphics.draw_line(369, 54, 363, 48, 1, LCDColorConst::BLACK);
    graphics.draw_line(369, 54, 375, 48, 1, LCDColorConst::BLACK);
    // Text
    Graphics::Default().draw_text(current_stop_name, 243, 40);
    // If out of bounds
    if out_of_bounds {
        Graphics::Default().draw_text("Wtf are u going", 243, 64);
        return;
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
    Graphics::Default().draw_text(&(distance_str + " " + next_stop_name), 243, 64);
}

pub fn draw_floor() {
    Graphics::Default().draw_line(0, 210, 400, 210, 1, LCDColorConst::BLACK);
}

pub fn draw_velocity_bar(value: f32, delta: f32, should_blink: bool) {
    let graphics = Graphics::Default();
    let value = clamp(value, 0.0, 1.0);

    // Text
    Graphics::Default().draw_text("Velocity:", 232, 154);

    // Bar
    graphics.fill_rect(232, 178, 144, 15, LCDColorConst::WHITE);
    graphics.draw_rect(232, 178, 144, 15, LCDColorConst::BLACK);

    graphics.draw_rect(235, 181, 138, 9, LCDColorConst::BLACK);

    // Bar will flash if above 95%
    if should_blink && value > 0.95 && delta % 40.0 < 20.0 {
        return;
    }

    graphics.fill_rect(
        235,
        181,
        ((138 as f32) * value) as i32,
        9,
        LCDColorConst::BLACK,
    );
}

pub fn draw_score(score: usize) {
    Graphics::Default().draw_text(&("Score: ".to_owned() + &score.to_string()), 16, 16);
}

pub fn draw_explosion(timer: i32, rng: &mut SmallRng) {
    let graphics = Graphics::Default();

    for i in 0..10 {
        let x = rng.next_u32() % 200;
        let y = rng.next_u32() % 120;
        graphics.fill_ellipse(
            -20 + x as i32,
            30 + y as i32,
            40,
            40,
            0.0,
            0.0,
            if i >= 5 {
                LCDColorConst::WHITE
            } else {
                LCDColorConst::BLACK
            },
        );
    }

    if timer % 4 == 0 {
        graphics.fill_rect(
            POINT2D_ZERO.x,
            POINT2D_ZERO.y,
            SIZE2D_SCREEN_SIZE.x,
            SIZE2D_SCREEN_SIZE.y,
            LCDColorConst::XOR,
        );
    }
}

pub fn draw_post_explosion_screen(timer: f32) {
    let graphics = Graphics::Default();

    // Background
    graphics.fill_rect(
        POINT2D_ZERO.x,
        POINT2D_ZERO.y,
        SIZE2D_SCREEN_SIZE.x,
        SIZE2D_SCREEN_SIZE.y,
        LCDColorConst::BLACK,
    );

    // Box
    graphics.fill_rect(
        29,
        71,
        (342.0 * timer) as i32,
        (98.0 * timer) as i32,
        LCDColorConst::WHITE,
    );

    // Show edges and text only if timer has been reached
    if timer < 1.0 - f32::EPSILON {
        return;
    }

    graphics.draw_rect(29, 71, 342, 98, LCDColorConst::BLACK);
    graphics.draw_rect(32, 74, 336, 92, LCDColorConst::BLACK);

    // Text
    Graphics::Default().draw_text("I told you not accelerate too much", 44, 84);
    Graphics::Default().draw_text("Oh and by the way, you're fired", 44, 108);
    Graphics::Default().draw_text("Press B to try again", 44, 141);
}

pub fn draw_game_ended_screen(timer: f32, delta: f32, score: i32) {
    let graphics = Graphics::Default();

    // Background
    graphics.fill_rect(
        POINT2D_ZERO.x,
        POINT2D_ZERO.y,
        SIZE2D_SCREEN_SIZE.x,
        SIZE2D_SCREEN_SIZE.y,
        LCDColorConst::XOR,
    );

    // Box
    graphics.fill_rect(
        28,
        55,
        (344.0 * timer) as i32,
        (129.0 * timer) as i32,
        LCDColorConst::WHITE,
    );

    // Show edges and text only if timer has been reached
    if timer < 1.0 - f32::EPSILON {
        return;
    }

    // Edges
    graphics.draw_rect(28, 55, 344, 129, LCDColorConst::BLACK);
    graphics.draw_rect(32, 59, 336, 121, LCDColorConst::BLACK);

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
        " after Ciampino"
    });

    Graphics::Default().draw_text(&text, 44, 69);
    Graphics::Default().draw_text(prime_minister_rating(distance.abs()), 44, 93);
    Graphics::Default().draw_text(&("Final score: ".to_owned() + &score.to_string()), 44, 117);
    Graphics::Default().draw_text("Press B to try again", 44, 155);
}

pub fn draw_intro_screen(timer: f32, delta: f32) {
    let graphics = Graphics::Default();
    let scale = 1.0 - timer;

    // If timer is ongoing, play animation
    if timer >= 0.0 + f32::EPSILON {
        graphics.fill_rect(
            22,
            30,
            (356.0 * scale) as i32,
            (179.0 * scale) as i32,
            LCDColorConst::WHITE,
        );
        graphics.draw_rect(
            22,
            30,
            (356.0 * scale) as i32,
            (179.0 * scale) as i32,
            LCDColorConst::BLACK,
        );
    }

    // Otherwise draw window trail and text
    for i in 0..20 {
        let x = ((i as f32 + (delta / 30.0)).sin() * (i as f32 / 3.0)) as i32;
        let y = ((i as f32 + (delta / 30.0)).cos() * (i as f32 / 3.0)) as i32;
        graphics.fill_rect(x + 22, -y + 30, 356, 179, LCDColorConst::WHITE);
        graphics.draw_rect(x + 22, -y + 30, 356, 179, LCDColorConst::BLACK);
        if i == 19 {
            graphics.draw_text("Train to Ciampino", 44, 50);
            graphics.draw_text("A minister is requesting a special stop", 44, 88);
            graphics.draw_text("Turn the crank to accelerate the train", 44, 112);
            graphics.draw_text("But not too much or it will explode", 44, 136);
            graphics.draw_text("Press A to start", 44, 174);
        }
    }
}

#[inline(always)]
pub fn screen_shake(amount: usize, rng: &mut SmallRng) {
    let x = rng.next_u32() % (amount * 2) as u32;
    let y = rng.next_u32() % (amount * 2) as u32;
    Display::Default().set_offset((amount as i32) - x as i32, (amount as i32) - y as i32);
}
