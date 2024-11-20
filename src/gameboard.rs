// Size of the gameboard according to Tetris Guidelines
use crate::consts::*;

pub struct Gameboard {
    pub cells: [[u8; CELL_WIDTH_COUNT]; CELL_HEIGHT_COUNT],
}

impl Gameboard {
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; CELL_WIDTH_COUNT]; CELL_HEIGHT_COUNT],
        }
    }
}
