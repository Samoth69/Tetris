use crate::consts::*;
use crate::gameboard::CellKind;
use crate::gameboard_controller::GameboardController;
use graphics::types::Color;
use graphics::{line, Context, Graphics};

pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    pub height: f64,
    pub width: f64,

    pub board_background_color: Color,
    pub board_edge_color: Color,
    pub board_edge_radius: f64,

    pub cell_edge_color: Color,
    pub cell_edge_radius: f64,
}

impl GameboardViewSettings {
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [BOARD_MARGIN as f64; 2],
            height: (CELL_HEIGHT_COUNT * CELL_SIZE) as f64,
            width: (CELL_WIDTH_COUNT * CELL_SIZE) as f64,

            board_background_color: [0.0, 0.0, 0.0, 1.0],
            board_edge_color: [1.0, 1.0, 1.0, 0.25],
            board_edge_radius: 1.0,

            cell_edge_color: [1.0, 1.0, 1.0, 0.25],
            cell_edge_radius: 1.0,
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView { settings }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};

        let ref settings = &self.settings;
        let max_x = settings.position[0] + settings.width;
        let max_y = settings.position[1] + settings.height;

        // background
        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.width,
            settings.height,
        ];
        Rectangle::new(settings.board_background_color).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );

        // Declare the format for cell and section lines.
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);

        for height in 0..CELL_HEIGHT_COUNT {
            let y =
                settings.position[1] + height as f64 / CELL_HEIGHT_COUNT as f64 * settings.height;
            let hline = [settings.position[0], y, max_x, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);

            for width in 0..CELL_WIDTH_COUNT {
                let x =
                    settings.position[0] + width as f64 / CELL_WIDTH_COUNT as f64 * settings.width;
                let yline = [x, settings.position[1], x, max_y];
                cell_edge.draw(yline, &c.draw_state, c.transform, g);

                // draw tetrominoes
                let cell = controller.gameboard.cells[height][width];
                if let Some(ucell) = cell {
                    let rect: Rectangle = match ucell {
                        CellKind::I => Rectangle::new([0.0, 1.0, 1.0, 0.75]),
                        CellKind::O => Rectangle::new([1.0, 1.0, 0.0, 0.75]),
                        CellKind::T => Rectangle::new([1.0, 0.0, 1.0, 0.75]),
                        CellKind::J => Rectangle::new([0.0, 0.0, 1.0, 0.75]),
                        CellKind::L => Rectangle::new([1.0, 0.5, 0.0, 0.75]),
                        CellKind::S => Rectangle::new([0.0, 1.0, 0.0, 0.75]),
                        CellKind::Z => Rectangle::new([1.0, 0.0, 0.0, 0.75]),
                    };
                    let cell_rect = [y + 1.0, x + 1.0, CELL_SIZE as f64, CELL_SIZE as f64];
                    rect.draw(cell_rect, &c.draw_state, c.transform, g);
                }
            }
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );
    }
}
