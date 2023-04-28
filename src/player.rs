use std::time::Duration;

use crate::{
    frame::{Drawable, Frame},
    shot::Shot,
    NUM_COLS, NUM_ROWS,
};

pub struct Player {
    x: usize,
    y: usize,

    shots: Vec<Shot>,
}

impl Player {
    // Initialize the player position
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 3 {
            self.shots.push(Shot::new(self.x, self.y));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A"; // Yep! the A is the spaceship
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
