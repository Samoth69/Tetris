use crate::gameboard::Gameboard;
use piston::GenericEvent;

// A controller is an object that handles events and manipulates some data. The data manipulated by a controller is called a "model".
pub struct GameboardController {
    pub gameboard: Gameboard,
}

impl GameboardController {
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}
