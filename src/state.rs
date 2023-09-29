extern crate olindba_chess;

use std::collections::HashSet;

use ggez::*;

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
        };
        
        Ok(game_state)
    }
}

