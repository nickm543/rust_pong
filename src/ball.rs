extern crate rand;
use rand::{thread_rng, Rng};

use macroquad::prelude::*;
use crate::paddle::Paddle;

const BALL_SPEED: f32 = 5.0;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub xvel: f32,
    pub yvel: f32,
}

impl Ball {
    pub fn new() -> Ball {
        let mut rng = thread_rng();
        let rand_dir = rng.gen_range(1..4);

        Ball {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            w: 25.0,
            h: 25.0,
            xvel: BALL_SPEED,
            yvel: BALL_SPEED,
        }
    }

    pub fn draw(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, WHITE);
    }

    pub fn update(&mut self) {
        // Start moving on game start
        self.x += self.xvel;
        self.y += self.yvel;

        // Reset ball
        if is_key_down(KeyCode::R) {
            self.x = (screen_width() / 2.0) - (self.w / 2.0);
            self.y = (screen_height() / 2.0) - (self.h / 2.0);
        }
    }

    pub fn do_collision(&mut self, paddle1: &Paddle, paddle2: &Paddle) {
        // Top of screen
        if self.y < 0.0 {
            self.x += 1.0;
            self.y += 1.0;
            self.yvel = -self.yvel;
        }
        // Bottom of screen
        if self.y + self.h > screen_height() {
            self.x -= 1.0;
            self.y -= 1.0;
            self.yvel = -self.yvel;
        }

        // Left paddle
        if self.x <= paddle1.x + paddle1.w && self.y <= paddle1.y + paddle1.h && self.y >= paddle1.y {
            if self.y >= paddle1.y && (self.y + self.h) <= (paddle1.y + paddle1.h) {
                self.x += 1.0;
                self.y += 1.0;
                self.xvel = self.xvel.abs();
            }
        }

        // Right paddle
        if self.x + self.w >= paddle2.x && self.y <= paddle2.y + paddle2.h && self.y >= paddle2.y {
            if self.y >= paddle2.y && (self.y + self.h) <= (paddle2.y + paddle2.h) {
                self.x -= 1.0;
                self.y -= 1.0;
                self.xvel = -self.xvel;
            }
        }
    }
}
