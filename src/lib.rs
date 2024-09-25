pub mod frame;
pub mod snake;
pub mod render;
pub mod food;

pub const TICK_INTERVAL: u64 = 300;
pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 30;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Tickable{
    fn tick(&mut self);
}
