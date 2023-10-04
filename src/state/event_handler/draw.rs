use ggez::{*, graphics::{Color, Rect, Sampler}};
use crate::state::*;

const TILE_DIMENSIONS: f32 = 64.0;

// source: http://omgchess.blogspot.com/2015/09/chess-board-color-schemes.html
const DARK: (u8, u8, u8) = (184, 139, 74);
const LIGHT: (u8, u8, u8) = (227, 193, 111);

const SELECT: (u8, u8, u8, u8) = (139, 171, 112, 192);
const MOVES: (u8, u8, u8, u8) = (173, 216, 230, 128);

impl<T: Board> State<T> {
    pub(crate) fn handle_draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, graphics::Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());

        // Board Layer
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

        // Pieces Layer
        for tile in 0..64 {
            if let Ok(image) = self.assets.get_image_from_piece(self.chess_board.get_piece_at(tile)) {
                if self.selected_piece_index != Some(tile) {
                    canvas.draw(image, graphics::DrawParam::default()
                    .dest_rect(Self::index_to_rect(tile)));
                }
            }
        }

        // Highlight Layer
        let select = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                TILE_DIMENSIONS * self.selected_tile_pos.0 as f32,
                TILE_DIMENSIONS * self.selected_tile_pos.1 as f32,
                TILE_DIMENSIONS, 
                TILE_DIMENSIONS
            ),
            Color::from_rgba(SELECT.0, SELECT.1, SELECT.2, SELECT.3),
        )?;
        canvas.draw(&select, graphics::DrawParam::default());

        let moves = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, TILE_DIMENSIONS, TILE_DIMENSIONS),
            Color::from_rgba(MOVES.0, MOVES.1, MOVES.2, MOVES.3),
        )?;
        if let Some(piece) = self.selected_piece_index {
            for legal_move in self.chess_board.get_moves_at(piece) {
                let dst = graphics::Rect::new(
                    TILE_DIMENSIONS * Self::as_col(legal_move.to),
                    TILE_DIMENSIONS * Self::as_row(legal_move.to),
                    1.0,
                    1.0,
                );
                canvas.draw(&moves, graphics::DrawParam::default().dest_rect(dst));
            }
        } else {
            for legal_move in self.chess_board.get_moves_at(self.selected_tile_index) {
                let dst = graphics::Rect::new(
                    TILE_DIMENSIONS * Self::as_col(legal_move.to),
                    TILE_DIMENSIONS * Self::as_row(legal_move.to),
                    1.0,
                    1.0,
                );
                canvas.draw(&moves, graphics::DrawParam::default().dest_rect(dst));
            }
        }

        // Selected Piece Layer
        if let Some(piece) = self.selected_piece_index {
            if let Ok(image) = self.assets.get_image_from_piece(self.chess_board.get_piece_at(piece)) {
                canvas.draw(image, graphics::DrawParam::default()
                .dest_rect(Rect::new(self.mouse_pos.x - 32.0, self.mouse_pos.y - 32.0, 4.0, 4.0)));
            }
        }

        // End Screen Layer
        let screen = graphics::Rect::new(
            0.0,
            0.0,
            16.0,
            16.0,
        );
        match self.chess_board.get_game_state() {
            chess_network_protocol::Joever::White => canvas.draw(&self.assets.white_win_screen, graphics::DrawParam::default().dest_rect(screen)),
            chess_network_protocol::Joever::Black => canvas.draw(&self.assets.black_win_screen, graphics::DrawParam::default().dest_rect(screen)),     
            chess_network_protocol::Joever::Draw => canvas.draw(&self.assets.draw_screen, graphics::DrawParam::default().dest_rect(screen)),
            _ => (),
        }

        canvas.finish(context)?;

        Ok(())
    }

    fn index_to_rect(index: usize) -> Rect {
        let row = (index / 8) as f32;
        let col = (index % 8) as f32;

        graphics::Rect::new(col * TILE_DIMENSIONS, row * TILE_DIMENSIONS, 4.0, 4.0)
    }

    pub(crate) fn as_row(i: usize) -> f32 {
        (i / 8) as f32
    }

    pub(crate) fn as_col(i: usize) -> f32 {
        (i % 8) as f32
    }
}