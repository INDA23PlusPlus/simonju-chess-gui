use ggez::*;
use crate::state::*;

impl<T: Board> State<T> {
    pub(crate) fn handle_update(&mut self, context: &mut Context) -> GameResult {
        
        self.delta_time = context.time.delta();
        self.chess_board.update();
        // Refresh connection to server every few seconds.
        Ok(())
    }
}