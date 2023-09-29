use ggez::*;

use crate::state::*;

impl State {
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
        if self.chess_board.get_legal_moves(self.selected_tile_index).is_empty() {
            return Ok(())
        }

        let selected_piece = self.chess_board.board[self.selected_tile_index];

        match selected_piece.get_color() {
            WHITE => if self.chess_board.turn == BLACK { return Ok(()) },
            BLACK => if self.chess_board.turn == WHITE { return Ok(()) },
            _ => return Ok(())
        }

        match selected_piece.get_type() {
            PAWN | BISHOP | KNIGHT | ROOK | QUEEN | KING => {
                self.selected_piece_index = Some(self.selected_tile_index);
            },
            _ => (),
        }

        /* DEBUG */
        let mut piece = String::new();
        match selected_piece.get_color() {
            WHITE => piece.push_str("White "),
            BLACK => piece.push_str("Black "),
            _ => (),
        }
        match selected_piece.get_type() {
            PAWN => piece.push_str("Pawn "),
            BISHOP => piece.push_str("Bishop "),
            KNIGHT => piece.push_str("Knight "),
            ROOK => piece.push_str("Rook "),
            QUEEN => piece.push_str("Queen "),
            KING => piece.push_str("King "),
            _ => piece.clear(),
        }
        if !piece.is_empty() {
            piece.push_str("Selected!");
        }
        println!("{piece}");

        Ok(())
    }

    fn make_move(&mut self) -> GameResult {
        if let Some(piece) = self.selected_piece_index {
            let current_turn = self.chess_board.turn; // Fix for promotion error.
            for legal_move in self.chess_board.get_legal_moves(piece) {
                if legal_move.get_from() == piece && legal_move.get_to() == self.selected_tile_index {
                    self.chess_board.make_move(legal_move);
                    self.selected_piece_index = None;
                    match self.chess_board.get_game_state() {
                        GameState::InProgress | 
                        GameState::Check => (),
                        GameState::Checkmate |
                        GameState::DrawBy50MoveRule | 
                        GameState::InsufficientMaterial | 
                        GameState::Stalemate => self.is_end = false,
                    }
                    // Fix for promotion error.
                    if current_turn == self.chess_board.turn {
                        self.chess_board.turn ^= 1;
                    }
                } else {
                    self.selected_piece_index = None;
                }
            }
            Ok(())
        } else {
            Err(GameError::CustomError("No piece to selected".to_string()))
        }
    }
}