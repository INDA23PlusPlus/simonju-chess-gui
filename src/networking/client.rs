use chess_network_protocol::Joever;

use crate::networking::ClientGame;
use crate::board::{Board, Ply};

impl Board for ClientGame {
    fn get_board(&self) -> [olindba_chess::Piece; 64] {
        self.board.board
    }

    fn get_turn(&self) -> usize {
        self.board.turn
    }

    fn get_moves(&self) -> Vec<Ply> {
        self.moves.clone()
    }

    fn get_moves_at(&self, i: usize) -> Vec<Ply> {
        let mut mvs = vec![];
        for mv in &self.moves {
            if mv.from == i {
                mvs.push(*mv)
            }
        }
        mvs
    }

    fn get_game_state(&self) -> Joever {
        self.game_state
    }

    fn get_piece_at(&self, i: usize) -> olindba_chess::Piece {
        self.board.get_piece_at(i)
    }

    fn make_move(&mut self, mv: Ply) {
        // Set board state based on mv
        // Send board state to server
        todo!()
    }

    fn update(&mut self) {
        // Accept state from server
        // Update state if modified
        todo!()
    }
}