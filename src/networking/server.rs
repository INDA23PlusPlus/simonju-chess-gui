use crate::networking::ServerGame;
use crate::board::{Board, Ply};
use chess_network_protocol::*;
use olindba_chess::GameState;
use serde_json::*;

impl Board for ServerGame {
    fn get_board(&self) -> [olindba_chess::Piece; 64] {
        self.board.board
    }

    fn get_turn(&self) -> usize {
        self.board.turn
    }

    fn get_moves(&self) -> Vec<Ply> {
        let mut mvs = vec![];
        for mv in self.board.get_all_legal_moves() {
            mvs.push(Ply {from: mv.get_from(), to: mv.get_to(), promotion: olindba_chess::QUEEN_PROMOTION});
        }
        mvs
    }

    fn get_moves_at(&self, i: usize) -> Vec<Ply> {
        let mut mvs = vec![];
        for mv in self.board.get_legal_moves(i) {
            mvs.push(Ply {from: mv.get_from(), to: mv.get_to(), promotion: olindba_chess::QUEEN_PROMOTION});
        }
        mvs
    }

    fn get_game_state(&self) -> Joever {
        match self.board.get_game_state() {
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
        self.board.get_piece_at(i)
    }

    fn make_move(&mut self, mv: Ply) {
        // Make move
        self.board.make_move_from_to(mv.from, mv.to, mv.promotion);

        // Send state to client
        todo!();
        let state = ServerToClient::State {
            board: [[Piece::BlackBishop; 8]; 8],
            moves: vec![Move {
                start_x: 0,
                start_y: 0,
                end_x: 0,
                end_y: 0,
                promotion: Piece::None,
            }],
            joever: Joever::Ongoing,
            move_made: Move {
                start_x: 0,
                start_y: 0,
                end_x: 0,
                end_y: 0,
                promotion: Piece::None,
            }
        };
    }

    fn update(&mut self) {
        // Get desired move from client
        // Call make_move()
        // If unsuccessful, resend state to client
    }
}