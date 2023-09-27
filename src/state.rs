extern crate olindba_chess;

use ggez::*;

use olindba_chess::*;

use assets::*;

mod event_handler;
mod assets;

pub(crate) struct State {
    chess_board: Game,
    assets: Assets,
    delta_time: std::time::Duration,
}

impl State {
    pub fn new(context: &mut Context) -> GameResult<State> {
        let game_state = State {
            chess_board: Game::starting_position(),
            assets: Assets::new(context)?,
            delta_time: std::time::Duration::new(0, 0),
        };
        
        Ok(game_state)
    }
}

