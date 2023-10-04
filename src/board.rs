use chess_network_protocol::{Move, ServerToClient};
use olindba_chess::{Game, GameState};

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

    fn create_board(&self) -> [[chess_network_protocol::Piece; 8]; 8] {
        let mut board = [[chess_network_protocol::Piece::None; 8]; 8];

        for (i, p) in self.get_board().iter().enumerate() {
            board[i_to_x(i)][i_to_y(i)] = match p.get_color() {
                olindba_chess::WHITE => match p.get_type() {
                    olindba_chess::PAWN => chess_network_protocol::Piece::WhitePawn,
                    olindba_chess::BISHOP => chess_network_protocol::Piece::WhiteBishop,
                    olindba_chess::KNIGHT => chess_network_protocol::Piece::WhiteKnight,
                    olindba_chess::ROOK => chess_network_protocol::Piece::WhiteRook,
                    olindba_chess::QUEEN => chess_network_protocol::Piece::WhiteQueen,
                    olindba_chess::KING => chess_network_protocol::Piece::WhiteKing,
                    _ => chess_network_protocol::Piece::None,
                }
                olindba_chess::BLACK => match p.get_type() {
                    olindba_chess::PAWN => chess_network_protocol::Piece::BlackPawn,
                    olindba_chess::BISHOP => chess_network_protocol::Piece::BlackBishop,
                    olindba_chess::KNIGHT => chess_network_protocol::Piece::BlackKnight,
                    olindba_chess::ROOK => chess_network_protocol::Piece::BlackRook,
                    olindba_chess::QUEEN => chess_network_protocol::Piece::BlackQueen,
                    olindba_chess::KING => chess_network_protocol::Piece::BlackKing,
                    _ => chess_network_protocol::Piece::None,
                }
                _ => chess_network_protocol::Piece::None,
            }
        }
        board
    }

    fn create_moves(&self) -> Vec<chess_network_protocol::Move> {
        let mut moves = vec![];
        for mv in self.get_moves() {
            moves.push(chess_network_protocol::Move {
                start_x: mv.fromx(),
                start_y: mv.fromy(),
                end_x: mv.tox(),
                end_y: mv.toy(),
                promotion: mv.prom(),
            })
        }
        moves
    }
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
            mvs.push(Ply {
                from: mv.get_from(),
                to: mv.get_to(),
                promotion: olindba_chess::QUEEN_PROMOTION,
                color: self.turn
            });
        }
        mvs
    }

    fn get_moves_at(&self, i: usize) -> Vec<Ply> {
        let mut mvs = vec![];
        for mv in self.get_legal_moves(i) {
            mvs.push(Ply {
                from: mv.get_from(),
                to: mv.get_to(),
                promotion: olindba_chess::QUEEN_PROMOTION,
                color: self.turn
            });
        }
        mvs
    }

    fn get_game_state(&self) -> chess_network_protocol::Joever {
        match self.get_game_state() {
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

    fn get_piece_at(&self, i: usize) -> olindba_chess::Piece {
        self.board[i]
    }

    fn make_move(&mut self, mv: Ply) {
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
    pub color: usize,
}

impl Ply {
    pub fn fromy(&self) -> usize { self.from / 8 }
    pub fn fromx(&self) -> usize { self.from % 8 }
    pub fn toy(&self) -> usize { self.to / 8 }
    pub fn tox(&self) -> usize { self.to % 8 }

    pub fn prom(&self) -> chess_network_protocol::Piece {
        match self.color {
            olindba_chess::WHITE => {
                match self.promotion {
                    olindba_chess::BISHOP_PROMOTION => chess_network_protocol::Piece::WhiteBishop,
                    olindba_chess::KNIGHT_PROMOTION => chess_network_protocol::Piece::WhiteKnight,
                    olindba_chess::ROOK_PROMOTION => chess_network_protocol::Piece::WhiteRook,
                    olindba_chess::QUEEN_PROMOTION => chess_network_protocol::Piece::WhiteQueen,
                    _ => chess_network_protocol::Piece::WhiteQueen,
                }
            }
            olindba_chess::BLACK => {
                match self.promotion {
                    olindba_chess::BISHOP_PROMOTION => chess_network_protocol::Piece::BlackBishop,
                    olindba_chess::KNIGHT_PROMOTION => chess_network_protocol::Piece::BlackKnight,
                    olindba_chess::ROOK_PROMOTION => chess_network_protocol::Piece::BlackRook,
                    olindba_chess::QUEEN_PROMOTION => chess_network_protocol::Piece::BlackQueen,
                    _ => chess_network_protocol::Piece::BlackQueen,
                }
            }
            _ => chess_network_protocol::Piece::None,
        }
    }
}

pub fn i_to_x(i: usize) -> usize { i % 8 }
pub fn i_to_y(i: usize) -> usize { i / 8 }
pub fn xy_to_i(x: usize, y: usize) -> usize { x + y * 8 }