use std::sync::mpsc::TryRecvError;

use chess_network_protocol::{Joever, ServerToClient, ClientToServer, Move};

use crate::networking::{ClientGame, Player};
use crate::board::{Board, Ply, i_to_x, i_to_y, Tile, xy_to_i};

impl Board for ClientGame {
    fn get_board(&self) -> [chess_network_protocol::Piece; 64] {
        let mut board = [chess_network_protocol::Piece::None; 64];

        for (index, tile) in self.board.into_iter().flatten().enumerate() {
            board[index] = tile;
        }

        board
    }

    fn get_turn(&self) -> usize {
        match self.turn {
            super::Player::White => olindba_chess::WHITE,
            super::Player::Black => olindba_chess::BLACK,
        }
        // self.board.turn
    }

    fn get_player(&self) -> usize {
        match self.player {
            super::Player::White => olindba_chess::WHITE,
            super::Player::Black => olindba_chess::BLACK,
        }
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

    fn get_piece_at(&self, i: usize) -> Tile {
        match self.board[i_to_x(i)][i_to_y(i)] {
            chess_network_protocol::Piece::BlackPawn => Tile::new(olindba_chess::PAWN, olindba_chess::BLACK),
            chess_network_protocol::Piece::BlackKnight => Tile::new(olindba_chess::KNIGHT, olindba_chess::BLACK),
            chess_network_protocol::Piece::BlackBishop => Tile::new(olindba_chess::BISHOP, olindba_chess::BLACK),
            chess_network_protocol::Piece::BlackRook => Tile::new(olindba_chess::ROOK, olindba_chess::BLACK),
            chess_network_protocol::Piece::BlackQueen => Tile::new(olindba_chess::QUEEN, olindba_chess::BLACK),
            chess_network_protocol::Piece::BlackKing => Tile::new(olindba_chess::KING, olindba_chess::BLACK),
            chess_network_protocol::Piece::WhitePawn => Tile::new(olindba_chess::PAWN, olindba_chess::WHITE),
            chess_network_protocol::Piece::WhiteKnight => Tile::new(olindba_chess::KNIGHT, olindba_chess::WHITE),
            chess_network_protocol::Piece::WhiteBishop => Tile::new(olindba_chess::BISHOP, olindba_chess::WHITE),
            chess_network_protocol::Piece::WhiteRook => Tile::new(olindba_chess::ROOK, olindba_chess::WHITE),
            chess_network_protocol::Piece::WhiteQueen => Tile::new(olindba_chess::QUEEN, olindba_chess::WHITE),
            chess_network_protocol::Piece::WhiteKing => Tile::new(olindba_chess::KING, olindba_chess::WHITE),
            chess_network_protocol::Piece::None => Tile::new(olindba_chess::EMPTY, olindba_chess::EMPTY),
        }
    }

    fn make_move(&mut self, mv: Ply) {
        // Set board state based on mv
        // Send board state to server
        let tile = self.get_piece_at(mv.from);
        type Cnpp = chess_network_protocol::Piece;
        let piece = match tile.get_color() {
            olindba_chess::WHITE => match tile.get_type() {
                olindba_chess::PAWN => Cnpp::WhitePawn,
                olindba_chess::BISHOP => Cnpp::WhiteBishop,
                olindba_chess::KNIGHT => Cnpp::WhiteKnight,
                olindba_chess::ROOK => Cnpp::WhiteRook,
                olindba_chess::QUEEN => Cnpp::WhiteQueen,
                olindba_chess::KING => Cnpp::WhiteKing,
                _ => Cnpp::None,
            }
            olindba_chess::BLACK => match tile.get_type() {
                olindba_chess::PAWN => Cnpp::BlackPawn,
                olindba_chess::BISHOP => Cnpp::BlackBishop,
                olindba_chess::KNIGHT => Cnpp::BlackKnight,
                olindba_chess::ROOK => Cnpp::BlackRook,
                olindba_chess::QUEEN => Cnpp::BlackQueen,
                olindba_chess::KING => Cnpp::BlackKing,
                _ => Cnpp::None,
            }
            _ => Cnpp::None,
        };

        self.board[mv.fromx()][mv.fromy()] = chess_network_protocol::Piece::None;
        self.board[mv.tox()][mv.toy()] = chess_network_protocol::Piece::None;

        self.tcp_handler.write_sender.send(ClientToServer::Move(chess_network_protocol::Move {
            start_x: mv.fromx(),
            start_y: mv.fromy(),
            end_x: mv.tox(),
            end_y: mv.toy(),
            promotion: match self.player {
                Player::White => Cnpp::WhiteQueen,
                Player::Black => Cnpp::BlackQueen,
            },
        }));
    }

    fn update(&mut self) {
        let deserialized = match self.tcp_handler.read_receiver.try_recv() {
            Ok(x) => x,
            Err(x) => match x {
                TryRecvError::Empty => return,
                TryRecvError::Disconnected => return, // Todo
            },
        };

        match deserialized {
            ServerToClient::State { board, moves, joever, move_made } => {
                self.board = board;
                type Cnpp = chess_network_protocol::Piece;
                self.moves.clear();
                for mv in moves {
                    self.moves.push(Ply {
                        from: xy_to_i(mv.start_x, mv.start_y),
                        to: xy_to_i(mv.start_x, mv.start_y),
                        promotion: match mv.promotion {
                            Cnpp::WhiteBishop | Cnpp::BlackBishop => olindba_chess::BISHOP_PROMOTION,
                            Cnpp::WhiteKnight | Cnpp::BlackKnight => olindba_chess::KNIGHT_PROMOTION,
                            Cnpp::WhiteRook | Cnpp::BlackRook => olindba_chess::ROOK_PROMOTION,
                            Cnpp::WhiteQueen | Cnpp::BlackQueen => olindba_chess::QUEEN_PROMOTION,
                            _ => olindba_chess::EMPTY,
                        },
                        color: olindba_chess::EMPTY, // Todo
                    })
                }
            },
            ServerToClient::Resigned { board, joever } => (), // Optional
            ServerToClient::Draw { board, moves } => (), // Optional
            ServerToClient::Error { board, moves, joever, message } => (),
        }
    }
}