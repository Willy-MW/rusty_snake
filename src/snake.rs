use std::collections::VecDeque;
use crate::{Direction, Tickable, NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

pub const HEAD_CHAR_UP: &str = "▲";
pub const HEAD_CHAR_DOWN: &str = "▼";
pub const HEAD_CHAR_LEFT: &str = "◀";
pub const HEAD_CHAR_RIGHT: &str = "▶";
pub const BODY_CHAR: &str = "■";

pub struct Snake {
    body: VecDeque<(usize, usize)>,
    pub length: usize,
    current_direction: Direction,
    next_direction: Direction,
    has_collided: bool,
}

impl Snake {
    pub fn new() -> Self {
        let mut body = VecDeque::with_capacity(NUM_COLS * NUM_ROWS);
        body.push_back((NUM_COLS / 2, NUM_ROWS / 2));

        Self {
            body,
            length: 4,
            current_direction: Direction::Up,
            next_direction: Direction::Up,
            has_collided: false,
        }
    }

    pub fn has_collided(&self) -> &bool {
        &self.has_collided
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction == Direction::Up && self.current_direction == Direction::Down { return; }
        if direction == Direction::Down && self.current_direction == Direction::Up { return; }
        if direction == Direction::Left && self.current_direction == Direction::Right { return; }
        if direction == Direction::Right && self.current_direction == Direction::Left { return; }

        self.next_direction = direction;
    }

    pub fn get_head_position(&self) -> (usize, usize) {
        (self.body.back().unwrap().0, self.body.back().unwrap().1)
    }

    pub fn set_head_position(&mut self, position: (isize, isize)) {
        if position.0 < 0 || position.0 >= NUM_COLS as isize || position.1 < 0 || position.1 >= NUM_ROWS as isize {
            self.has_collided = true;
            return;
        }

        let new_position = (position.0 as usize, position.1 as usize);

        for i in 0..self.body.len()-1 {
            if self.body[i] == new_position {
                self.has_collided = true;
                return;
            }
        }

        self.body.push_back(new_position);

        if self.body.len() > self.length {
            self.body.pop_front();
        }
    }
    pub fn move_head(&mut self, direction: Direction) {
        let (mut head_x, mut head_y) = (self.get_head_position().0 as isize, self.get_head_position().1 as isize);

        (head_x, head_y) = match direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        self.set_head_position((head_x, head_y))
    }
}

impl Tickable for Snake {
    fn tick(&mut self) {
        self.current_direction = self.next_direction;
        self.move_head(self.current_direction);
    }
}

impl Drawable for Snake {
    fn draw(&self, frame: &mut Frame) {
        for (index, body_part) in self.body.iter().enumerate() {
            let body_part_char = if index + 1 == self.body.len() {
                match self.current_direction {
                    Direction::Up => HEAD_CHAR_UP,
                    Direction::Down => HEAD_CHAR_DOWN,
                    Direction::Left => HEAD_CHAR_LEFT,
                    Direction::Right => HEAD_CHAR_RIGHT,
                }
            } else {
                BODY_CHAR
            };

            frame[body_part.0][body_part.1] = body_part_char;
        }
    }
}