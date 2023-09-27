use ggez::{*, graphics::{Color, Image}};

use crate::state::*;

const TILE_DIMENSIONS: f32 = 64.0;

// source: http://omgchess.blogspot.com/2015/09/chess-board-color-schemes.html
const DARK: (u8, u8, u8) = (184,139,74);
const LIGHT: (u8, u8, u8) = (227,193,111);

impl State {
    pub(crate) fn handle_draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, graphics::Color::BLACK);
        
        for row in 0..8 {
            for col in 0..8 {
                let rect = graphics::Mesh::new_rectangle(
                    context,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(TILE_DIMENSIONS * col as f32, TILE_DIMENSIONS * row as f32, TILE_DIMENSIONS, TILE_DIMENSIONS),
                    match (row + col) % 2 == 0 {
                        true => Color::from_rgb(LIGHT.0, LIGHT.1, LIGHT.2),
                        false => Color::from_rgb(DARK.0, DARK.1, DARK.2),
                    }
                )?;

                canvas.draw(&rect, graphics::DrawParam::default());
            }
        }

        for tile in 0..64 {
            if let Ok(image) = self.assets.get_image_from_piece(self.chess_board.board[tile]) {
                canvas.draw(image, graphics::DrawParam::default());
            }
        }

        canvas.finish(context)?;
        Ok(())
    }
}