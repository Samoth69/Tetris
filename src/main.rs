mod gameboard;
mod gameboard_controller;
mod gameboard_view;
mod consts;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventSettings, Events, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, WindowSettings};
use crate::gameboard::Gameboard;
use crate::gameboard_controller::GameboardController;
use crate::gameboard_view::{GameboardView, GameboardViewSettings};
use crate::consts::*;

fn main() {
    let mut events = Events::new(EventSettings::new());
    let opengl = OpenGL::V3_2;

    let width = CELL_WIDTH_COUNT * CELL_SIZE + BOARD_MARGIN * 2;
    let height = CELL_HEIGHT_COUNT * CELL_SIZE + BOARD_MARGIN * 2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Samousse Tetris", [width as u32, height as u32])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([0.1; 4], g);
                gameboard_view.draw(&gameboard_controller, &c, g);
            })
        }
    }
}
