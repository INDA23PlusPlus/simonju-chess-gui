use ggez::{*, input::keyboard::KeyInput};
use crate::state::*;

mod draw;
mod update;
mod mouse;

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.handle_update(context)
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.handle_draw(context)
    }

    fn mouse_button_down_event(
            &mut self,
            context: &mut Context,
            button: event::MouseButton,
            x_pos: f32,
            y_pos: f32,
        ) -> GameResult {
        self.handle_mouse_button_down(context, button, x_pos, y_pos)
    }

    fn mouse_motion_event(
            &mut self,
            context: &mut Context,
            x_pos: f32,
            y_pos: f32,
            x_delta: f32,
            y_delta: f32,
        ) -> GameResult {
        self.handle_mouse_motion(context, x_pos, y_pos, x_delta, y_delta)
    }

    fn key_down_event(
            &mut self,
            context: &mut Context,
            input: input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), GameError> {
        match input.keycode {
            Some(input::keyboard::KeyCode::R) => self.reset(),
            _ => (),
        }
        Ok(())
    }
}