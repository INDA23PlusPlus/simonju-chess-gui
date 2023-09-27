extern crate olindba_chess;

use ggez::*;

use olindba_chess::*;

mod event_handler;
mod game_loop;

struct GameState {
    chess_board: Game,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let game_state = GameState {
            chess_board: Game::starting_position(),
        };
        
        Ok(game_state)
    }
}