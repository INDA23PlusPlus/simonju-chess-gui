use olindba_chess::{Game, Piece};

pub(crate) trait Board {
    fn get_board(&self) -> [olindba_chess::Piece; 64];
    fn get_turn(&self) -> usize;
    fn get_moves(&self) -> Vec<olindba_chess::Move>;
    fn get_moves_from(&self) -> Vec<olindba_chess::Move>;
}

impl Board for Game {
    fn get_board(&mut self) -> [olindba_chess::Piece; 64] {
        self.board
    }

    fn get_turn(&mut self) -> usize {
        self.turn
    }
}

