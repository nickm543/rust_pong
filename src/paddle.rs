use macroquad::prelude::*;

const PADDLE_SPEED: f32 = 6.0;

pub enum Side {
    Left,
    Right
}

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    side: Side
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        let mut _x: f32 = 20.0;

        match side {
            Side::Left => _x = 20.0,
            Side::Right => _x = screen_width() - 30.0
        }

        Paddle {
            x: _x,
            y: screen_height() / 2.0,
            w: 15.0,
            h: 65.0,
            side: side
        }
    }

    pub fn draw(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, WHITE);
    }

    pub fn update(&mut self) {
        match self.side {
            Side::Left => {
                if is_key_down(KeyCode::W) {
                    if self.y >= 0.0 {
                        self.y -= PADDLE_SPEED;
                    }
                }
                if is_key_down(KeyCode::S) {
                    if self.y + self.h <= screen_height() {
                        self.y += PADDLE_SPEED;
                    }
                }
            },
            Side::Right => {
                if is_key_down(KeyCode::Up) {
                    if self.y >= 0.0 {
                        self.y -= PADDLE_SPEED;
                    }
                }
                if is_key_down(KeyCode::Down) {
                    if self.y + self.h <= screen_height() {
                        self.y += PADDLE_SPEED;
                    }
                }
            }
        }
    }
}
