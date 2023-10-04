use ggez::*;
use crate::state::*;

impl<T: Board> State<T> {
    pub(crate) fn handle_mouse_button_down(
        &mut self,
        _context: &mut Context,
        button: event::MouseButton,
        x_pos: f32,
        y_pos: f32,
    ) -> GameResult {
        if button == event::MouseButton::Left {
            let row = Self::snap_to_board(y_pos);
            let col = Self::snap_to_board(x_pos);

            self.set_selected_tile(row, col);

            match self.selected_piece_index {
                Some(_) => self.make_move()?,
                None => self.make_selection()?,
            }
        }
        if button == event::MouseButton::Right {
            self.selected_piece_index = None;
        }

        Ok(())
    }
    
    pub(crate) fn handle_mouse_motion(
        &mut self,
        _context: &mut Context,
        x_pos: f32,
        y_pos: f32,
        _x_delta: f32,
        _y_delta: f32,
    ) -> GameResult {
        self.mouse_pos = Point2 {x: x_pos, y: y_pos };
        let row = Self::snap_to_board(y_pos);
        let col = Self::snap_to_board(x_pos);

        self.set_selected_tile(row, col);
        
        Ok(())
    }

    fn snap_to_board(pos: f32) -> usize {
        (pos / 64.0).floor().abs() as usize
    }

    fn set_selected_tile(&mut self, row: usize, col: usize) {
        self.selected_tile_index = col + row * 8;
        self.selected_tile_pos = (col, row);
    }

    fn make_selection(&mut self) -> GameResult {
        if self.chess_board.get_moves_at(self.selected_tile_index).is_empty() {
            return Ok(())
        }

        let selected_piece = self.chess_board.get_piece_at(self.selected_tile_index);

        match selected_piece.get_color() {
            WHITE => if self.chess_board.get_turn() == BLACK { return Ok(()) },
            BLACK => if self.chess_board.get_turn() == WHITE { return Ok(()) },
            _ => return Ok(())
        }

        match selected_piece.get_type() {
            PAWN | BISHOP | KNIGHT | ROOK | QUEEN | KING => {
                self.selected_piece_index = Some(self.selected_tile_index);
            },
            _ => (),
        }

        Ok(())
    }

    fn make_move(&mut self) -> GameResult {
        if let Some(piece) = self.selected_piece_index {
            self.selected_piece_index = None;
            'moves: for legal_move in self.chess_board.get_moves_at(piece) {
                if legal_move.from == piece && legal_move.to == self.selected_tile_index {
                    /* FIX PROMOTION */
                    self.chess_board.make_move(legal_move);
                    break 'moves;
                }
            }
            Ok(())
        } else {
            Err(GameError::CustomError("No piece selected".to_string()))
        }
    }
}