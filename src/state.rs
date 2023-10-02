use std::net::{TcpListener, TcpStream};
use ggez::{*, mint::Point2};
use olindba_chess::*;
use assets::*;

mod event_handler;
mod assets;

pub(crate) struct State {
    chess_board: Game,
    assets: Assets,
    delta_time: std::time::Duration,
    selected_tile_index: usize,
    selected_tile_pos: (usize, usize),
    selected_piece_index: Option<usize>,
    is_end: bool,
    mouse_pos: Point2<f32>,
}

impl State {
    pub fn new(context: &mut Context) -> GameResult<State> {
        let game_state = State {
            chess_board: Game::starting_position(),
            assets: Assets::new(context)?,
            delta_time: std::time::Duration::new(0, 0),
            selected_tile_index: 0,
            selected_tile_pos: (0, 0),
            selected_piece_index: None,
            is_end: true,
            mouse_pos: Point2 { x: 0.0, y: 0.0 },
        };
        
        Ok(game_state)
    }

    pub fn reset(&mut self) {
            self.chess_board = Game::starting_position();
            self.selected_piece_index = None;
            self.is_end = true;
    }
}

