use olindba_chess::{Game, Piece, GameState, WHITE};
use crate::networking::*;

pub(crate) trait Board {
    fn get_board(&self) -> [olindba_chess::Piece; 64];
    fn get_turn(&self) -> usize;
    fn get_moves(&self) -> Vec<Ply>;
    fn get_moves_at(&self, i: usize) -> Vec<Ply>;
    fn get_game_state(&self) -> chess_network_protocol::Joever;
    fn get_piece_at(&self, i: usize) -> olindba_chess::Piece;

    // Use to make moves and send data
    fn make_move(&mut self, mv: Ply);

    // Use to accept data
    fn update(&mut self);
}

type LocalGame = Game;

impl Board for LocalGame {
    fn get_board(&self) -> [olindba_chess::Piece; 64] {
        self.board
    }

    fn get_turn(&self) -> usize {
        self.turn
    }

    fn get_moves(&self) -> Vec<Ply> {
        let mut mvs = vec![];
        for mv in self.get_all_legal_moves() {
            mvs.push(Ply {from: mv.get_from(), to: mv.get_to(), promotion: olindba_chess::QUEEN_PROMOTION});
        }
        mvs
    }

    fn get_moves_at(&self, i: usize) -> Vec<Ply> {
        let mut mvs = vec![];
        for mv in self.get_legal_moves(i) {
            mvs.push(Ply {from: mv.get_from(), to: mv.get_to(), promotion: olindba_chess::QUEEN_PROMOTION});
        }
        mvs
    }

    fn get_game_state(&self) -> chess_network_protocol::Joever {
        match self.get_game_state() {
            GameState::Checkmate => {
                match self.get_turn() {
                    WHITE => chess_network_protocol::Joever::Black,
                    BLACK => chess_network_protocol::Joever::White,
                    _ => chess_network_protocol::Joever::Indeterminate,
                }
            },
            GameState::DrawBy50MoveRule | GameState::InsufficientMaterial | GameState::Stalemate => chess_network_protocol::Joever::Draw,
            _ => chess_network_protocol::Joever::Ongoing,
        }
    }

    fn get_piece_at(&self, i: usize) -> olindba_chess::Piece {
        self.board[i]
    }

    fn make_move(&mut self, mv: Ply) {
        println!("from: {}, to: {}", mv.from, mv.to);
        self.make_move_from_to(mv.from, mv.to, mv.promotion);
    }

    fn update(&mut self) {
        // Do Nothing
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ply {
    pub from: usize,
    pub to: usize,
    pub promotion: usize,
}