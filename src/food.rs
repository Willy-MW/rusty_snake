use crate::frame::{Drawable, Frame};
use crate::{Tickable, NUM_COLS, NUM_ROWS};

pub struct Food {
    pub position: (usize, usize),
    current_char: &'static str,
}

pub const FOOD_CHAR_1: &str = "●";
pub const FOOD_CHAR_2: &str = "◉";

impl Food {
    pub fn new() -> Food {
        Food {
            position: (NUM_COLS / 2, NUM_ROWS / 2 - 1),
            current_char: FOOD_CHAR_1,
        }
    }
}

impl Tickable for Food {
    fn tick(&mut self) {
        if self.current_char == FOOD_CHAR_1 {
            self.current_char = FOOD_CHAR_2;
        } else {
            self.current_char = FOOD_CHAR_1;
        }
    }
}

impl Drawable for Food {
    fn draw(&self, frame: &mut Frame) {
        frame[self.position.0][self.position.1] = self.current_char;
    }
}