use std::io::{Write, Read};
use std::sync::mpsc::TryRecvError;

use crate::networking::ServerGame;
use crate::board::{Board, Ply, xy_to_i, Tile};
use chess_network_protocol::*;
use olindba_chess::GameState;
use serde::*;

impl Board for ServerGame {
    fn get_board(&self) -> [chess_network_protocol::Piece; 64] {
        let mut board = [chess_network_protocol::Piece::None; 64];

        for (index, tile) in self.board.board.into_iter().enumerate() {
            board[index] = match tile.get_color() {
                olindba_chess::WHITE => match tile.get_type() {
                    olindba_chess::PAWN => chess_network_protocol::Piece::WhitePawn,
                    olindba_chess::BISHOP => chess_network_protocol::Piece::WhiteBishop,
                    olindba_chess::KNIGHT => chess_network_protocol::Piece::WhiteKnight,
                    olindba_chess::ROOK => chess_network_protocol::Piece::WhiteRook,
                    olindba_chess::QUEEN => chess_network_protocol::Piece::WhiteQueen,
                    olindba_chess::KING => chess_network_protocol::Piece::WhiteKing,
                    _ => chess_network_protocol::Piece::None,
                },
                olindba_chess::BLACK => match tile.get_type() {
                    olindba_chess::PAWN => chess_network_protocol::Piece::BlackPawn,
                    olindba_chess::BISHOP => chess_network_protocol::Piece::BlackBishop,
                    olindba_chess::KNIGHT => chess_network_protocol::Piece::BlackKnight,
                    olindba_chess::ROOK => chess_network_protocol::Piece::BlackRook,
                    olindba_chess::QUEEN => chess_network_protocol::Piece::BlackQueen,
                    olindba_chess::KING => chess_network_protocol::Piece::BlackKing,
                    _ => chess_network_protocol::Piece::None,    
                },
                _ => chess_network_protocol::Piece::None,
            }
        }

        board
    }

    fn get_turn(&self) -> usize {
        self.board.turn
    }

    fn get_player(&self) -> usize {
        match self.player {
            super::Player::White => olindba_chess::WHITE,
            super::Player::Black => olindba_chess::BLACK,
        }
    }

    fn get_moves(&self) -> Vec<Ply> {
        let mut mvs = vec![];
        for mv in self.board.get_all_legal_moves() {
            mvs.push(Ply {
                from: mv.get_from(),
                to: mv.get_to(),
                promotion: olindba_chess::QUEEN_PROMOTION,
                color: self.board.turn,
            });
        }
        mvs
    }

    fn get_moves_at(&self, i: usize) -> Vec<Ply> {
        let mut mvs = vec![];
        for mv in self.board.get_legal_moves(i) {
            mvs.push(Ply {
                from: mv.get_from(),
                to: mv.get_to(),
                promotion: olindba_chess::QUEEN_PROMOTION,
                color: self.board.turn,
            });
        }
        mvs
    }

    fn get_game_state(&self) -> Joever {
        match self.board.get_game_state() {
            GameState::Checkmate => {
                match self.get_turn() {
                    olindba_chess::WHITE => chess_network_protocol::Joever::Black,
                    olindba_chess::BLACK => chess_network_protocol::Joever::White,
                    _ => chess_network_protocol::Joever::Indeterminate,
                }
            },
            GameState::DrawBy50MoveRule | GameState::InsufficientMaterial | GameState::Stalemate => chess_network_protocol::Joever::Draw,
            _ => chess_network_protocol::Joever::Ongoing,
        }
    }

    fn get_piece_at(&self, i: usize) -> Tile {
        self.board.get_piece_at(i)
    }

    fn make_move(&mut self, mv: Ply) {
        // Make move
        if !self.board.make_move_from_to(mv.from, mv.to, mv.promotion) {
            return;
        }

        // Create state
        let board = self.create_board();
        let moves = self.create_moves();
        let joever = self.get_game_state();
        let move_made = Move {
            start_x: mv.fromx(),
            start_y: mv.fromy(),
            end_x: mv.tox(),
            end_y: mv.toy(),
            promotion: mv.prom(),
        };

        let state = ServerToClient::State {
            board,
            moves,
            joever,
            move_made,
        };

        // Send state to client
        self.tcp_handler.write_sender.send(state);
    }

    fn update(&mut self) {
        let deserialized = match self.tcp_handler.read_receiver.try_recv() {
            Ok(x) => x,
            Err(x) => match x {
                TryRecvError::Empty => return,
                TryRecvError::Disconnected => return, // Todo
            },
        };

        // Get desired move from client
        let mut mv = Ply { from: 1337, to: 1337, color: 1337, promotion: 1337 };
        match deserialized {
            ClientToServer::Move(m) => {
                mv = Ply { 
                    from: xy_to_i(m.start_x, m.start_y),
                    to: xy_to_i(m.end_x, m.end_y),
                    promotion: match m.promotion {
                        Piece::WhiteBishop | Piece::BlackBishop => olindba_chess::BISHOP_PROMOTION,
                        Piece::WhiteKnight | Piece::BlackKnight => olindba_chess::KNIGHT_PROMOTION,
                        Piece::WhiteRook | Piece::BlackRook => olindba_chess::ROOK_PROMOTION,
                        Piece::WhiteQueen | Piece::BlackQueen => olindba_chess::QUEEN_PROMOTION,
                        _ => olindba_chess::QUEEN_PROMOTION,
                    },
                    color: self.board.turn,
                };
            },
            ClientToServer::Resign => (), // Optional
            ClientToServer::Draw => (), // Optional
        }

        // Make move
        self.board.make_move_from_to(mv.from, mv.to, mv.promotion);

        // Create state
        let board = self.create_board();
        let moves = self.create_moves();
        let joever = self.get_game_state();
        let move_made = Move {
            start_x: mv.fromx(),
            start_y: mv.fromy(),
            end_x: mv.tox(),
            end_y: mv.toy(),
            promotion: mv.prom(),
        };

        let state = ServerToClient::State {
            board,
            moves,
            joever,
            move_made,
        };

        // Send state to client
        self.tcp_handler.write_sender.send(state);
    }
}