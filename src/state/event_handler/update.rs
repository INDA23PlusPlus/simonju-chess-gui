use ggez::*;
use crate::state::*;

impl State {
    pub(crate) fn handle_update(&mut self, context: &mut Context) -> GameResult {
        self.delta_time = context.time.delta();
        // Refresh connection to server every few seconds.
        Ok(())
    }
}