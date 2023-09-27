use ggez::*;

use crate::state::*;

impl State {
    pub(crate) fn handle_mouse_button_down(
        &mut self,
        context: &mut Context,
        button: event::MouseButton,
        x_pos: f32,
        y_pos: f32,
    ) -> GameResult {
    Ok(())
    }
    
    pub(crate) fn handle_mouse_motion(
        &mut self,
        context: &mut Context,
        x_pos: f32,
        y_pos: f32,
        x_delta: f32,
        y_delta: f32,
    ) -> GameResult {
    Ok(())
    }
}