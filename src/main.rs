mod paddle;
mod ball;
mod player;

use macroquad::prelude::*;
use crate::paddle::{Paddle, Side};
use crate::ball::Ball;
use crate::player::Player;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut paddle1: Paddle = Paddle::new(Side::Left);
    let mut paddle2: Paddle = Paddle::new(Side::Right);
    let mut ball: Ball = Ball::new();
    let mut player1: Player = Player::new();
    let mut player2: Player = Player::new();

    let text_params = TextParams {
        font_size: 26,
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        // Handle game resetting or ending
        if is_key_down(KeyCode::R) {
            ball.x = screen_width() / 2.0;
            ball.y = screen_height() / 2.0;

            // Reset score
            player1.score = 0;
            player2.score = 0;
        }
        if is_key_down(KeyCode::Q) {
            return
        }

        // Draws
        paddle1.draw();
        paddle2.draw();
        ball.draw();

        // Center divider line
        draw_line(
            screen_width() / 2.0,
            0.0,
            screen_width() / 2.0,
            screen_height(),
            3.0,
            WHITE
        );

        // Updates
        ball.update();
        ball.do_collision(&paddle1, &paddle2);

        paddle1.update();
        paddle2.update();

        // Score points and reset ball after scoring
        if ball.x <= 0.0 {
            player2.score += 1;
            ball.x = (screen_width() / 2.0) - (ball.w / 2.0);
            ball.y = (screen_height() / 2.0) - (ball.h / 2.0);
        }
        if ball.x >= screen_width() {
            player1.score += 1;
            ball.x = (screen_width() / 2.0) - (ball.w / 2.0);
            ball.y = (screen_height() / 2.0) - (ball.h / 2.0);
        }

        // Draw score on the screen
        draw_text_ex(&player1.score.to_string(), (screen_width() / 2.0) - 40.0, 30.0, text_params);
        draw_text_ex(&player2.score.to_string(), (screen_width() / 2.0) + 25.0, 30.0, text_params);

        next_frame().await
    }
}
