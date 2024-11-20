use crate::gameboard::Gameboard;
use piston::{Button, GenericEvent, Key};

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
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space {
                println!("Press Space key");
            }
        }
        if let Some(_args) = e.update_args() {
            // println!("Update args: {:?}", _args);
        }
    }
}
