use rand::random;
use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);

    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }

    cols
}

pub fn get_random_empty_position(frame: &Frame) -> Option<(usize, usize)> {
    let x = (random::<f32>() * (NUM_COLS - 1) as f32).round() as usize;
    let y = (random::<f32>() * (NUM_ROWS - 1) as f32).round() as usize;

    if frame[x][y] == " " {
        return Some((x, y));
    }

    let mut empty_positions = Vec::with_capacity(NUM_ROWS * NUM_COLS);

    for (x, cols) in frame.iter().enumerate() {
        for (y, str) in cols.iter().enumerate() {
            if *str == " " { empty_positions.push((x, y)); }
        }
    }

    if empty_positions.len() == 0 {
        return None;
    }

    let rand = (random::<f32>() * (empty_positions.len() - 1) as f32) as usize;

    Some(empty_positions[rand])
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}