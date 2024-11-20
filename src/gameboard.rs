// Size of the gameboard according to Tetris Guidelines
use crate::consts::*;

// https://en.wikipedia.org/wiki/Tetromino#One-sided_tetrominoes
#[derive(Copy, Clone)]
pub enum CellKind {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

pub struct Gameboard {
    pub cells: [[Option<CellKind>; CELL_WIDTH_COUNT]; CELL_HEIGHT_COUNT],
}

impl Gameboard {
    pub fn new() -> Gameboard {
        let mut ret = Gameboard {
            cells: [[None; CELL_WIDTH_COUNT]; CELL_HEIGHT_COUNT],
        };
        ret.cells[1][1] = Some(CellKind::I);
        ret.cells[1][2] = Some(CellKind::O);
        ret.cells[1][3] = Some(CellKind::T);
        ret.cells[1][4] = Some(CellKind::J);
        ret.cells[1][5] = Some(CellKind::L);
        ret.cells[1][6] = Some(CellKind::S);
        ret.cells[1][7] = Some(CellKind::Z);
        return ret;
    }
}
